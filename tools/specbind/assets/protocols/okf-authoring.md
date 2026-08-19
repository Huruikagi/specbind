# OKF authoring protocol

This protocol is the shared baseline for creating or rewriting any managed
Markdown document inside a SpecBind spec root. It applies to every supported
agent and cannot be waived by a project template or shared rule.

The configured spec root is an **Open Knowledge Format v0.2 Knowledge Bundle**.
The canonical specification is
[Open Knowledge Format v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md).
This protocol states the authoring constraints SpecBind depends on; it does not
reproduce the specification. Targeting a different OKF version is an explicit
SpecBind compatibility change, never inherited silently from the upstream URL.

## Concept documents

An ordinary managed Markdown file is an OKF *concept document*.

- It begins on its very first line with a YAML Front Matter delimiter. Nothing —
  not a blank line, comment, or byte-order mark — may precede it.
- Its Front Matter must parse as YAML and must contain a non-empty `type`.
- `type` carries the document's machine identity. Never invent, translate, or
  reformat it. When a template supplies it, keep the literal value.
- Collection profiles additionally carry a stable `artifact_id`. It identifies
  the document across revisions, so do not renumber or rename it to describe
  current content.
- The body is Markdown. Unless an artifact profile fixes a structure, headings
  and section order belong to the project's template.

## Preserve what you did not author

- Unknown top-level Front Matter keys are valid OKF extensions. Preserve their
  semantic values when you rewrite a document. CLI-owned round trips may
  canonicalize their order and YAML presentation.
- Do not delete a field merely because SpecBind assigns it no meaning. Another
  tool, or the project itself, may own it.
- Do not add SpecBind-looking metadata that no accepted profile defines.

## Template instruction comments

A Markdown HTML comment whose trimmed content begins with the exact token
`specbind:instruction` is guidance addressed to you, not content of the document
you are producing.

```markdown
<!-- specbind:instruction Summarize the responsibility in one paragraph. -->
```

- It appears only in managed templates. Reading a template returns it intact,
  because it is the reason the template is worth reading.
- Follow it while authoring, then **omit it from the artifact you write**. This
  is the one thing you carry out of a template rather than into the document.
- A leaked comment in a live artifact is a defect the CLI reports. It is not
  inert decoration: a later reader cannot tell instruction from specification,
  and the document stops being the artifact it claims to be.
- Never write one into a live artifact yourself, and never leave one in place
  when revising a document that has one.

A template may also carry scaffold headings with no content beneath them. Those
are structure, not instruction: keep the ones the artifact needs and fill them.

## Reserved files

`index.md` and `log.md` are OKF reserved files, not concept documents. They are
never routed as typed artifacts.

- Neither carries Front Matter, and therefore neither carries a `type`.
- `log.md` is the per-spec release history. Its body is one document title,
  then ISO 8601 `YYYY-MM-DD` date headings ordered **newest first**, with a flat
  prose list under each date.
- SpecBind release finalization inserts `log.md` entries in newest-first order
  and must remain idempotent for the same milestone. Do not hand-append entries
  or reorder existing dates while authoring other artifacts.

## Writing while a completion stands

Completion evidence is bound to a project revision, and the only project change
it tolerates is a Spec's own transition into `release_ready`. Once any
participating Spec holds accepted completion, **every other write stales it**,
and that Spec's completion handshake has to be re-run before the milestone can be
released. This is true however unrelated the write looks; path boundaries do not
establish non-impact.

Check before you write, not after:

```sh
specbind milestone status
```

A Spec whose state is `release_ready` holds accepted completion. If any does:

- say so **before** writing, naming which Specs lose their evidence and that each
  needs its handshake re-run
- let the user decide. The write is not forbidden and the choice is theirs — a
  milestone where nothing can be corrected after its first completion is worse
  than one where corrections are known to cost something

In the ordinary ordering this never arises, because authoring precedes
implementation. It arises in the milestone that has partly finished, which is
where an unannounced revalidation cycle is most expensive.

## Relationships

Express a relationship between documents as an ordinary Markdown link to the
target path. Do not invent a Front Matter reference graph, a custom link syntax,
or a parallel index; SpecBind derives its own traceability from the accepted
artifact profiles rather than from prose links.

## OKF metadata is not SpecBind authority

OKF permits optional lifecycle, trust, provenance, and attestation fields. They
may appear and must be preserved, but they carry no SpecBind meaning.

- Workflow state, gate evidence, approvals, fingerprints, and completion
  evidence live only where the accepted SpecBind profiles put them.
- An OKF status, confidence, or attestation value never advances a gate, marks a
  change approved, or substitutes for CLI-validated state.
- When the two appear to disagree, the SpecBind lifecycle artifacts are
  authoritative and the OKF metadata is descriptive.

## Profiles add to this baseline

Each SpecBind artifact type layers its own required profile on top of this
protocol: exact `type`, singleton or collection multiplicity, required
metadata, and any fixed body grammar. Satisfying this protocol is necessary but
not sufficient.

This document is authoring guidance, not an executable schema. The CLI validates
every deterministic OKF and profile requirement independently, and no protocol,
skill, rule, or template can make CLI-invalid content valid.
