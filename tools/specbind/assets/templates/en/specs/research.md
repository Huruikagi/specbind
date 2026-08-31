---
type: SpecBind Research
---

<!-- specbind:instruction create bind=spec
Resolve `spec` to the canonical Spec identity in the current authoring context.
Replace every `{{spec}}` reference with that exact value and keep it in the
title so the artifact remains identifiable when read outside its directory.
-->

# `{{spec}}` Research

<!-- specbind:instruction create
Materialize this document only when an open question actually blocks Requirements
or Design. Ordinary changes need no research document.
-->

<!-- specbind:instruction maintain
Keep this as current milestone input rather than an append-only activity log.
Record sources, findings, alternatives, and the decision each investigation
supports. Move every conclusion needed after release into Requirements, Design,
or Contract, and remove sections that do not apply.
-->

<!-- specbind:instruction consume
Treat this as milestone-local supporting evidence, not durable authority.
Requirements, Design, and Contract must remain understandable without it.
-->

## Summary

<!-- specbind:instruction maintain
Summarize what the investigation established, what remains unknown, and what
Requirements or Design can now decide. Keep detailed evidence under each
"Research question."
-->

## Research questions

<!-- specbind:instruction maintain
Repeat the following subsection set for every question needed by the current
decision. Keep current conclusions rather than a chronological activity log;
revise a result in place when further investigation changes it.
-->

<!-- specbind:instruction create
Replace `<question>` with an actual open question and repeat the subsection set
for each one. Do not leave the empty example heading in a live artifact.
-->

### `<question>`

#### Context

<!-- specbind:instruction maintain
Explain why this question blocks Requirements or Design and define the boundary
the investigation must resolve.
-->

#### Sources consulted

<!-- specbind:instruction maintain
Identify evidence actually consulted using a URL, document name, or
project-relative path that another reader can revisit. Do not present an
unsupported assumption as a finding.
-->

#### Findings

<!-- specbind:instruction maintain
Distinguish facts, constraints, and remaining uncertainty established from the
sources. Do not state a conclusion without its supporting evidence.
-->

#### Implications

<!-- specbind:instruction maintain
State which Requirements, Design, or Contract decision this result enables, or
what remains unresolved. Delete a research question whose implications cannot
be explained.
-->

## Options and trade-offs

<!-- specbind:instruction maintain
Use this section only when several viable choices remain for the same question.
Compare what each gains, gives up, and the condition under which it should be
selected. Delete this section when only one viable option remains.
-->

## Risks and follow-up

<!-- specbind:instruction maintain
Record uncertainty that remains after investigation, time-bounded assumptions,
and conditions that require another check. Delete this section when none apply.
Keep sources under their research question rather than duplicating them here.
-->
