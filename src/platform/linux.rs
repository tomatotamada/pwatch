use super::PortScanner;
use crate::port::PortInfo;
use std::collections::HashMap;
use std::fs;

pub struct LinuxScanner;

impl PortScanner for LinuxScanner {
    fn scan() -> Vec<PortInfo> {
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
}

fn parse_proc_net_line(line: &str, proto: &str) -> Option<(u16, String, u64)> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 10 {
        return None;
    }

    let state = fields[3];
    if state != "0A" {
        return None;
    }

    let local_addr = fields[1];
    let port_hex = local_addr.rsplit(':').next()?;
    let port = u16::from_str_radix(port_hex, 16).ok()?;

    let inode: u64 = fields[9].parse().ok()?;

    Some((port, proto.to_string(), inode))
}

fn build_inode_pid_map() -> HashMap<u64, u32> {
    let mut map = HashMap::new();

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

fn read_proc_comm(pid: u32) -> String {
    let comm = fs::read_to_string(format!("/proc/{}/comm", pid))
        .unwrap_or_default()
        .trim()
        .to_string();

    if comm.is_empty()
        || !comm
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b == b'-' || b == b'_')
    {
        if let Ok(exe) = fs::read_link(format!("/proc/{}/exe", pid)) {
            if let Some(name) = exe.file_name() {
                return name.to_string_lossy().to_string();
            }
        }
    }

    comm
}

fn read_proc_cmdline(pid: u32) -> String {
    let path = format!("/proc/{}/cmdline", pid);
    fs::read_to_string(&path)
        .unwrap_or_default()
        .replace('\0', " ")
        .trim()
        .to_string()
}
