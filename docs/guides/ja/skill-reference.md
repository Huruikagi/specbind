# スキルリファレンス

> 📖 **English guide:** [Skill Reference](../skill-reference.md)

specbind の Skills ワークフロー向けリファレンスである。`--claude-skills` または `--codex-skills` でインストールした場合は、このページを参照する。

## まずどこから始めるか

最初に覚えるべきなのは skill 名の一覧ではなく、「何をしたい時にどこから入るか」である。

| やりたいこと | 最初に使うもの | 次の典型アクション |
| --- | --- | --- |
| 新しい依頼を振り分けたい | `/kiro-discovery` | `kiro-spec-init`、`kiro-spec-batch`、または直接実装 |
| 1つの新規 spec を作りたい | `/kiro-spec-init` | `/kiro-spec-requirements` |
| 大きい構想を複数 spec に分けたい | `/kiro-spec-batch` | 生成された spec をレビューし、承認済みのものから進める |
| 承認済みタスクを実装したい | `/kiro-impl` | `/kiro-validate-impl` |
| feature 全体を検証したい | `/kiro-validate-impl` | findings を直すか、`GO` / `NO-GO` / `MANUAL_VERIFY_REQUIRED` を返す |
| プロジェクト共通コンテキストを整えたい | `/kiro-steering` または `/kiro-steering-custom` | spec workflow を開始または再開 |

## ワークフローの中心になる skills

### `/kiro-discovery`

新しい仕事はあるが、それが 1 spec なのか、複数 spec なのか、spec 不要なのか、まだ分からない時に使う。

- 役割:
  - 依頼を route する
  - scope を整える
  - `brief.md` と必要なら `roadmap.md` を書く
  - 次のコマンドを示して止まる
- 典型的な分岐:
  - 既存 spec に戻す
  - spec 不要として直接実装する
  - 1つの新規 spec を作る
  - 複数 spec に分解する

### `/kiro-spec-batch`

discovery や roadmap の結果、複数 spec に分けるべきと分かっている時に使う。

- 役割:
  - 複数 spec を並列生成する
  - cross-spec の整合性を保つ
  - 1つの巨大 spec ではなく roadmap ベースの backlog を作る

### `/kiro-impl`

`tasks.md` が承認済みで、実装を進めたい時に使う。

- モード:
  - 自律モード: task 引数なし。task ごとに fresh implementer + reviewer + debugger
  - マニュアルモード: task 引数あり。main context で TDD + review gate
- 保証したいこと:
  - reviewer 承認前に完了しない
  - success claim の前に `kiro-verify-completion` を通す
  - remediation / debug は bounded にする

### `/kiro-validate-impl`

実装後に、task 単体ではなく feature 全体を横断して検証したい時に使う。

- 主に見るもの:
  - task 間 integration
  - requirements coverage
  - design alignment
  - full-suite の証拠
- 返り値:
  - `GO`
  - `NO-GO`
  - `MANUAL_VERIFY_REQUIRED`

## 補助的だが重要な skills

これらは独立 skill だが、多くの利用者は `/kiro-impl` の中で間接的に使う。

### `kiro-review`

task-local の adversarial review protocol。

- 主な利用箇所:
  - 自律モードの reviewer subagent
  - マニュアルモードの review gate
- 主に確認するもの:
  - spec compliance
  - boundary fit
  - mechanical verification
  - 必要なら RED-phase evidence

### `kiro-debug`

root-cause-first の debug protocol。

- 使われる場面:
  - implementer が blocked
  - reviewer rejection が収束しない
  - validation で deeper issue が見つかる
- 主な出力:
  - `ROOT_CAUSE`
  - `CATEGORY`
  - `FIX_PLAN`
  - `NEXT_ACTION`

### `kiro-verify-completion`

success claim の前に fresh evidence を要求する gate。

- 主な利用箇所:
  - task 完了前
  - fix が効いたと主張する前
  - feature success を報告する前
- 返り値:
  - `VERIFIED`
  - `NOT_VERIFIED`
  - `MANUAL_VERIFY_REQUIRED`

## `/kiro-impl` の内部: dispatch と iteration

「ここでの subagent って何？」という疑問の大半は `/kiro-impl` の中で起きている。このワークフローは `.claude/agents/kiro/` 配下の事前定義ファイルに依存せず、実装 dispatch は skill 自身が持っている。

### 動的 dispatch（静的 agent ファイルではない）

- `tdd-task-implementer.md` のような事前定義ファイルは `.claude/agents/` 配下に存在しない
- `/kiro-impl` は各プラットフォーム標準の subagent primitive（例: Claude Code の Task tool）経由で fresh 実行コンテキストを都度 spawn する。使うプロンプトテンプレートは skill が持つ
- この設計により、Claude Code と Codex で同じ `/kiro-impl` のワークフロー契約を使いつつ、各環境固有のdispatch記法はそれぞれのSkillテンプレートに保持できる

### タスクごとの 3 ロール

各タスクは最大 3 つのロールで進行する:

- **Implementer** — 仕様から Task Brief を作り、TDD（Feature Flag Protocol の RED → GREEN）で実装する fresh 実行コンテキスト
- **Reviewer** — 独立した reviewer pass。`git diff`、TODO grep、テストスイート、タスク境界の検証を行う
- **Debugger** — implementer が BLOCKED を返したか、reviewer が 2 ラウンド reject した時に起動。失敗履歴を持たないクリーンなコンテキストで root cause を調査し（Web 検索あり）、修正プランを次の implementer に渡す。1 タスクあたり最大 2 ラウンド

これら 3 つのロールは上で触れた 3 つの supporting skill（`kiro-review`、`kiro-debug`、`kiro-verify-completion`）に対応する。dispatch は動的で、`.claude/agents/` 配下にファイルを置く必要はない。

### 知見の伝播

タスクから横断的な知見（例: "better-sqlite3 は Electron 向け ABI rebuild が必要"）が得られた場合、`tasks.md` の `## Implementation Notes` に記録され、以降の implementer プロンプトに注入される。これが「後のタスクが前のタスクの発見を活用する」仕組み。

### 1 task per iteration

各イテレーションは 1 タスクのみ処理する。長時間の自律実行でもコンテキスト衛生を保ち、中断後の `/kiro-impl` 再実行を安全にし、review / debug のスコープを有界に保つため。

## Skills モード dispatch のカスタマイズ

Skillsはプロンプトを動的に生成するため、カスタマイズはsteering、templates、rules、インストール済みSkillファイルを中心に行う。

1. **Steering ドキュメント** — 主なカスタマイズポイント。Implementer と reviewer は steering からルールを継承するので、アーキテクチャや規約の変更は `{{KIRO_DIR}}/steering/*.md` に反映する
2. **Templates と rules** — `{{KIRO_DIR}}/settings/templates/*.md` と `{{KIRO_DIR}}/settings/rules/*.md` を更新して Task Brief と review 観点に影響を与える
3. **Skill ファイル** — 上級者向け。dispatch 動作・review gate・iteration 戦略を調整したい場合は、インストールされた `.claude/skills/` または `.agents/skills/` 配下の `SKILL.md` を直接編集する

## 読む順番のおすすめ

1. [仕様駆動開発ガイド](spec-driven.md)
2. このスキルリファレンス

