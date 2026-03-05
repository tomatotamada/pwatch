use crate::port::PortInfo;
use colored::Colorize;
use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};

pub fn print_port_list(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!("{}", "リスニング中のポートが見つかりません".yellow());
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            format!("{}", "PORT".cyan().bold()),
            format!("{}", "PROTO".green()),
            format!("{}", "PID".yellow()),
            format!("{}", "PROCESS".magenta()),
            format!("{}", "COMMAND".white().bold()),
        ]);

    for p in ports {
        table.add_row(vec![
            format!("{}", p.port.to_string().cyan().bold()),
            format!("{}", p.protocol.green()),
            format!("{}", p.pid.to_string().yellow()),
            format!("{}", p.process_name.magenta()),
            p.command.clone(),
        ]);
    }

    println!("{table}");
}

pub fn print_check_result(port: u16, info: Option<&PortInfo>) {
    match info {
        Some(p) => {
            println!(
                "ポート {} は {} (PID: {}) が使用中",
                port.to_string().red().bold(),
                p.process_name.cyan(),
                p.pid.to_string().yellow()
            );
            println!("  コマンド: {}", p.command);
        }
        None => {
            println!("ポート {} は{}", port.to_string().bold(), "未使用".green());
        }
    }
}

pub fn print_kill_result(port: u16, info: &PortInfo, result: Result<(), String>, force: bool) {
    let sig = if force { "SIGKILL" } else { "SIGTERM" };
    match result {
        Ok(()) => println!(
            "{} ポート {} のプロセス {} (PID: {}) に {} を送信しました",
            "✓".green(),
            port,
            info.process_name.cyan(),
            info.pid,
            sig,
        ),
        Err(e) => eprintln!("{} {}", "✗".red(), e),
    }
}
