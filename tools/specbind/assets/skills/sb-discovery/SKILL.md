---
name: sb-discovery
description: Turn a change request, Source Collection, or selected existing implementation into confirmed Spec and milestone boundaries. Owns ordinary scope discovery and the fixed-revision reverse-establishment orchestration.
argument-hint: "<change, source path, GitHub Milestone, or existing area to adopt>"
---

# Decide what the work is

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

## Select exactly one Discovery procedure

Use reverse mode only when the maintainer explicitly asks to establish Specs
from working code and tests or to resume an active reverse establishment. Read
[Reverse establishment](references/reverse.md) completely and follow it. It
owns the confirmed orchestration through Requirements, Design, Contract
Review, and non-release finalization. Do not also read the ordinary procedure.
An ordinary change to an existing repository never triggers an implementation
scan merely because code exists.

Every other request uses [ordinary change Discovery](references/ordinary.md).
Read that procedure completely before classifying or changing anything. It
owns the shared entry, ownership, confirmation, mutation, Brief, checkpoint,
and reporting contract.

For an ordinary request, also read exactly one provider procedure when its
explicit selector is present:

- For a local file or directory explicitly supplied as Discovery input, read
  [Local-files Source Collection](references/local-files.md) completely before
  classification. Do not infer a conventional source directory.
- For explicit `OWNER/REPO` plus a Milestone number, or exactly
  `https://github.com/OWNER/REPO/milestone/NUMBER`, read
  [GitHub Milestone Source Collection](references/github-milestone.md)
  completely before classification. No other URL shape selects that provider.

Do not load a provider procedure for an ordinary conversational request. Do not
substitute one provider when an explicit selector is invalid or unavailable.

## Shared authority boundary

The invocation selects a procedure; it does not confirm a scope that has not
been presented. Follow the selected procedure's confirmation boundary before
any lifecycle or artifact mutation. The CLI performs lifecycle and state
changes; Discovery authors only the artifacts its selected procedure names.
