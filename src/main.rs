mod display;
mod port;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pwatch", about = "A fast, friendly port viewer and process killer")]
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
    Check {
        port: u16,
    },
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
    let _args = Cli::parse();
}
