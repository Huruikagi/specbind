# Local-files Source Collection

Use this procedure only when the maintainer explicitly supplies a local file or
directory as Discovery input. Do not infer a source location from conventional
names such as `docs`, `requirements`, or `notes`.

Read the shared semantic contract once before opening the collection:

```sh
specbind protocol read source-material
```

## Resolve the collection

Resolve a relative locator from the project root. The complete collection must
remain inside that root. One file is a one-item collection; one directory means
every descendant item recursively.

Build the inventory before interpreting content. Order it by portable
project-relative path, independent of filesystem enumeration. For every entry,
establish all of these:

- it is an ordinary file, not a directory, symlink, device, or other special
  entry;
- it is Git-tracked at the current baseline, not untracked or ignored;
- it can be read completely as UTF-8 text;
- its text format is understandable enough to classify without guessing.

Use the host's ordinary filesystem and Git read capabilities. Do not invent a
platform-specific shell pipeline as part of the product contract. Never follow a
symlink, leave the project to resolve a target, convert a document, or copy it
into the project.

Report every known unsupported or unreadable entry together. Then stop before
classification, Gate invalidation, milestone mutation, or managed-artifact
authoring. Do not continue with the files that happened to work. Ask the
maintainer to narrow the collection or provide a durable UTF-8 representation.

Record a clean-worktree snapshot before reading and compare it after inventory.
Acquisition is read-only. A changed source file or another acquisition write is
a failure, not a convenience to clean up.

## Classify with complete source coverage

Treat each Source Item as request context and apply the entry and ownership rules
from [ordinary change Discovery](ordinary.md). Several files may describe one
work item; one file may inform several Specs. Do not create a Spec per document.

The confirmation payload keeps the ordinary four fields and adds:

```text
Source coverage: <collection locator; every item; included, excluded, duplicate,
or unresolved disposition; relevant work items; and one-line reason>
```

An unresolved source item means the proposal is not approvable. Ask the
maintainer to settle the ambiguity before changing state.

## Preserve the capture

When creating the Roadmap, fill the selected body with the collection locator
and complete coverage mapping as well as the ordinary request, decomposition,
and dependency rationale. When a milestone already exists, read it with
`specbind milestone scope --include-body` and merge the mapping into the complete
current prose.

For each Spec-backed Brief, name only the exact project-relative Source Items
that inform that Spec and state why each matters. A shared item appears in every
relevant Brief. Direct items still have no Brief.

Do not paste whole source files into Roadmap or Brief. Do not claim that a
locator makes the later artifact self-contained; Requirements and Design own
promotion under the shared protocol.
