# Configuration aftercare

Run this phase after every successful configuration mutation.

## Re-establish current state

Run:

```sh
specbind configuration show
```

Then run the list, read, dry-run, parser, or status checks named by every
applicable configuration procedure. Judge from CLI output and repository state,
not from delegated narration.

## Classify follow-up

- **Required**: the configuration is invalid, a derived product asset has not
  been regenerated, or the requested surface is unusable without it. Complete
  this before reporting success.
- **Recommended**: consistency or the next ordinary workflow benefits from it,
  but current configuration remains valid. Perform it when already authorized;
  otherwise present it precisely.
- **Optional**: it changes additional artifacts, lifecycle evidence, durable
  knowledge, Git state beyond the active adapter-directed checkpoint, or an
  external system. Preview it and obtain its own choice.

Common mappings:

| Change | Follow-up |
| --- | --- |
| Agent or role | Reinstall and verify derived assets |
| Template | Offer existing-artifact reconciliation |
| Design candidate or selection | Inspect affected existing Specs |
| Rule | Offer review, never automatic rewrite |
| Steering | Check active-work assumptions |
| Adapter | Check the owning operational workflow |
| Language | Enumerate retained old-language project content, including `language-style` |
| Project instructions | Verify exact managed blocks and Skill discovery |

Declining optional follow-up completes the requested configuration change. Name
what remains and its effect without reporting it as a failure.

After required and authorized aftercare, read the Git adapter. Treat the
configuration change as one eligible workflow unit: when the active adapter
requires a local checkpoint, stage only that unit and create it before
reporting completion. Never absorb unrelated changes or optional aftercare that
was declined.

## Separate authority boundaries

Do not infer authorization for existing-artifact mutation, lifecycle approval,
push, branch changes, tags, deployment, publication, destructive removal, or
history rewrite from the configuration request. The narrow local checkpoint
above is the only Git exception. Follow the owning workflow and its confirmation
boundary for each.
