**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code スタンドアロンセットアップ (100% Rust エンジン)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

**Claude Code** 環境を高性能、動的、かつ安全に管理するために開発された **100% Rust ベース** のローカルデプロイメント、セキュリティ監査、およびメモリエンジン (`claude-code-setup.exe`) です。

従来の Bash (`.sh`) および Python (`.py`) スクリプトは完全に削除され、単一の Rust CLI ツールに統合・再構築されました。

---

## 🎯 1. 目的と特徴

- **100% 純粋な Rust アーキテクチャ:** Shell スクリプトや Python ランタイムへの依存関係が一切ありません。
- **動的パス正規化:** ハードコードされたパスパターン (例: `/home/jb_remus`) は、ターゲット環境およびローカルホームディレクトリに自動調整されます。
- **マルチターゲット対応 MCP 管理 (`--target`):**
  - **Claude Code** (`~/.claude.json`)、**プロジェクト** (`./.mcp.json`)、および **Claude Desktop** (`claude_desktop_config.json`) の設定を同じ CLI から動的に管理可能。
  - 未定義の JSON フィールドを保持し、自動 `.bak` バックアップを作成する安全な設定書き込み。
- **SQLite ベースの高速メモリエンジン (ベクトル + グラフ):**
  - **高速ノート追加 (`memory-note`):** 既存ファイルを上書きせずに kebab-case ファイル名で安全にノートを作成。
  - **FTS5 キーワード検索:** 特殊クエリ構文に対する自動エスケープを備えた全文検索。
  - **ローカル埋め込み:** `fastembed` (Multilingual-E5-Small) を使用した完全にオフラインでのコサイン類似度計算。
  - **グラフエッジと Wikilink:** `[[Note-Name]]` 参照およびセマンティック結合に基づく BFS 近傍探索 (`memory-related`)。
  - **RRF ハイブリッドランキング:** Reciprocal Rank Fusion (`k=60`) アルゴリズムにより、キーワード検索とベクトル検索を最高精度で統合。
- **自動修復機能付きセキュリティ監査 (`security-audit --fix`):**
  - 設定ファイル内のプレーンテキスト機密トークンをスキャン。
  - Unix システムにおけるファイル権限の自動修復。
  - Git ブランチ保護および機密スキャンフックの自動インストール。
- **自律的かつ安全な Git ワークフロー (`agent-workflow`):**
  - リモートのデフォルトブランチからフィーチャーブランチを自動作成。
  - 保護されたメインブランチへの直接プッシュを防止。

---

## 🏗️ 2. アーキテクチャとモジュール

```
claude-code-complete-setup/
├── Cargo.toml                  # プロジェクトマニフェストおよび Rust 依存関係
├── src/
│   ├── main.rs                 # CLI エントリポイントおよびコマンドルーター
│   ├── cli.rs                  # Clap ベースのコマンド、ターゲットおよびフラグ定義
│   ├── mcp.rs                  # マルチターゲット対応の MCP サーバーマネージャー
│   ├── memory_engine.rs        # FTS5 + ベクトル + グラフ + RRF + memory-note エンジン
│   ├── installer.rs            # スケルトンディレクトリ、初期 README、.env 作成
│   ├── security.rs             # 自動修復付きセキュリティ監査人およびフックマネージャー
│   ├── branch_manager.rs       # 保護ブランチ対応の Git ワークフロー実行エンジン
│   ├── tester.rs               # 診断テストスイート実行エンジン
│   └── agent.rs                # エージェント統合インターフェース
└── docs/                       # セットアップおよびトラブルシューティングガイド
```

### モジュールの役割
- `src/main.rs`: CLI 引数を解析し、対応するモジュール関数に割り当てます。
- `src/cli.rs`: Clap の `Parser` 構造体を通じて、すべてのサブコマンド、オプション (`--target`, `--fix`, `--hooks`, `--mode`)、ヘルプテキストを管理します。
- `src/mcp.rs`: `--target` パラメータ (`claude-code`, `project`, `claude-desktop`) に基づいて MCP 設定を読み書きします。
- `src/memory_engine.rs`: テキストを分割して埋め込み、`knowledge_notes` および `note_edges` テーブルを管理します。`memory-note` により安全にノートを追加します。
- `src/installer.rs`: `~/claude_global_memory/knowledge` ディレクトリおよび初期 `README.md` を安全に初期化します。
- `src/security.rs`: 権限と機密情報を監査し、`--fix` で自動修復し、Git フックを設置します。
- `src/branch_manager.rs`: ブランチ作成および保護ブランチガードのチェックを自動化します。
- `src/tester.rs`: システムの診断 (`status`) およびテスト実行を行います。

---

## 🚀 3. インストールと設定

### 必須要件
- **Rust ツールチェーン:** `rustc` および `cargo` (1.80 以上)

### コンパイル
```bash
# リリースバイナリのビルド
cargo build --release

# 生成されるバイナリ:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### 自動セットアップおよび診断
```bash
# 自動セットアップの実行とセキュリティフックの設置
./target/release/claude-code-setup install --hooks

# 診断ステータスの実行
./target/release/claude-code-setup status
```

---

## 📖 4. 使用方法と例

### コマンド概要一覧

| コマンド | 説明 |
| :--- | :--- |
| `install [--hooks]` | 完全自動環境セットアップ、スケルトン初期化および `.env` 設定 |
| `test` / `status` | Claude CLI、`.claude.json`、メモリ DB、フックの診断ステータス表示 |
| `mcp-list [--target T]` | 指定ターゲットの設定済み MCP サーバー一覧を表示 |
| `mcp-set <srv> [...] [--target T]` | MCP サーバーの設定を追加・更新 (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | 設定項目の削除、またはサーバーの完全削除 (`--remove` 必須) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | 設定を保持したまま MCP サーバーの有効化 / 無効化 |
| `memory-note <タイトル> [--body ...]` | ナレッジベースに新しい Markdown ノートを安全に追加 |
| `memory-index [--source ディレクトリ]...` | ノートを SQLite + ベクトル + グラフエンジンにインデックス化 |
| `memory-search <クエリ> [--mode ...]` | FTS5 キーワード、ベクトル、または RRF ハイブリッドモードで検索 |
| `memory-related <ノート.md>` | グラフエッジを通じて関連ノートを探索 |
| `install-hooks [--repo-dir パス]` | Git pre-commit ブランチ保護フックをインストール |
| `security-audit [--fix]` | セキュリティ監査を実行。`--fix` で自動修復を適用 |
| `agent-workflow [-t 種別] -d 説明` | 保護ブランチガード付きの自律型 Git ワークフローを実行 |

### 使用シナリオ例

#### ターゲット別 MCP サーバーの管理
```bash
# プロジェクトレベル (.mcp.json) で MCP サーバーを設定
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# Claude Desktop 設定のサーバーを無効化
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# サーバーの完全削除 (--remove フラグが必須)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### ノートの追加とハイブリッド検索
```bash
# 新しいノートの追加
./target/release/claude-code-setup memory-note "アーキテクチャの決定" --body "Rust による単一バイナリ化が完了。"

# ノートのインデックス化
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# RRF ハイブリッド検索の実行
./target/release/claude-code-setup memory-search "Rust アーキテクチャ" --mode hybrid --limit 5

# 関連ノートの探索
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. 品質ゲートとテスト

本プロジェクトには 24 個の単体テストが含まれており、すべて正常に通過しています:

```bash
cargo test
```

### 品質基準
- **単体テスト (24/24 パス):** マルチターゲット MCP 管理、JSON Value 保持、FTS5 エスケープ、RRF ハイブリッドランキング、Mean-pooling、Wikilink 抽出、機密情報監査、保護ブランチガード。
- **コードフォーマット:** `cargo fmt --check`
- **継続的インテグレーション (CI):** Ubuntu、macOS、Windows 上で `.github/workflows/rust.yml` および `.github/workflows/release.yml` を使用して検証。

---

## 👥 6. 貢献者

| 貢献者 | 役割 / 責任 | メトリクス |
| :--- | :--- | :--- |
| **Ercan ER** | リードアーキテクト、Rust 移行および主開発者 | 26 commits |
| **Kassam** | 自律型 AI エージェント、Rust エンジンおよびモジュール開発者 | 共同制作者 / 貢献者 |
| **Copilot** | AI コーディングアシスタント | 4 commits |
| **jb_remus** | オリジナル開発者 (Upstream) | 2 commits |
| **Mihenk** | コード監査人および品質査定者 | 1 commit |
| **arturo-ebuck** | オープンソース貢献者 | 1 commit |

---

## 📄 7. ライセンスとリソース

本プロジェクトは [MIT ライセンス](LICENSE) のもとで公開されています。

### 関連ドキュメント
- [デプロイメントガイド](DEPLOYMENT_GUIDE.md)
- [手動セットアップガイド](docs/MANUAL_SETUP.md)
- [トラブルシューティングガイド](docs/TROUBLESHOOTING.md)
- [開発者指示書](docs/dev/TASK-KASSAM-1-2.md)
