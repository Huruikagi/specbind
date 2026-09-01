# Operational adapters

Use adapters for project-specific Release, Git, and deferred-finding procedure.
They are natural-language policy, not executable hooks or permission grants.

## Inspect state

```sh
specbind adapter list
specbind adapter read <selector>
```

Preserve the closed selector set and distinguish:

- `absent`: no project file;
- `scaffold`: exact scaffold marker still present;
- `active`: project guidance; and
- an active document with an empty body: an explicit no-project-specific-work
  choice, not an unconfigured scaffold.

Remove the scaffold marker only when the body states a complete deliberate
policy. Never invent credentials, destinations, release labels, commands, or
external success evidence.

## Authority

An adapter says how an already-authorized workflow should operate. Requesting a
mutating configuration workflow authorizes the narrow local checkpoint required
by an active Git adapter. The adapter alone does not authorize push, branch
changes, tags, deployment, upload, history rewrite, or external messages.

## Aftercare

- A Release adapter change can stale accepted completion. Report the need to
  revalidate completion and start Release again; never continue publication in
  the same authorization.
- A Git adapter change affects later checkpoints. Do not rewrite existing Git
  history to conform to it.
- A deferred adapter change affects later findings. Preserve already recorded
  findings unless moving them was explicitly requested and previewed.

Re-run `adapter list` and `adapter read`. Confirm the intended state rather than
only the file's presence.
