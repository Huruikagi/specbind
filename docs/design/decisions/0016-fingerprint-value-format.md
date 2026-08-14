# 0016: Encode fingerprints as tagged lowercase SHA-256 values

Status: Accepted

## Context

Gate evidence must retain fingerprints of the exact input revisions that passed so the CLI can detect later out-of-band edits. The stored value needs an unambiguous algorithm identifier and a strict representation that behaves consistently across YAML, JSON Schema, Rust, diagnostics, and tests.

This decision defines only the stored fingerprint value. Input selection, metadata projection, and content canonicalization remain separate decisions.

## Decision

- A fingerprint is stored as a string in the form `sha256:<digest>`.
- `<digest>` is exactly 64 lowercase hexadecimal characters.
- The JSON Schema pattern is `^sha256:[0-9a-f]{64}$`.
- Uppercase hexadecimal, omitted algorithm tags, alternate separators, and surrounding whitespace are invalid.
- Equality uses exact string comparison after the producer has computed the canonical input bytes defined by the applicable gate contract.

Example:

```yaml
input_revisions:
  requirements.md: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

The `sha256:` tag makes the persisted algorithm explicit and leaves room for a future schema version to accept another algorithm without guessing from digest length.

## Consequences

- Schema validation can reject malformed fingerprint values before semantic gate validation.
- Rust and TypeScript tooling can exchange fingerprint values without a separate algorithm field.
- Producers must emit lowercase hexadecimal.
- Changing the accepted algorithm or representation requires an explicit schema evolution decision.

## Open questions

- The exact tasks-gate evidence key used to store the normalized task-plan fingerprint. Decision 0028 defines the projection and serialization; Decision 0038 fixes the design gate's two individually fingerprinted inputs, and Decision 0037 fixes completion evidence without upstream artifact fingerprints.
