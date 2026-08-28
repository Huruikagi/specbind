---
type: SpecBind Implementation Notes
artifact_id: main
---

<!-- specbind:instruction create bind=spec
Resolve `spec` to the canonical Spec identity in the current authoring context.
Replace every `{{spec}}` reference with that exact value and keep it in the
title so the artifact remains identifiable when read outside its directory.
-->

<!-- specbind:instruction create bind=artifact_id
Resolve `artifact_id` to the literal collection identity in this template's
Front Matter. Replace every `{{artifact_id}}` reference with that exact value
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
