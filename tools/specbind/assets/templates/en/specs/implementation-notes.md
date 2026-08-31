---
type: SpecBind Implementation Notes
artifact_id: main
---

<!-- specbind:instruction create output=spec
Produce `spec` from the canonical Spec identity in the current authoring context.
Replace every `{{spec}}` reference with that exact output and keep it in the
title so the artifact remains identifiable when read outside its directory.
-->

<!-- specbind:instruction create output=artifact_id
Produce `artifact_id` from the literal collection identity in this template's
Front Matter. Replace every `{{artifact_id}}` reference with that exact output
and keep it in the title so separate note collections remain distinguishable.
-->

# `{{spec}}` Implementation Notes — `{{artifact_id}}`

<!-- specbind:instruction maintain
Persistent free-form memory for whoever implements this change next. Record the
non-obvious: rejected approaches, environment quirks, and decisions that the
Requirements, Design, and Contract documents do not already carry.
-->

<!-- specbind:instruction consume
Treat this as implementation memory, not specification authority. This document
is never an approval gate input.
-->

## Implementation cautions

<!-- specbind:instruction maintain
Give each independently useful topic a descriptive H3 heading. Keep together
the fact, reason, impact, and verification or workaround the next implementer
needs. Remove activity history, facts obvious from current code, and decisions
already owned by Requirements, Design, or Contract. When no caution remains,
do not preserve an empty section; consider removing this document itself.
-->

<!-- specbind:instruction create
Replace the empty section with at least one real H3 heading and substantive
implementation caution. Do not save a headings-only live artifact.
-->

### `<caution>`
