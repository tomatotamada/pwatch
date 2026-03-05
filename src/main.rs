mod config;
mod display;
mod platform;
mod port;
mod tui;
use clap::{Parser, Subcommand};
use colored::Colorize;
use figlet_rs::FIGfont;

#[derive(Parser)]
#[command(
    name = "pwatch",
    about = "A fast, friendly port viewer and process killer"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    /// 出力をJSON形式にする
    #[arg(long, global = true)]
    pub json: bool,
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
    /// 設定を変更
    Config {
        /// 設定項目 (banner)
        key: String,
        /// 値 (on/off)
        value: String,
    },
}

fn main() {
    let args = Cli::parse();
    let cfg = config::load();

    if cfg.show_banner && !args.json {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("pwatch").unwrap();
        let colors = [
            "red", "yellow", "green", "cyan", "blue", "magenta",
        ];
        for line in figure.to_string().lines() {
            for (i, ch) in line.chars().enumerate() {
                print!("{}", ch.to_string().color(colors[i % colors.len()]));
            }
            println!();
        }
    }

    match args.command {
        Command::List => {
            let ports = port::scan();
            if args.json {
                display::print_json(&ports);
            } else {
                display::print_port_list(&ports);
            }
        }
        Command::Check { port: p } => {
            let info = port::check(p);
            if args.json {
                display::print_json(&info.into_iter().collect::<Vec<_>>());
            } else {
                display::print_check_result(p, info.as_ref());
            }
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
            let mut terminal = ratatui::init();
            let mut app = tui::app::App::new();
            loop {
                terminal
                    .draw(|f| tui::ui::draw(f, &app))
                    .expect("描画に失敗しました");
                tui::handler::handle_events(&mut app).expect("イベント処理に失敗しました");
                if app.should_quit {
                    break;
                }
            }
            ratatui::restore();
        }
        Command::Config { key, value } => {
            let mut cfg = cfg;
            match key.as_str() {
                "banner" => match value.as_str() {
                    "on" => {
                        cfg.show_banner = true;
                        config::save(&cfg).expect("設定の保存に失敗しました");
                        println!("バナー表示を有効にしました");
                    }
                    "off" => {
                        cfg.show_banner = false;
                        config::save(&cfg).expect("設定の保存に失敗しました");
                        println!("バナー表示を無効にしました");
                    }
                    _ => eprintln!("値は on または off を指定してください"),
                },
                _ => eprintln!("不明な設定項目: {} (使用可能: banner)", key),
            }
        }
    }
}
