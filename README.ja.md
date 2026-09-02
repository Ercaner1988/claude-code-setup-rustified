**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code スタンドアロンセットアップ (Rust コア単一バイナリ)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

**Claude Code** 環境を管理するためのローカルな配備・セキュリティ監査・記憶エンジン (`claude-code-setup`) です。実行時は単一の Rust バイナリであり、実行にあたって Rust・Python・Node のインストールは必要ありません。

### 誠実性に関する注記: 本リポジトリは 100% Rust ではありません

GitHub の言語統計および計測されたコードベース構成比 (2026-09-02): **Rust 2891 行 / 非 Rust 279 行 = 行数比 91.2% Rust** (GitHub Linguist: **Rust 90.5%, Shell 3.5%, Python 3.2%, PowerShell 2.8%**)。

| 言語 / ファイル | 行数 | GitHub 割合 | 実行される場面 |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`, 10 ファイル) | 2891 | 90.5% | 実行時 (CLI + MCP サーバー) |
| `install-macos.sh` (Shell / Bash) | 121 | 3.5% | Linux/macOS へのインストール時 |
| `package-extension.py` (Python) | 87 | 3.2% | リリース発行時 / `.mcpb` パッケージングのみ (CI) |
| `install-windows.ps1` (PowerShell) | 71 | 2.8% | Windows へのインストール時 |

追加の依存関係:
- `src/security.rs` 内の pre-commit フックは**埋め込み bash スクリプト**として書き出されます (`#!/usr/bin/env bash`)。フックの実行には Git 付属の bash が必要です。
- `.github/workflows/release.yml` のリリース経路は `actions/setup-python` と `npx @anthropic-ai/mcpb validate` を用いるため、**CI 連鎖は Python と Node に依存します**。
- 記憶エンジンの埋め込み層は `fastembed` を通じて **ONNX Runtime の事前コンパイル済み C++ バイナリをダウンロードします** (`ort-download-binaries`)。

正確な要約: **実行時バイナリは純粋な Rust。ただしインストール・パッケージング・リリース経路は Bash + PowerShell + Python + Node を使用します。**

---

## 🎯 1. 到達点と完了事項

- **単一バイナリの実行時:** 従来の Bash および Python の*実行時*スクリプトは Rust へ移行されました。インストールおよびパッケージング用スクリプト (`install-*.{sh,ps1}`、`package-extension.py`) は意図的に残されています。インストーラー自体はバイナリがダウンロードされる前に動作しなければならないからです。
- **動的なパス正規化:** ハードコードされたパス定義 (例: `/home/jb_remus`) は、対象 OS とローカルユーザーのホームディレクトリに動的に適合します。
- **複数ターゲット対応の MCP 管理 (`--target`):**
  - **Claude Code** (`~/.claude.json`)、**プロジェクト** (`./.mcp.json`)、**Claude Desktop** (`claude_desktop_config.json`) の各設定を単一の CLI から管理できます。
  - `serde_json::Value` 構造により型付けされていない JSON フィールドを保持する原子的書き込みエンジン (`.bak` 自動バックアップ付き)。
- **MCP サーバーモード (`--mcp-mode`):** 同一のバイナリが stdin/stdout 上で JSON-RPC を話す MCP サーバーへと変化し、`manifest.json` に宣言された 8 個のツールを Claude Desktop に提供します。
- **記憶エンジン (SQLite + ベクトル + グラフ):**
  - **高速なノート追加 (`memory-note`):** kebab-case のファイル名による安全なノート作成。
  - **FTS5 キーワード検索:** 引用符のエスケープ機構を備えた SQLite キーワード索引。
  - **ローカル埋め込み:** `fastembed` (Multilingual-E5-Small) によるコサイン類似度。モデルは初回使用時に Hugging Face からダウンロードされ `$HOME/.claude/fastembed_cache` に保存されます。**この初回ダウンロード以降**、検索は完全にオフラインで動作します。
  - **グラフ辺と Wikilink:** `[[ノート名]]` リンクおよび閾値を超える意味的な辺を経由した近傍検索 (`memory-related`)。
  - **RRF ハイブリッド順位付け:** Reciprocal Rank Fusion (`k=60`) による FTS5 とベクトル検索の融合。
- **自動修正付きセキュリティ監査 (`security-audit --fix`):**
  - 設定ファイル内の平文機密情報の走査 (`ghp_`、`github_pat_`、`sk-`、`xox[baprs]-`、`AKIA`)。
  - ファイル権限を 600 へ引き締め — **Unix のみ**。Windows では ACL ベースの権限に関する情報が表示されるだけで、修正は行われません。
  - Git pre-commit のブランチ保護および機密情報走査フックの設置。
- **自律的な Git ワークフロー (`agent-workflow`):**
  - リモートの既定ブランチからの機能ブランチ自動作成。
  - 保護された主要ブランチへの直接 push の阻止。

---

## 🏗️ 2. アーキテクチャとモジュール

```
claude-code-setup-rustified/
├── Cargo.toml                  # Rust 依存関係とパッケージ定義 (v0.1.6)
├── manifest.json               # Claude Desktop 拡張マニフェスト (8 MCP ツール)
├── icon.png                    # 拡張アイコン
├── .env.example                # 環境変数の見本
├── src/
│   ├── main.rs                 # CLI 入口点とコマンド振り分け (123 行)
│   ├── cli.rs                  # Clap によるコマンド・ターゲット・フラグ定義 (222)
│   ├── mcp.rs                  # 複数ターゲット対応、JSON Value 保持型 MCP 管理器 (488)
│   ├── mcp_server.rs           # MCP stdio JSON-RPC サーバー。8 ツールを CLI へ対応付け (436)
│   ├── memory_engine.rs        # FTS5 + ベクトル + グラフ + RRF + memory-note エンジン (821)
│   ├── installer.rs            # 骨格ディレクトリ・初期 README・.env の設置器 (191)
│   ├── security.rs             # 自動修正付きセキュリティ監査器とフック管理器 (296)
│   ├── branch_manager.rs       # 保護ブランチ防護付き自律 Git ワークフロー実行器 (161)
│   ├── tester.rs               # システム・環境診断テスト実行器 (123)
│   └── agent.rs                # エージェント統合インターフェース (30)
├── install-windows.ps1         # PowerShell インストーラー (Rust ではない)
├── install-macos.sh            # Bash インストーラー (Rust ではない)
├── package-extension.py        # .mcpb パッケージャー、CI で呼ばれる (Rust ではない)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # 3 プラットフォームのバイナリと .mcpb のリリース経路
└── docs/                       # インストールおよび障害対応の手引き
```

### モジュールの責務
- `src/main.rs`: コマンドライン引数を解析し、`--mcp-mode` が与えられた場合は制御を MCP サーバーへ、そうでなければ該当モジュールの関数へ委譲します。
- `src/cli.rs`: Clap の `Parser` 構造により 15 個のサブコマンド、フラグ (`--target`、`--fix`、`--hooks`、`--mode`、`--min-score`)、および全体フラグ `--mcp-mode` を管理します。
- `src/mcp.rs`: `--target` (`claude-code`、`project`、`claude-desktop`) に応じて MCP 設定を読み書きし、未知のフィールドを削除せずに原子的に書き込みます。
- `src/mcp_server.rs`: stdin/stdout の JSON-RPC ループを構成し、`manifest.json` の 8 ツール (`mcp_list`、`mcp_add`、`security_audit`、`memory_note`、`memory_index`、`memory_search`、`status`、`test`) を実際の CLI コマンドへ対応付けます。この対応付けは `her_arac_gercek_bir_cli_komutuna_esleniyor` テストにより固定されています。
- `src/memory_engine.rs`: ノートを約 1500 文字の窓に分割して埋め込み、平均化 (mean-pooling) します。SQLite の `knowledge_notes` と `note_edges` テーブルを管理します。埋め込みキャッシュは `$HOME/.claude/fastembed_cache`。
- `src/installer.rs`: `$HOME/claude_global_memory/knowledge` ディレクトリと初期 `README.md` を決して上書きせずに作成し、`.env` が無ければ複製します。
- `src/security.rs`: 平文の機密情報を走査し、権限を検査し、`--fix` で修正し、Git フックを設置します (フックは埋め込み bash スクリプトです)。
- `src/branch_manager.rs`: 自律的なブランチ作成、保護ブランチの阻止、安全な commit/push の流れを管理します。
- `src/tester.rs`: システム診断 (`status`) とテスト検証を行います。

---

## 🚀 3. インストールと設定

### クイックスタート

インストールは 2 種類あります。どちらを望むか決めてください。

**Claude Desktop 拡張 (推奨)** — [最新リリース](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest)からお使いの OS に合うパッケージをダウンロードし、Claude Desktop → Settings → Extensions 画面へドラッグしてください:

| OS | ファイル | 概算サイズ |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 MB |
| macOS | `claude-code-setup-macos.mcpb` | 10 MB |
| Linux | `claude-code-setup-linux.mcpb` | 12 MB |

**コマンドライン道具** — 端末から使う場合:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

これらのインストーラーは PowerShell と Bash のスクリプトであり (Rust ではありません)、ダウンロードしたバイナリをユーザーディレクトリへ設置し PATH に追加します (管理者権限は不要)。拡張の**登録は行いません** — 拡張には上記の `.mcpb` の経路を用いてください。確認には新しい端末で `claude-code-setup status` を実行します。

詳細なインストールは [INSTALLATION.ja.md](INSTALLATION.ja.md) を参照してください。

---

### 手動インストール: ソースからのビルド

#### 必要条件
- **Rust ツールチェーン:** `rustc` と `cargo` (1.80 以上)
- 初回ビルド時、`fastembed` が ONNX Runtime のバイナリをダウンロードするため、ネットワーク接続が必要です。

#### ビルド
```bash
cargo build --release

# 生成されるバイナリ:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### 自動セットアップと環境診断
```bash
# 前提条件を確認し、記憶の骨格を設置します
./target/release/claude-code-setup install --hooks

# システムおよび環境の診断状態
./target/release/claude-code-setup status
```

---

## 📖 4. 使用法と実例

### コマンド一覧

| コマンド | 説明 |
| :--- | :--- |
| `--mcp-mode` (全体フラグ) | バイナリを stdin/stdout で JSON-RPC を話す MCP サーバーとして実行します |
| `install [--hooks] [--skip-prereqs]` | 環境設定、記憶の骨格、`.env` の複製 |
| `test` / `status` | Claude CLI、`.claude.json`、記憶 DB、フックの診断 |
| `mcp-list [--target T]` | 設定済み MCP サーバーをターゲット別に一覧表示します |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | MCP サーバーを追加または更新します (`--target`: `claude-code`、`project`、`claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | 変数を削除、またはサーバーを完全に除去します (`--remove` が必須) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | 設定を壊さずにサーバーを有効化/無効化します |
| `memory-note <題名> [--body ...] [--dir D]` | 知識基盤へ新しい Markdown ノートを追加します |
| `memory-index [--source ディレクトリ]… [--edge-threshold 0.70]` | ノートを SQLite + ベクトル + グラフエンジンへ索引付けします |
| `memory-search <問い> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | FTS5 キーワード、ベクトル、または RRF ハイブリッドの各モードで記憶を検索します |
| `memory-related <note.md>` | グラフ辺と Wikilink を通じて関連ノートを一覧表示します |
| `install-hooks [--repo-dir パス]` | リポジトリへ pre-commit セキュリティフックを設置します |
| `security-audit [--fix]` | セキュリティ監査を実施し、`--fix` で自動修正を適用します |
| `agent-workflow [--branch-type 種別] --description 説明 [--files F]…` | 保護ブランチ防護付きの自律 Git ブランチ・commit ワークフローを実行します |

すべてのコマンドはテスト隔離のため `--home-dir` の上書きを受け付けます (`install-hooks` と `agent-workflow` を除く)。

### 使用例

#### ターゲット別の MCP サーバー管理
```bash
# プロジェクト水準 (.mcp.json) で MCP サーバーを定義
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# Claude Desktop 設定内のサーバーを無効化
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# サーバーを完全に除去 (--remove フラグは必須)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### 記憶へのノート追加と RRF ハイブリッド検索
```bash
./target/release/claude-code-setup memory-note "設計上の決定" --body "実行時の Rust ネイティブバイナリ化が完了した。"
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "Rust 設計" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. テストと品質ゲート

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

ソースには **31 個のテスト**が定義されています。うち 1 個 (`test_enforce_file_permissions_fixes_mode`) は `#[cfg(unix)]` で標されているため Windows ではコンパイルされません。計測結果: **Windows で 30/30、Unix で 31/31 が緑** (2026-09-02)。

ファイル別内訳: `memory_engine.rs` 14、`mcp.rs` 5、`mcp_server.rs` 5、`security.rs` 3、`branch_manager.rs` 2、`installer.rs` 2。

### 品質基準
- **網羅範囲:** MCP の複数ターゲット管理、JSON `Value` の保持、FTS5 の文字エスケープ、RRF ハイブリッド順位付け、mean-pooling、Wikilink 解析、埋め込みキャッシュ経路の回帰、機密情報走査、MCP ツールと CLI の対応付け、保護ブランチの防護。
- **書式:** `cargo fmt --all -- --check` → 清潔 (2026-09-02)。
- **静的解析:** `cargo clippy --all-targets -- -D warnings` → 警告なし (2026-09-02)。
- **継続的統合:** `.github/workflows/rust.yml` が 3 つの OS (ubuntu、windows、macos) で fmt + clippy + test + release ビルドを実行します。`.github/workflows/release.yml` は 3 プラットフォームのバイナリと `.mcpb` パッケージを生成します。この経路は Python と Node を使用します。

---

## 👥 6. 貢献者

以下の数値は `git shortlog -sne --all` と、コミット本文の `Co-authored-by` 標識の計数によって計測されました (2026-09-02、総計 45 コミット)。

| 貢献者 | 役割 / 責務 | 計測された貢献 |
| :--- | :--- | :--- |
| **Ercan ER** | 設計、Rust 移行、主開発者 | 41 コミット (著者) |
| **Claude Opus 5** | 自律 AI エージェント、モジュール開発 | 14 コミット (共著者) |
| **Copilot App** | AI コーディング補助 | 11 コミット (共著者) |
| **Claude Opus 4.8** | 自律 AI エージェント | 3 コミット (共著者) |
| **Claude** (版指定なし) | 自律 AI エージェント | 2 コミット (共著者) |
| **jb_remus** | 上流 (upstream) の原著者 | 2 コミット (著者) |
| **Mihenk** | コード検査官・品質審判 | 1 コミット (著者) |
| **arturo-ebuck** | オープンソース貢献者 | 1 コミット (著者) |

**Kassam** は `Cargo.toml` の `authors` 欄に記録されたエージェント名であり、独立した Git 著者記録はありません。

---

## 📄 7. ライセンスと資料

本プロジェクトは [MIT ライセンス](LICENSE) の下で提供されます (著作権 © 2026 Ercan Er)。

### 関連文書
- [配備の手引き](DEPLOYMENT_GUIDE.md)
- [手動セットアップの手引き](docs/MANUAL_SETUP.md)
- [障害対応の手引き](docs/TROUBLESHOOTING.md)
- [開発者向け指令](docs/dev/TASK-KASSAM-1-2.md)
