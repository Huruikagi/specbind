# Customize SpecBind for a project

SpecBind exposes specific project-owned customization surfaces. Start with the
desired outcome rather than editing files by guesswork. `sb-configure`
inspects current state, routes the change to its owner, verifies it, and
completes required aftercare.

```text
$sb-configure Make Requirements acceptance criteria more test-oriented.
```

Examples below use the default `.specbind` root. Substitute the configured
`specDir` when different.

## Choose the owning surface

| What you want to change | Owner | Main read path |
| --- | --- | --- |
| Structure, headings, or examples in Requirements, Design, or Roadmap | `settings/templates/` | `template list/read` |
| Shared authoring and judgment criteria | `settings/rules/` | `rule list/read` |
| Release, Git, deferred-finding, or final implementation-validation operations | `settings/adapters/` | `adapter list/read` |
| Durable project context | `steering/` | `steering list/read` |
| Spec root, artifact language, or selected Agents | `.specbind.json` and `install` | `install --dry-run` |
| Models used for Agent roles | `.specbind.json` `agentRoles` | `install --dry-run` after editing |

## Artifact templates

The default project-owned templates are:

- `settings/templates/specs/requirements.md`
- `settings/templates/specs/design.md`
- `settings/templates/specs/ui.md`
- `settings/templates/roadmap.md`

`design.md` and `ui.md` are Design candidates. The
`design-template-selection` Rule classifies every `design/<artifact_id>` as
`required`, `conditional`, or `disabled`. By default `design/main` is required,
while `design/ui` applies only to user-visible screens, interactions, states,
responsive behavior, or accessibility. Add a matching classification and, for
`conditional`, an application condition whenever you add a Design template.
Missing, duplicate, or unknown classifications fail closed.

The Roadmap template controls only project-authored body content such as the
Milestone request, boundaries, decomposition rationale, and dependencies. The
CLI owns `milestone_id`, baseline, target release, and work-item state.

Other embedded Spec and Steering templates can be overridden deliberately at
the `template_path` reported by:

```sh
specbind template list spec
specbind template read spec requirements
specbind template list steering
specbind template read steering document
specbind template list milestone
specbind template read milestone roadmap
specbind template resolve spec <spec> <selector>
```

`template resolve` reports the selected source and exact project-relative
destination. Use that reported path rather than reconstructing it.

### Named creation outputs and instructions

A Markdown template may use project-defined `{{name}}` output references. Every
distinct name requires exactly one `create output=<name>` instruction and at
least one reference. The Agent follows the instruction once and may produce a
short string or a complete Markdown fragment. It places that same output at
every reference to the name. The CLI validates only this correspondence; it
never produces or compares the content.

```markdown
<!-- specbind:instruction create output=components
Produce one H3 subsection for every new or changed responsibility boundary.
Give each subsection its actual component name and describe its responsibility.
-->

{{components}}
```

The `components` output may contain several different H3 sections; it is still
one produced Markdown fragment. Output names must be nonempty and contain no
whitespace or braces. References are not allowed in Front Matter. Missing,
duplicate, unused, or non-`create` output declarations and unresolved live
references are diagnostics. `template read` returns the raw template
byte-for-byte, including instructions and output references.

The official `spec` and `artifact_id` names are not built-ins. Their ordinary
`create output` instructions tell the Agent to produce them from the current
authoring context or literal Front Matter.

The raw template may not itself be a valid live artifact. The Agent follows
creation instructions, replaces output references, adds real content, removes `create`
comments, and then validates the result.

Template changes affect future materialization by default. Existing artifacts
are not silently rewritten. When asked, `sb-configure` previews
reconciliation candidates as format-only, instruction-update, structural,
semantic, or conflict. Applying changes requires a separate confirmation, and
semantic changes route to the artifact-owning workflow. Reconciliation never
rewrites Gate state, completion evidence, released archives, or CLI-owned
structured state merely to match a template.

Every `specbind:instruction` has one scope:

```markdown
<!-- specbind:instruction create Decide the initial identifier. -->
<!-- specbind:instruction maintain Preserve existing identifiers during updates. -->
<!-- specbind:instruction consume Treat this section as context, not authority. -->
```

- `create` is followed only during initial materialization and is not retained.
- `maintain` is copied into the artifact and read during later updates.
- `consume` is copied into the artifact and read when it is used as input.

Read only the relevant durable instruction scope when appropriate:

```sh
specbind artifact read <spec> <selector> --for maintain
specbind artifact read <spec> <selector> --for consume
specbind steering read <selector> --for maintain
specbind steering read <selector> --for consume
```

!!! warning
    Preserve machine-readable structure such as `type`, `artifact_id`, required
    identifiers, and relationships. Customization is supported only around
    those contracts.

## Shared Rules

Rules hold project judgments used across artifacts, Agents, authoring,
validation, and review. Use a template instruction for guidance local to one
artifact shape; use a Rule when the same policy must affect multiple operations.
Product protocols, Skill workflow, and CLI invariants are not project Rules.

| Rule | Project-owned policy |
| --- | --- |
| `ears-format.md` | Requirements expression, subjects, and testability |
| `design-principles.md` | Architecture, interfaces, data, error handling, and design detail |
| `design-template-selection.md` | Applicability of every Design template |
| `contract-principles.md` | Ownership, outward seams, compatibility, and dependency direction |
| `tasks-generation.md` | Task size, decomposition, and testing work |
| `steering-principles.md` | Durable Steering granularity, examples, and update policy |
| `language-style.md` | Natural-language prose across artifacts and Skill reports while preserving exact identifiers |

Only these seven known paths are read in v1. Adding an arbitrary Rule filename does
not extend the registry.

Initial installation creates the six language-neutral defaults for every
project. When `--language ja` is selected, it also creates the Japanese
`language-style.md` default. That Rule is optional and project-owned: every
product Skill reads it for natural-language prose, but its absence leaves the
configured-language output contract intact. A later install keeps an existing
copy rather than replacing it.

```sh
specbind rule list
specbind rule read ears-format --for consume
specbind rule read ears-format --for maintain
```

Rules may contain `maintain` and `consume` instructions, but not creation-only
instructions. An absent Rule reports `NO_CHANGE RULE_ABSENT`; product protocols
still apply. Rules cannot weaken required artifact structure, Gates, approvals,
state transitions, mandatory Skill steps, or CLI validation.

## Operational adapters

Adapters describe project-specific operations in natural language. Code blocks
are guidance for the Agent, not automatically executed hooks.

| Adapter | Purpose |
| --- | --- |
| `release.md` | Prepare, publish, verify, and clean up a release |
| `git.md` | Checkpoint size, staging, commit messages, branch, and push policy |
| `deferred.md` | Destination for real findings that do not hold a Gate |
| `validation.md` | Additional project-specific procedures for final Spec implementation validation |

```sh
specbind adapter list
specbind adapter read git
```

Adapter state distinguishes `absent`, untouched `scaffold`, and `active`. A
scaffold contains the exact `<!-- specbind:adapter-scaffold -->` marker; while
present, its body is not operational policy. Remove the marker after supplying
real guidance. The default deferred adapter records findings in
`.specbind/deferred.md`; this is not a work queue until a person accepts an item
into a Roadmap.

If release remains a scaffold, the Release Skill investigates the repository,
proposes concrete guidance, and after approval saves and locally commits only
`release.md`, then stops. Completion must be revalidated before a later release
run. An empty body with retained Front Matter explicitly means no project-
specific release work.

The Git adapter's default checkpoints each safe workflow unit locally on the
current branch and does not push or rewrite history. Implementation Tasks are
processed and checkpointed one at a time. Agent authorization boundaries still
apply: writing `push` into the adapter does not itself authorize pushing.

The Validation adapter is an inactive scaffold until a project defines extra
final-validation procedure. `sb-configure` can inspect existing scripts, CI,
runtime instructions, fixtures, browser or device setup, and connected-tool
integration, then propose a complete replacement or an update. Active applicable
steps add to the mandatory completion protocol and canonical repository checks;
they never replace or weaken them. A known mismatch is `NO-GO`, while a required
step that cannot run because its environment, credential, device, manual
observer, or tool is unavailable is `MANUAL_VERIFY_REQUIRED`.

The body may describe commands, browser or device interaction, connected tools
such as MCP servers, manual observations, setup, observable success, and
cleanup. The adapter grants no credential use, external mutation, source edit,
or permission to repair a finding. An empty body explicitly adds no project-
specific procedure. Changing this adapter after completion was accepted makes
that earlier evidence stale through the ordinary project-revision rule.

## Steering

Steering records durable project knowledge such as product purpose,
technology, structure, testing, and security direction. Do not use it for
temporary task notes or rapidly changing state. `sb-steering` can
bootstrap, synchronize, repair, or add documents after inspecting the current
catalog.

```sh
specbind steering list
specbind steering read <selector> --for consume
```

The conventional `product`, `tech`, and `structure` split is only a default;
projects may rename, merge, split, or omit them. Steering is not a Gate input,
but editing project content after completion evidence has been accepted can
require completion revalidation. The simplest windows are before the first
completion in a Milestone or after release cleanup.

## Recommended order for shaping a project

For an initial post-install review, or when template choices depend on unknown
project characteristics, start by proposing Steering bootstrap or
synchronization. Then compare the accepted durable guidance and repository
facts with the current Requirements and Design templates and their shared
Rules. Update an existing template or Rule when a responsibility is common to
that surface; add a Design template only when a distinct durable responsibility
needs its own recurring design decisions and traceability across multiple
Specs.

An empty Steering collection remains valid. Do not create it merely because it
is empty, and do not block an explicit narrow template edit that does not need
missing project guidance. Steering records durable project facts; the
`design-template-selection` Rule still decides whether each candidate applies
to one Spec.

Technology labels alone do not require another template. User-visible Web or
mobile work normally uses the existing UI candidate. API compatibility and
infrastructure conventions normally begin in Steering and the Design or
Contract Rules. Add a conditional candidate such as `design/api` or
`design/infrastructure` only when that responsibility repeatedly needs
independent design treatment; write its condition in terms of the
responsibility, not framework presence.

If a request mentions possible future API or infrastructure work but current
Steering and repository facts do not establish a distinct recurring
responsibility, `sb-configure` presents the existing-surface and
conditional-candidate options and asks which is intended. It does not infer a
new candidate from a future technology label alone.

## One-off Design supplements

During Design authoring, the Agent may find that one Spec has a durable
responsibility with its own ownership boundary or verification concern, but no
existing selected Design communicates it clearly. It creates a Spec-local
supplement in the current Design draft, recording an artifact ID, covered
Requirements, target path, and the alternative of extending an existing Design.
This does not add a separate confirmation pause: the ordinary Design Gate
remains the review boundary.

The one-off document lives at `specs/<spec>/design/<artifact_id>.md` below the
configured SpecBind root and is an ordinary `SpecBind Design`: traceability,
validation, fingerprints, and the Design Gate include it. It does not add a
project template or edit `design-template-selection`. If the same responsibility
later recurs independently, the Agent recommends reviewing promotion to a
project-owned conditional candidate.

## Project settings and role models

Initial installation selects artifact language, Agents, Spec root, and project
instruction integration:

```sh
specbind install --dry-run --agent codex --language en --spec-dir .specbind --project-instructions
```

The v1 `specDir` cannot be changed after installation. Agents and language are
stored in `.specbind.json`; Agents may be added later. Use `remove-agent` for
one integration and `uninstall` for the whole project integration.

Override role capabilities through `.specbind.json` rather than editing
generated role files:

```json
{
  "agentRoles": {
    "codex": {
      "implementer": {
        "model": "gpt-5.6-sol",
        "reasoningEffort": "high"
      }
    },
    "claudeCode": {
      "researcher": {
        "model": "sonnet"
      }
    }
  }
}
```

Supported roles are `planner`, `implementer`, `reviewer`, `debugger`, and
`researcher`. `generic` has no generated role definitions. After editing,
ensure the worktree is clean, review `install --dry-run`, and reinstall to
regenerate `.codex/agents/` or `.claude/agents/` files.

## Not customizable

These are product-managed contracts:

- product Skill bodies and generated Agent role definitions;
- embedded protocols and schemas;
- Gates, approvals, fingerprints, transitions, and required traceability;
- CLI-owned state in `spec.yaml`, `tasks.yaml`, and Roadmap; and
- managed blocks in root instruction files.

Put project guidance in a template, Rule, adapter, or Steering document instead
of modifying product-managed files.

## Recommended change flow

Give the coding agent the outcome and let `sb-configure`:

1. inspect current configuration and relevant catalogs;
2. classify the request by ownership;
3. preview the change and obtain required confirmation;
4. apply it through the owner, including reinstall or delegation when needed;
5. rerun mechanical verification and complete or explicitly defer required,
   recommended, and optional aftercare.

Steering authoring remains owned by `sb-steering`, and semantic artifact
changes remain owned by their planning Skills. Configuration still owns the
end-to-end result when it delegates. Deletion, push, branch changes, tags,
history operations, external actions, and lifecycle changes retain their own
authorization boundaries.

---

[Guide home](../index.md) | [Core concepts](./concepts.md) | [Current generated artifact index](../reference/current-artifact-index.md)
