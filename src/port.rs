use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: u32,
    pub process_name: String,
    pub command: String,
}

/// /proc/net/tcp および /proc/net/tcp6 からLISTEN状態のポート情報を取得
pub fn scan() -> Vec<PortInfo> {
    let mut entries = Vec::new();

    for (path, proto) in [("/proc/net/tcp", "tcp"), ("/proc/net/tcp6", "tcp6")] {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines().skip(1) {
                if let Some(entry) = parse_proc_net_line(line, proto) {
                    entries.push(entry);
                }
            }
        }
    }

    // inode→PIDを解決
    let inode_pid_map = build_inode_pid_map();
    let mut result: Vec<PortInfo> = entries
        .into_iter()
        .filter_map(|(port, proto, inode)| {
            let pid = *inode_pid_map.get(&inode)?;
            let process_name = read_proc_comm(pid);
            let command = read_proc_cmdline(pid);
            Some(PortInfo {
                port,
                protocol: proto,
                pid,
                process_name,
                command,
            })
        })
        .collect();

    result.sort_by_key(|p| p.port);
    result
}

/// 指定ポートのPortInfoを取得（未使用ならNone）
pub fn check(port: u16) -> Option<PortInfo> {
    scan().into_iter().find(|p| p.port == port)
}

/// 指定PIDにシグナルを送信
pub fn kill_process(pid: u32, force: bool) -> Result<(), String> {
    let sig = if force {
        Signal::SIGKILL
    } else {
        Signal::SIGTERM
    };
    signal::kill(Pid::from_raw(pid as i32), sig).map_err(|e| match e {
        nix::errno::Errno::EPERM => {
            format!(
                "Permission denied (PID {}). Try: sudo pwatch kill <PORT>",
                pid
            )
        }
        nix::errno::Errno::ESRCH => format!("Process {} not found (already exited?)", pid),
        _ => format!("Failed to kill PID {}: {}", pid, e),
    })
}

/// /proc/net/tcp の1行をパースし、LISTENなら (port, protocol, inode) を返す
fn parse_proc_net_line(line: &str, proto: &str) -> Option<(u16, String, u64)> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 10 {
        return None;
    }

    // fields[3] = state, 0A = LISTEN
    let state = fields[3];
    if state != "0A" {
        return None;
    }

    // fields[1] = local_address (e.g. "0100007F:1F90")
    let local_addr = fields[1];
    let port_hex = local_addr.rsplit(':').next()?;
    let port = u16::from_str_radix(port_hex, 16).ok()?;

    // fields[9] = inode
    let inode: u64 = fields[9].parse().ok()?;

    Some((port, proto.to_string(), inode))
}

/// 全プロセスの fd を走査して inode → PID のマップを構築
fn build_inode_pid_map() -> std::collections::HashMap<u64, u32> {
    let mut map = std::collections::HashMap::new();

    let proc_dir = match fs::read_dir("/proc") {
        Ok(d) => d,
        Err(_) => return map,
    };

    for entry in proc_dir.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let pid: u32 = match name_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let fd_dir = format!("/proc/{}/fd", pid);
        let fds = match fs::read_dir(&fd_dir) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for fd_entry in fds.flatten() {
            if let Ok(link) = fs::read_link(fd_entry.path()) {
                let link_str = link.to_string_lossy();
                if let Some(inode_str) = link_str
                    .strip_prefix("socket:[")
                    .and_then(|s| s.strip_suffix(']'))
                {
                    if let Ok(inode) = inode_str.parse::<u64>() {
                        map.insert(inode, pid);
                    }
                }
            }
        }
    }

    map
}

/// /proc/[pid]/comm からプロセス名を取得
fn read_proc_comm(pid: u32) -> String {
    let path = format!("/proc/{}/comm", pid);
    fs::read_to_string(&path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// /proc/[pid]/cmdline からコマンドラインを取得
fn read_proc_cmdline(pid: u32) -> String {
    let path = format!("/proc/{}/cmdline", pid);
    fs::read_to_string(&path)
        .unwrap_or_default()
        .replace('\0', " ")
        .trim()
        .to_string()
}
