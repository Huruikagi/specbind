# 対応エージェントを Claude Code と Codex に限定する移行計画

## 目的

未公開の specbind 3.x の対応対象を、実機で継続検証できる Claude Code と Codex に限定する。
対応数よりも、各 Skill の挙動を実際に検証し、両環境で同じワークフローを保証できることを優先する。

## 移行方針

- **作業種別**: migration（破壊的な対応範囲縮小）
- **統合範囲**: skills
- **残すインストール対象**:
  - `claude-code-skills`（`--claude-skills`、`--claude-code-skills`）
  - `codex-skills`（`--codex-skills`）
- **削除する対象**:
  - Cursor
  - GitHub Copilot
  - Gemini CLI
  - Windsurf
  - Qwen Code
  - OpenCode
  - Antigravity
  - Claude Code の非推奨 commands / agents モード
  - Codex のブロック済み prompts モード
- **互換性**: 破壊的変更。未公開のため非推奨期間は設けない。

## 調査結果

### 残す2環境

| 項目 | Claude Code | Codex |
|---|---|---|
| 登録ID | `claude-code-skills` | `codex-skills` |
| Skill配置先 | `.claude/skills/kiro-*/` | `.agents/skills/kiro-*/` |
| プロジェクト指示ファイル | `CLAUDE.md` | `AGENTS.md` |
| Skill呼び出し例 | `/kiro-discovery` | `$kiro-discovery` |
| Skill本体 | `SKILL.md` | `SKILL.md` |
| UIメタデータ | 必要なSkillの `agents/openai.yaml` | 必要なSkillの `agents/openai.yaml` |
| 引数表現 | Skill本文中の `<...>` / コマンド例 | Skill本文中の `<...>` / コマンド例 |

外部仕様の新規導入ではなく、すでに実装・検証済みの2環境を残す変更であるため、公式ドキュメントから新しいフォーマット仕様を採取する作業は不要と判断する。

### 現在の構造

- エージェント登録とCLI別名は `tools/specbind/src/agents/registry.ts` に集約されている。
- manifestは `tools/specbind/templates/manifests/<agent-id>.json` にある。
- エージェント別テンプレートは `tools/specbind/templates/agents/<agent-id>/` にある。
- 各実インストール経路は `tools/specbind/test/realManifest*.test.ts` で検証される。
- READMEは現在「8プラットフォームで同じ17 Skill」を保証しているため、2環境への表現変更が必須である。

## 基準パターンと差分

基準は `claude-code-skills` と `codex-skills` の既存実装とする。新しいエージェント形式は追加しない。

両者で維持する差分:

- Skill配置先: `.claude/skills` と `.agents/skills`
- プロジェクト指示ファイル: `CLAUDE.md` と `AGENTS.md`
- 呼び出し記法: `/kiro-*` と `$kiro-*`
- Claude固有frontmatter / tool宣言と、Codex向け `agents/openai.yaml`
- subagent起動方法など、各環境固有の実行指示

削除後は、共通の意味変更をこの2テンプレートへ同じ変更単位で反映する。

## 実装計画

### 1. エージェント登録とCLIを2経路へ限定する

- `tools/specbind/src/agents/registry.ts` から、`claude-code-skills` と `codex-skills` 以外の登録を削除する。
- Codex promptsモード専用のブロック処理など、到達不能になる分岐を `tools/specbind/src/index.ts` から削除する。
- CLIヘルプ、選択UI、設定の既定値が2経路だけを示すことを確認する。
- `tools/specbind/package.json` のキーワードから削除対象エージェント名を除く。

### 2. 不要なmanifestを削除する

残すもの:

- `tools/specbind/templates/manifests/claude-code-skills.json`
- `tools/specbind/templates/manifests/codex-skills.json`

それ以外のエージェントmanifestを削除する。

### 3. 不要なエージェント別テンプレートを削除する

残すもの:

- `tools/specbind/templates/agents/claude-code-skills/`
- `tools/specbind/templates/agents/codex-skills/`

それ以外の `tools/specbind/templates/agents/<agent-id>/` を削除する。共有設定テンプレートは維持する。

### 4. テストを2環境の契約へ更新する

- `realManifestClaudeCodeSkills.test.ts` と `realManifestCodexSkills.test.ts` を残す。
- 削除した経路専用の `realManifest*.test.ts` を削除する。
- `args.test.ts`、`agentLayout.test.ts`、CLI entry系テストなどから削除済みID・別名の期待値を除く。
- 削除したエージェントIDとCLIフラグが拒否されることを、代表例で明示的に検証する。
- 残した2環境の17 Skill、配置先、言語切替、上書き動作の既存検証を維持する。

### 5. 公開ドキュメントを2環境へ合わせる

- `tools/specbind/README.md`
- `tools/specbind/README_ja.md`
- `tools/specbind/README_zh-TW.md`
- `README.md` および `docs/README/` 配下の現行案内
- `docs/guides/` と `docs/guides/ja/` の対応表・CLI例・移行案内
- 必要に応じて `CHANGELOG.md` とrelease notes

過去リリースの履歴として残す文脈と、現在サポートしているように読める文脈を区別する。過去の事実を機械的に消さず、現行仕様の記述だけを2環境へ更新する。

## 変更対象

### 編集

- `tools/specbind/src/agents/registry.ts`
- `tools/specbind/src/index.ts`
- `tools/specbind/package.json`
- CLI・layout・設定関連テスト
- 現行サポート範囲を説明するREADMEとガイド
- `CHANGELOG.md` または次回リリース向け記録

### 削除

- Claude Code / Codex Skills以外のmanifest
- Claude Code / Codex Skills以外のエージェント別テンプレート
- 上記の実インストール経路だけを検証するテスト

### 作成

- 原則なし（本計画書を除く）

## 検証計画

`tools/specbind` で以下を実施する。

1. `npm test`
2. `npm run build`
3. Claude Code Skillsのdry-run
   - `node dist/index.js --agent claude-code-skills --dry-run`
4. Codex Skillsのdry-run
   - `node dist/index.js --agent codex-skills --dry-run`
5. 一時ディレクトリへの `--overwrite=force` 適用テスト
6. `--lang ja` と `--lang en` を両環境で確認
7. 両環境で17 Skillが期待する配置先へ生成されることを確認
8. 削除済みの代表的なID・フラグ（例: `cursor-skills`、`--gemini-skills`）が受理されないことを確認
9. リポジトリ全体を検索し、現行サポートを「8環境」とする記述や削除済みCLI例が残っていないことを確認

## ロールバック

この変更を単独コミットにまとめ、そのコミットをrevertすることで、登録・manifest・テンプレート・テスト・ドキュメントを一括復元できるようにする。

## 未解決事項・承認ポイント

実装前に次の前提を承認する。

1. 「2環境」は、エージェントの種類だけでなくインストール経路もSkills版2つに限定する。
2. 非推奨の `--claude` / `--claude-agent` と、ブロック済みの `--codex` も削除する。
3. 未公開のため、他エージェント向けCLIに非推奨警告や互換スタブを残さない。

## 完了条件

- CLIがClaude Code SkillsとCodex Skillsだけを列挙・受理する。
- パッケージに2環境以外のmanifest・エージェント別テンプレートが含まれない。
- 現行ドキュメントが2環境のみをサポート対象として説明する。
- 両環境のテスト、build、dry-run、適用テスト、日英言語確認が成功する。
- 削除済みエージェントの選択が明確な入力エラーになる。

## 実装・検証結果

- `npm test`: 24 test files / 144 tests passed
- `npm run build`: 成功
- Claude Code Skills: `en` / `ja` のdry-run成功
- Codex Skills: `en` / `ja` のdry-run成功
- 一時ディレクトリへの実適用:
  - Claude Code Skills: 17 Skill、`CLAUDE.md`、共有設定、日本語指示を確認
  - Codex Skills: 17 Skill、`AGENTS.md`、共有設定、日本語指示を確認
- 削除済み入力の拒否:
  - `--gemini-skills`: 拒否
  - `--agent cursor-skills`: 拒否
- `npm pack --dry-run --json --ignore-scripts`:
  - packaged agent roots: `claude-code-skills`, `codex-skills`
  - packaged manifests: `claude-code-skills.json`, `codex-skills.json`
  - packaged files: 120
