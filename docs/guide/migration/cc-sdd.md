# cc-sddから移行する / Migrate from cc-sdd

まず、読み取り専用のプランナーを実行してください。プロジェクトは変更しません。
Start by running the read-only planner. It does not modify the project.

```sh
specbind migrate cc-sdd
```

!!! warning "Preview"

    旧Specなどのsemantic findingはガイドに従って変換し、外部JSON候補を
    `--accept-resolution`でCLI所有の状態へ受け渡してから`--apply`へ戻ります。

    Convert semantic findings such as legacy Specs through the guide, hand an
    external JSON candidate to CLI-owned state with `--accept-resolution`, then
    return to `--apply`.

## 日本語

SpecBindには、cc-sddからの移行経路が2つあります。機械的に意味を確認できる部分を
自動で変換する経路と、人の判断が必要な部分をエージェントに支援してもらう経路です。

CLIが`MANUAL_MIGRATION_REQUIRED`を返したら、CLIの出力全体と日本語ガイドのURLを、
CodexまたはClaude Codeに渡してください。エージェントは、移行の完了を宣言する前に、
必ずSpecBind CLIの検証へ戻ります。

- [日本語のマイグレーションガイド](../ja/migrate-from-cc-sdd.md)

## English

SpecBind supports two cc-sdd migration paths: deterministic automatic
conversion, and an agent-assisted path for cases that require semantic
decisions.

If the CLI reports `MANUAL_MIGRATION_REQUIRED`, give the complete CLI
output and the English guide URL to Codex or Claude Code. The agent must return
to SpecBind CLI validation before declaring the cutover complete.

- [English migration guide](../en/migrate-from-cc-sdd.md)
