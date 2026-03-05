use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use serde::Serialize;

use crate::platform::{PlatformScanner, PortScanner};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: u32,
    pub process_name: String,
    pub command: String,
}

/// OS別のスキャナを使ってLISTEN状態のポート情報を取得
pub fn scan() -> Vec<PortInfo> {
    PlatformScanner::scan()
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
