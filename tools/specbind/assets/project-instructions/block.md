## SpecBind

This project uses SpecBind for spec-driven development. The `specbind` CLI owns
the specification lifecycle: it validates artifacts, records approvals, and is
the only supported writer of machine state.

- Work through the installed `specbind-*` skills. Use `specbind-discovery` to
  turn a request into scope, and `specbind-status` to see where work stands.
- Never hand-edit `spec.yaml`, `tasks.yaml`, or the active roadmap. They are
  CLI-owned, and a hand edit produces state no command validated.
- Run `specbind --help` if the command is unfamiliar or appears unavailable.
