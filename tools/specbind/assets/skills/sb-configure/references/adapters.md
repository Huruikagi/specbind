# Operational adapters

Use adapters for project-specific Release, Git, deferred-finding, and final
implementation-validation procedure. They are natural-language policy, not
executable hooks or permission grants.

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

## Validation adapter

The `validation` adapter adds project-specific work to final Spec implementation
validation. Inspect repository evidence such as canonical scripts, CI, runtime
instructions, browser or device setup, fixtures, and existing external-tool
integration before proposing it. Present a complete replacement for a scaffold
or an exact diff for active guidance. Do not invent a command, credential,
environment, external destination, tool capability, or success observation.

Keep the boundary explicit:

- Requirements and Design say what must hold; the adapter says how to obtain
  additional project-specific evidence.
- Active applicable guidance adds to the product protocol and canonical project
  checks. It cannot replace, waive, narrow, or declare them passed.
- The body may describe commands, browser or device interaction, connected tools
  such as MCP servers, manual checks, setup, observable success, and cleanup.
  A code block is still prose to interpret, never an automatically executable
  hook.
- Absence, a scaffold, or an intentionally empty body means no additional
  project-specific procedure. It never weakens the existing validation scope.
- The adapter grants no credential, external mutation, source edit, or finding
  repair authority.

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
- A Validation adapter change alters the evidence required for later completion.
  Existing accepted completion becomes stale through the ordinary project-revision
  freshness rule; report the need to revalidate rather than treating an earlier
  `GO` as evidence under the new procedure.

Re-run `adapter list` and `adapter read`. Confirm the intended state rather than
only the file's presence.
