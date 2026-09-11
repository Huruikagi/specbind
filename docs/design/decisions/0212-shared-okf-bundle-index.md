# 0212: Install a shared OKF bundle-root index

Status: Accepted

## Context

Decision 0045 defines the configured Spec root as an OKF v0.2 Knowledge Bundle,
but the installed tree does not declare that target in the standard bundle-root
location or provide a human-facing entry point. A generated current inventory
would require ongoing synchronization even though the stable SpecBind directory
roles are already known at installation time. The project may also need its own
links and explanatory content in the same root index.

## Decision

`specbind install` maintains `<specDir>/index.md`, where `specDir` is resolved
from the requested or installed configuration rather than assumed to be
`.specbind`.

The file is a shared ownership surface:

- its bundle-root Front Matter declares `okf_version: "0.2"`;
- a localized product-managed block between exact whole-line
  `<!-- specbind:index -->` and `<!-- /specbind:index -->` markers explains the
  bundle and links the stable `specs/`, `steering/`, and `settings/` entry points;
- the block explicitly describes those links as entry points rather than a
  complete current inventory; and
- every byte outside the block remains project-owned and is preserved across
  refreshes.

For a missing file, installation creates the declaration and localized block.
For an existing root index without Front Matter or without `okf_version`, it
adds the missing declaration and appends the block without deleting existing
text. An existing compatible declaration is preserved. Once the block exists,
a refresh replaces only that exact region with the selected project language.

Exactly one opening and closing marker may appear in order. Missing pairs,
reversed markers, repeated markers, malformed Front Matter, a non-mapping Front
Matter value, or an `okf_version` other than string `"0.2"` stop planning with a
path-specific diagnostic. Installation never guesses which region is owned or
silently retargets another OKF version. Unknown existing Front Matter keys are
preserved when the version declaration is added.

Adding the declaration and first block to existing content removes no project
bytes and uses the same target-scoped safe-addition exception as the marked root
agent instructions. Replacing an established block retains the ordinary
committed, mutation-target-clean installation guard. Planning records exact
prior bytes so apply stops if the shared file changes after preview.

The root index is always installed; it is not controlled by the optional
`projectInstructions` setting and is not removed with one Agent integration.
Project uninstall retains or removes it with the configured complete knowledge
bundle. `specs/index.md` is reserved as a separate, optional future current Spec
browsing view and does not replace this static root entry point. Spec discovery
excludes that exact path now, so adding the file later cannot create a false
Spec identity. This decision does not yet define its generation, content, or
ownership contract.

## Consequences

- The configured bundle carries the standard OKF v0.2 declaration without a
  separate manifest.
- Users can extend the root index without forking a product-managed asset.
- Installation gains one localized embedded asset and one planned target.
- The index is navigation only: it is not lifecycle authority, gate evidence,
  a fingerprint input, or a promise that every linked directory already exists.
- A project may contain `specs/index.md` without breaking Spec enumeration, but
  SpecBind does not create, read, or maintain that file yet.
