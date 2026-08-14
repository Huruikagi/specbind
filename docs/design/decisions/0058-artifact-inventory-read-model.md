# 0058: Separate artifact inventory from content reads

Status: Accepted

## Context

Decision 0057 discovers spec-local Markdown by OKF type and assigns stable logical selectors independent of current paths. Agent workflows sometimes need a compact way to decide which artifacts matter before spending context on their bodies. When a required singleton or other selector is already known, they instead need a safe direct read without a redundant inventory round trip, reimplementing discovery, or trusting an agent-supplied path.

A single command that always returns every body would defeat the context-saving goal. Concatenating multiple raw Markdown documents would also introduce ambiguous artificial separators and make it unclear which bytes belong to which artifact.

## Decision

- The read-only CLI command family is:

  ```text
  specbind artifact list <spec> [--json]
  specbind artifact read <spec> <selector>... [--json]
  ```

- `<spec>` is the canonical spec identity. Artifact commands resolve the configured SpecBind root and never accept a spec directory path as a substitute.
- `artifact list` runs Decision 0057 discovery and profile validation but does not return document bodies or fingerprints.
- `artifact read` accepts logical selectors only. It re-runs current discovery, resolves selectors to current paths, validates the selected profiles, and reads the files. An agent-supplied path or fingerprint is not accepted as authority.
- `artifact list` is not a prerequisite for `artifact read`. Direct read provides the same current selector resolution and selected-profile validation whether or not the caller previously listed the spec.
- Both commands are read-only and never repair frontmatter, rename files, update evidence, or mutate lifecycle state.

## Inventory model

- JSON inventory has this top-level shape:

  ```json
  {
    "schema_version": 1,
    "spec": "checkout",
    "valid": true,
    "artifacts": [],
    "diagnostics": []
  }
  ```

- Every recognized artifact item contains exactly:
  - `selector`: its Decision 0057 logical selector
  - `type`: the exact OKF type
  - `path`: its current SpecBind-root-relative POSIX path
  - `artifact_id`: required only for collection profiles and omitted for singletons
- Inventory deliberately omits content, fingerprint, timestamps, Git revision, byte size, lifecycle evidence, and derived semantic summaries. Workflows needing those facts use their owning CLI operation.
- Artifact ordering is deterministic: `brief`, `requirements`, all `design/<artifact_id>`, `contract`, then all `implementation-notes/<artifact_id>`. Each collection is ordered by `artifact_id`. Mapping-key order is presentation only.
- `valid` is true only when discovery and every recognized SpecBind profile in the spec scope are valid. On discovery failure, the CLI returns every unambiguous artifact it safely discovered plus structured diagnostics, sets `valid: false`, and exits nonzero. Partial inventory is diagnostic information and does not authorize a lifecycle operation.
- Each diagnostic has stable `code`, `severity`, and `message` fields. It may add `selector`, `path`, `line`, and `column` when known. Artifact-command error codes and the complete versioned diagnostics schema remain part of CLI implementation design, but fields may not be removed or repurposed within schema version 1.
- Human output is a compact one-artifact-per-line rendering of the same ordered inventory, followed by diagnostics when present. It contains no additional semantic state that is absent from JSON.

## Content read model

- Without `--json`, `artifact read` requires exactly one selector and writes that artifact's original UTF-8 Markdown content to standard output without a SpecBind wrapper, heading, separator, or normalization.
- Diagnostics are written to standard error so successful raw standard output remains solely the selected document.
- Multiple selectors require `--json`. Supplying multiple selectors without it is a usage error.
- JSON read output has this top-level shape:

  ```json
  {
    "schema_version": 1,
    "spec": "checkout",
    "artifacts": [
      {
        "selector": "design/persistence",
        "type": "SpecBind Design",
        "artifact_id": "persistence",
        "path": "specs/checkout/persistence.md",
        "content": "---\ntype: SpecBind Design\nartifact_id: persistence\n---\n"
      }
    ]
  }
  ```

- Read results preserve request order. Duplicate requested selectors are a usage error rather than duplicated output.
- A multi-artifact read is all-or-nothing: if any requested selector is missing, ambiguous, invalid, or unreadable, the command returns no content payload and exits nonzero with diagnostics.
- Errors unrelated to the requested selectors remain visible as diagnostics from discovery, but they do not prevent a uniquely resolved, valid selected artifact from being read. Lifecycle and gate commands may still reject the overall invalid inventory under their stricter invariants.
- Unknown OKF types have no SpecBind logical selector in v1 and are not returned or readable through these commands. They remain valid bundle content and may receive an explicit extension discovery contract later.

## Agent usage

- A workflow directly requests a known singleton such as `requirements`, `contract`, or `brief` through `artifact read` without listing first.
- A workflow may also directly read a known collection selector such as `design/persistence` when that stable ID came from authoritative workflow context.
- A workflow uses `artifact list` when it needs to discover all members of a collection, determine which optional artifacts exist, choose among selectors, or diagnose the spec's artifact structure.
- If a direct read reports a missing or ambiguous selector, the workflow may use `artifact list` to obtain the broader inventory and diagnostics.
- Workflows never independently search filenames. Gate and review mutations independently rediscover and fingerprint their authoritative input set rather than trusting an earlier list or read response.

## Consequences

- Known singleton reads avoid a redundant inventory round trip, while collection discovery still costs only a small, predictable context envelope.
- Filename changes do not require skill changes because skills name logical selectors.
- Raw single-document reads remain natural Markdown, while multi-document reads retain explicit provenance in JSON.
- Discovery races cannot turn an earlier path into mutation authority; guarded operations always resolve and fingerprint current artifacts again.
- Project-wide inventory and unknown-type extension discovery remain separate follow-up capabilities rather than expanding the v1 spec-local command contract.
