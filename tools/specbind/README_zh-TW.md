# SpecBind: 面向 AI 程式代理的長時間規格驅動實作

[![license: MIT](https://img.shields.io/badge/license-MIT-green.svg)](../../LICENSE)

<div align="center" style="margin-bottom: 1rem; font-size: 1.1rem;"><sub>
<a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a> | 繁體中文
</sub></div>

**把已核准規格轉成長時間自律實作工作流。** 單一指令將 agentic SDLC 工作流安裝為 Agent Skills: discovery, requirements, design, tasks 以及帶有任務級別 independent review 的自律實作。為 Claude Code 與 Codex 提供相同的 17-skill 套件。

👻 **Kiro 風格。** Kiro IDE 的 spec-driven / agentic SDLC 風格。既有 Kiro 規格可直接使用。

## v3.0 的新功能

specbind v3.0 是圍繞 Agent Skills 與長時間自律實作的重寫。

- **`/kiro-discovery` 作為新入口。** discovery 把新需求路由到「擴充既有 spec / 直接實作 / 建立一個新 spec / 拆成多個 spec / mixed decomposition」其中之一。它會寫入 `brief.md` 以及必要時的 `roadmap.md`，讓你可以在不重新說明 scope 的情況下恢復工作。
- **`/kiro-impl` 執行長時間自律實作。** 每個任務由 fresh implementer 在 feature flag 後執行 TDD (RED → GREEN)，獨立的 reviewer 做機械驗證，失敗時由 auto-debug pass 在乾淨 context 中調查根本原因。任務間的知見透過 `tasks.md` 的 `## Implementation Notes` 傳給下一個 implementer。每次迭代處理 1 個任務，中斷後再執行也安全。
- **邊界優先的 spec discipline。** `design.md` 新增 File Structure Plan，成為任務邊界的依據。任務帶有 `_Boundary:_` / `_Depends:_` 標註。review 與 validation 尋找邊界違規而非僅看風格。
- **`/kiro-spec-batch` 支援多 spec initiative。** 從 roadmap 並行產生多個 spec，並執行 cross-spec review 以捕捉 spec 間矛盾、責務重複與介面不一致。
- **Claude Code 與 Codex 的 Agent Skills。** 每次安裝 17 個 skills、按需載入（progressive disclosure）。兩個整合都由實機持續維護與驗證。零外部依賴，subagent 透過各平台原生 spawn 啟動。

Skills 模式完整工作流與 `/kiro-impl` 內部細節請參考 [Skill Reference](https://github.com/Huruikagi/specbind/blob/main/docs/guides/skill-reference.md)。

從 v1.x 或 v2.x 升級？請參考 [Migration Guide](https://github.com/Huruikagi/specbind/blob/main/docs/guides/migration-guide.md#5-v2x-to-v30)。

## 為什麼選 specbind

specbind 把 spec 視為系統各部分之間的契約，不是交給代理的「命令文件」。程式碼仍然是 source of truth，spec 是用來讓程式碼各部分之間的契約變得明確，讓人類與代理可以平行工作而不需要持續同步。

賭注很簡單: 適當粒度的明確契約會讓團隊規模的 AI 驅動開發變得更快，而不是更慢。代理寫 spec，人類在 phase gate 核准契約，真正出貨的是程式碼。

邊界不是額外負擔，邊界的存在讓你可以在內部自由移動，同時保護外部。

完整的設計理由、取捨、適用與不適用情境: [Why specbind? A philosophy note](https://github.com/Huruikagi/specbind/blob/main/docs/guides/why-specbind.md)（英文版）。

## 快速開始

```bash
cd your-project
npx specbind@latest
```

預設會安裝 **Claude Code Skills** 與英文文件。若要指定 Codex 或其他語言:

```bash
npx specbind@latest --codex-skills --lang ja      # Codex, 日語
npx specbind@latest --claude-skills --lang zh-TW  # Claude Code, 繁體中文
```

支援 Claude Code、Codex 和13種語言。詳情請參考 [支援的代理](#支援的代理)。

然後在你的代理裡執行:

```bash
/kiro-discovery <想做的事情>
```

不確定從哪裡開始？先執行 `kiro-discovery`。它會幫你整理需求並告訴你下一步該用什麼指令。

### 常見工作流

| 你想做的事 | Skills 模式 |
|---|---|
| 開始新功能或產品規模的構想 | `kiro-discovery` → `kiro-spec-init` → `kiro-spec-requirements` → `kiro-spec-design` → `kiro-spec-tasks` → `kiro-impl` |
| 擴充既有系統 | `kiro-steering` → `kiro-discovery` 或 `kiro-spec-init` → 可選 `kiro-validate-gap` → `kiro-spec-design` → `kiro-spec-tasks` → `kiro-impl` |
| 分解大型 initiative | `kiro-discovery` → `kiro-spec-batch` |
| 不需要 spec 的小改動 | `kiro-discovery` → 直接實作 |

對較大規模的已核准任務集合，執行 `kiro-impl` 會以任務級別的 subagent spawn、independent review、失敗時 auto-debug 開始自律實作。

## 實際操作

範例: 建立一個新的 Photo Albums 功能。

```bash
/kiro-discovery Photo albums with upload, tagging, and sharing
# discovery 會寫入 brief.md（多 spec 時還會寫 roadmap.md）並提示下一個指令
/kiro-spec-init photo-albums
/kiro-spec-requirements photo-albums
/kiro-spec-design photo-albums
/kiro-spec-tasks photo-albums
/kiro-impl photo-albums
# 自律執行: 每個任務使用 fresh implementer, independent reviewer, auto-debug
```

spec 階段的典型產出（10 分鐘以內）:

- `requirements.md`: EARS 格式的需求與驗收條件。
- `design.md`: 附有 Mermaid 圖與 File Structure Plan 的架構文件。
- `tasks.md`: 帶有邊界與相依性標註的實作任務。

接著 `/kiro-impl` 會以 feature flag 後的 TDD (RED → GREEN)、獨立的 reviewer pass 以及失敗時的 auto-debug 自律執行任務。

## 支援的代理

兩個受支援的整合提供相同的17-skill套件，並以實機直接驗證。

| 代理 | Skills 模式 | 狀態 |
|---|---|---|
| **Claude Code** | `--claude-skills` | 支援 |
| **Codex** | `--codex-skills` | 支援 |

## 安裝詳情

### 語言

```bash
npx specbind@latest --lang zh-TW # 繁體中文
npx specbind@latest --lang ja    # 日語
npx specbind@latest --lang es    # 西班牙語
# 支援語言: en, ja, zh-TW, zh, es, pt, de, fr, ru, it, ko, ar, el
```

### 進階選項

```bash
# 套用前先預覽變更
npx specbind@latest --dry-run --backup

# 自訂 specs 目錄
npx specbind@latest --kiro-dir docs
```

## 自訂

編輯 `{{KIRO_DIR}}/settings/` 下的模板與規則以符合團隊工作流。

- `templates/`: requirements, design, tasks 的文件結構。
- `rules/`: AI 生成原則與判斷基準。

常見使用情境: PRD 風格需求、API 與資料庫 schema、核准關卡、JIRA 整合、領域特定標準。

實用範例與可複製程式碼片段請參考 [自訂指南](https://github.com/Huruikagi/specbind/blob/main/docs/guides/customization-guide.md)。

## 專案結構

安裝後，專案將新增:

```
project/
# 僅會安裝其中之一
├── .claude/skills/           # 17 skills（Claude Code Skills，預設）
├── .agents/skills/           # 17 skills（Codex Skills）
# 共用的專案記憶與 spec 狀態
├── .kiro/settings/templates/ # 共用模板（以 {{KIRO_DIR}} 展開）
├── .kiro/settings/rules/     # 共用規則
├── .kiro/specs/              # 功能規格文件
├── .kiro/steering/           # AI 指導文件
└── CLAUDE.md / AGENTS.md     # 專案設定（依代理而異）
```

實際只會建立所選代理需要的目錄。

## 文件

- Skill 參考: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/skill-reference.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/skill-reference.md)
- 指令參考: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/command-reference.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/command-reference.md)
- 自訂指南: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/customization-guide.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/customization-guide.md)
- 規格驅動開發指南: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/spec-driven.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/spec-driven.md)
- Why specbind? 哲學說明: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/why-specbind.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/why-specbind.md)
- Claude 子代理指南: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/claude-subagents.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/claude-subagents.md)
- 遷移指南: [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/migration-guide.md) | [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/migration-guide.md)
- [問題與支援](https://github.com/Huruikagi/specbind/issues) - 問題回報與提問
- [Kiro IDE](https://kiro.dev)

---

**穩定版 v3.0.0** 生產環境就緒。[回報問題](https://github.com/Huruikagi/specbind/issues) | MIT License

### 平台支援

- 支援 OS: macOS, Linux, Windows（預設自動偵測）。
- 所有平台使用統一的指令模板。`--os` 參數保留給相容性需求，可視情況指定。
