# Migrate from cc-sdd

SpecBind supports two cc-sdd migration paths: deterministic automatic
conversion and an agent-assisted path for cases that require semantic
decisions. Start by running the read-only planner. It does not modify the
project.

```sh
specbind migrate cc-sdd
```

!!! warning "Preview"

    `specbind migrate cc-sdd` is not implemented in the current Preview CLI.
    These pages publish the accepted migration procedure before the command is
    released. Do not attempt an in-place cc-sdd cutover with `specbind install`.

Choose a guide:

- [日本語のマイグレーションガイド](../ja/migrate-from-cc-sdd.md)
- [English migration guide](../en/migrate-from-cc-sdd.md)

If the future CLI reports `MANUAL_MIGRATION_REQUIRED`, give the matching guide
URL and the complete CLI output to Codex or Claude Code. The agent must return
to SpecBind CLI validation before declaring the cutover complete.
