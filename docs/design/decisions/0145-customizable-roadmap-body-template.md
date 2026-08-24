# 0145: Install a project-owned Roadmap body template

Status: Accepted

## Context

The active Roadmap deliberately remains Markdown because its body carries the
milestone-wide request and the rationale shared across work items. The CLI owns
the deterministic scope and dependency graph in Front Matter, while Decision
0046 leaves the body free-form for context such as approach, boundaries,
constraints, and decomposition decisions.

In practice, `milestone create` supplied only `# Roadmap` when its scope
candidate omitted `body`. Unlike Requirements and Design, the Roadmap had no
template that an authoring skill could read. The representation therefore
supported prose without giving a project a visible scaffold for writing it.

Decision 0008 already accepts `settings/templates/` as the shared
project-customization surface. Decision 0091 intentionally began with a narrow
installed set and allows that set to widen when a project has a real structural
choice. Roadmap body structure is such a choice, but Roadmap machine state is
not.

## Decision

- The official English and Japanese defaults include one milestone template
  with selector `roadmap`.
- `specbind install` creates the project-owned copy at
  `{{SPEC_DIR}}/settings/templates/roadmap.md` when absent. Refresh never
  overwrites an existing copy under Decision 0008.
- The template is an OKF document with `type: SpecBind Roadmap`. It omits the
  CLI-owned live fields `milestone_id`, `baseline_revision`, `target_release`,
  and `work_items`; those fields are invalid in this template profile.
- Its Markdown body is the customizable scaffold for the active Roadmap body.
  Headings and guidance may be changed by the project. They carry no machine
  semantics.
- The template is exposed through the read-only milestone template scope:

  ```text
  specbind template list milestone
  specbind template read milestone roadmap
  ```

- Discovery reads and materializes this template when it creates a new active
  milestone. It follows the scoped-instruction and OKF authoring protocol:
  `create` guidance does not enter the live body, while `maintain` and `consume`
  guidance remains durable.
- An existing active Roadmap is never reconciled with later template edits.
  Same-milestone scope changes begin with the complete existing Roadmap body
  and revise it in place only when the confirmed request changes its prose.
- The authoritative Roadmap is still created and mutated only through guarded
  milestone CLI operations. A template read is authoring input, not mutation
  authority.
- If a caller omits `body` outside the product Discovery workflow, the CLI may
  retain its minimal `# Roadmap` fallback. Installing a template does not make
  arbitrary low-level scope candidates implicitly author prose.

## Consequences

- Projects can define how milestone-wide change requests and decomposition
  rationale are recorded without changing the Roadmap DAG contract.
- A newly installed or refreshed project gains one more user-owned settings
  file, while existing customized settings remain untouched.
- Template Front Matter cannot masquerade as live milestone state.
- Template updates affect future Roadmaps only; Git and release archives remain
  the history of Roadmaps already created.

## Implementation status

Implemented in the Rust template catalog, installer, milestone template CLI
scope, and embedded Discovery skill. Focused catalog and CLI tests cover both
languages, project override resolution, install materialization, and rejection
of CLI-owned Roadmap fields.
