---
type: SpecBind Validation Adapter
---

# Validation adapter

<!-- specbind:adapter-scaffold -->

Describe additional project-specific procedures that
`sb-validate-implementation` must perform when deciding whether a Spec's whole
implementation is complete. This prose is not a script: code blocks are steps
to interpret, not hooks that SpecBind executes automatically.

Remove this scaffold marker only after the procedure is deliberate and
complete. Front Matter with an empty body explicitly means that this project
adds no validation procedure beyond SpecBind's mandatory protocol and the
canonical checks established by the repository.

For every applicable procedure, state:

- when it applies;
- the required setup, environment, fixture, account role, device, or tool;
- the exact action or command;
- the observable result that passes or fails the check; and
- cleanup needed after observation.

Procedures may use browser or device interaction, connected tools such as MCP
servers, or manual observation. Name required capabilities, but do not put
credential values or secrets here. If a required procedure cannot run,
validation returns `MANUAL_VERIFY_REQUIRED`; it is never silently skipped or
replaced with a weaker check.

This adapter adds evidence requirements. It cannot waive Requirements, Design,
canonical project checks, or the completion-verification protocol; declare its
own steps passed; grant permission for credentials or external mutations; or
authorize the validator to edit source or repair its own findings.
