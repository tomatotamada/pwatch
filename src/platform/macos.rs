use super::PortScanner;
use crate::port::PortInfo;
use std::process::Command;

pub struct MacosScanner;

impl PortScanner for MacosScanner {
    fn scan() -> Vec<PortInfo> {
        let output = match Command::new("lsof")
            .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n"])
            .output()
        {
            Ok(o) => o,
            Err(_) => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut result: Vec<PortInfo> = stdout
            .lines()
            .skip(1) // ヘッダー行をスキップ
            .filter_map(parse_lsof_line)
            .collect();

        result.sort_by_key(|p| p.port);
        result
    }
}

/// lsof出力の1行をパース
/// 形式: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
fn parse_lsof_line(line: &str) -> Option<PortInfo> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 10 {
        return None;
    }

    let process_name = fields[0].to_string();
    let pid: u32 = fields[1].parse().ok()?;

    // NAME列 (例: "*:8080", "127.0.0.1:3000")
    let name = fields[8];
    let port_str = name.rsplit(':').next()?;
    let port: u16 = port_str.parse().ok()?;

    // コマンドラインを取得 (psコマンド経由)
    let command = get_command_by_pid(pid);

    Some(PortInfo {
        port,
        protocol: "tcp".to_string(),
        pid,
        process_name,
        command,
    })
}

fn get_command_by_pid(pid: u32) -> String {
    Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
