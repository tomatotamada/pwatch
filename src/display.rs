use crate::port::PortInfo;

pub fn print_port_list(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!("リスニング中のポートが見つかりません");
        return;
    }

    println!(
        "{:<8} {:<6} {:<8} {:<20} {}",
        "PORT", "PROTO", "PID", "PROCESS", "COMMAND"
    );
    println!("{}", "-".repeat(70));
    for p in ports {
        println!(
            "{:<8} {:<6} {:<8} {:<20} {}",
            p.port, p.protocol, p.pid, p.process_name, p.command
        );
    }
}

pub fn print_check_result(port: u16, info: Option<&PortInfo>) {
    match info {
        Some(p) => {
            println!(
                "ポート {} は {} (PID: {}) が使用中",
                port, p.process_name, p.pid
            );
            println!("  コマンド: {}", p.command);
        }
        None => {
            println!("ポート {} は未使用", port);
        }
    }
}

pub fn print_kill_result(port: u16, info: &PortInfo, result: Result<(), String>, force: bool) {
    let sig = if force { "SIGKILL" } else { "SIGTERM" };
    match result {
        Ok(()) => println!(
            "ポート {} のプロセス {} (PID: {}) に {} を送信しました",
            port, info.process_name, info.pid, sig,
        ),
        Err(e) => eprintln!("エラー: {}", e),
    }
}
