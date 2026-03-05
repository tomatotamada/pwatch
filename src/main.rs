mod display;
mod port;
use clap::{Parser, Subcommand};
use figlet_rs::FIGfont;

#[derive(Parser)]
#[command(
    name = "pwatch",
    about = "A fast, friendly port viewer and process killer"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// 全リスニングポートをテーブル表示
    List,
    /// 指定ポートの使用状況を確認
    Check { port: u16 },
    /// 指定ポートのプロセスをキル
    Kill {
        port: u16,
        /// SIGKILLで強制キル
        #[arg(long)]
        force: bool,
    },
    /// TUIモードを起動
    Ui,
}

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("pwatch");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

    let args = Cli::parse();

    match args.command {
        Command::List => {
            let ports = port::scan();
            display::print_port_list(&ports);
        }
        Command::Check { port: p } => {
            let info = port::check(p);
            display::print_check_result(p, info.as_ref());
        }
        Command::Kill { port: p, force } => {
            let info = match port::check(p) {
                Some(info) => info,
                None => {
                    println!("ポート {} は未使用です", p);
                    return;
                }
            };
            let result = port::kill_process(info.pid, force);
            display::print_kill_result(p, &info, result, force);
        }
        Command::Ui => {
            eprintln!("TUIモードは未実装です");
        }
    }
}
