# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

`pwatch` — ポート使用状況の可視化とプロセスキルを行うCLI/TUIツール（Rust製）。
「ポートが既に使われている」エラー発生時に、ポート確認→プロセス特定→killを1コマンドで完結させる。

## ビルド・実行

```bash
cargo build              # ビルド
cargo run -- list        # 全リスニングポート表示
cargo run -- check 8080  # 特定ポートの使用状況確認
cargo run -- kill 8080   # ポートのプロセスをkill
cargo run -- ui          # TUIモード起動
cargo test               # テスト実行
cargo clippy             # リント
cargo fmt                # フォーマット
```

## アーキテクチャ

```
src/
├── main.rs          # CLIエントリポイント (clap derive)
├── port.rs          # PortInfo構造体、ポート情報取得、プロセスkill
├── display.rs       # CLI表示（テーブル・カラー・JSON出力）
├── platform/        # OS固有のポート情報取得（trait + #[cfg]で抽象化）
│   ├── mod.rs       # PortScanner trait定義、PlatformScanner re-export
│   ├── linux.rs     # /proc/net/tcp パース → inode → PID解決
│   └── macos.rs     # lsof -iTCP -sTCP:LISTEN パース
└── tui/             # TUIモード (ratatui + crossterm)
    ├── app.rs       # App状態管理、AppMode (Normal/Search/Confirm)
    ├── ui.rs        # ratatui描画ロジック
    └── handler.rs   # キーイベントハンドラ
```

### 主要データ構造

- `PortInfo` — ポート情報（port, protocol, pid, process_name, command）
- `App` — TUI状態（ports, selected, filter, mode, should_quit）
- `AppMode` — Normal / Search / Confirm の3状態
- `PortScanner` trait — `scan() -> Vec<PortInfo>` をOS別に実装

### プラットフォーム抽象化

`platform/mod.rs`で`PortScanner` traitを定義し、`#[cfg(target_os)]`で`PlatformScanner`としてre-export。利用側は`PlatformScanner::scan()`を呼ぶだけ。

- **Linux**: `/proc/net/tcp{,6}`パース → ステート0x0A(LISTEN)抽出 → inode → `/proc/[pid]/fd/`走査でPID特定
- **macOS**: `lsof -iTCP -sTCP:LISTEN -P -n`の出力をパース

## CLIインターフェース

| コマンド | 説明 |
|---|---|
| `pwatch list [--json]` | 全リスニングポートをテーブル/JSON表示 |
| `pwatch check <PORT> [--json]` | 指定ポートの使用状況確認 |
| `pwatch kill <PORT> [--force]` | プロセスkill（デフォルトSIGTERM、--forceでSIGKILL） |
| `pwatch ui` | TUIモード起動 |

## TUIキーバインド

- `q`/`Esc`: 終了（Searchモードではキャンセル）
- `↑`/`k`, `↓`/`j`: 選択移動
- `d`: SIGTERMキル確認、`D`: SIGKILLキル確認
- `/`: 検索モード、`r`: リフレッシュ、`y`/`n`: 確認応答

## 実装ロードマップ

1. **Phase 1**: 基本CLI（list/check/kill + /procパース + nixでkill）
2. **Phase 2**: 表示改善（colored テーブル + JSON出力 + serde導入）
3. **Phase 3**: TUIモード（ratatui + crossterm）
4. **Phase 4**: クロスプラットフォーム（macOS対応、trait抽象化）
5. **Phase 5**: 公開準備（README、CI/CD、crates.io）

## エラーハンドリング方針

- 権限不足: 取得可能な情報のみ表示 + `sudo`ヒント
- kill失敗: `--force`ヒントを表示
- エラーメッセージは「何が起きたか」+「次に何をすべきか」を含める
