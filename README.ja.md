# pwatch

ポート使用状況の可視化とプロセスキルを行うCLI/TUIツール。

[English README](README.md)

「ポートが既に使われている」エラー発生時に、ポート確認 → プロセス特定 → kill を1コマンドで完結させます。

## インストール

```bash
cargo install --path .
```

## 使い方

### 全リスニングポートを表示

```bash
pwatch list
```

JSON形式で出力:

```bash
pwatch list --json
```

### 特定ポートの使用状況を確認

```bash
pwatch check 8080
```

### ポートを使用しているプロセスをキル

```bash
pwatch kill 8080          # SIGTERM
pwatch kill 8080 --force  # SIGKILL
```

権限エラーが出る場合:

```bash
sudo pwatch kill 8080
```

### TUIモード

```bash
pwatch ui
```

| キー | 操作 |
|------|------|
| `j` / `↓` | 選択を下に移動 |
| `k` / `↑` | 選択を上に移動 |
| `d` | SIGTERM でキル (確認あり) |
| `D` | SIGKILL でキル (確認あり) |
| `/` | 検索モード |
| `r` | リフレッシュ |
| `q` / `Esc` | 終了 |

## 対応プラットフォーム

| OS | スキャン方法 |
|----|-------------|
| Linux | `/proc/net/tcp` 直接パース |
| macOS | `lsof` コマンド経由 |

## ビルド

```bash
cargo build --release
```

## ライセンス

MIT
