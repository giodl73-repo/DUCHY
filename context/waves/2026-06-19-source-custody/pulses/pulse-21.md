# Pulse 21: Rejected Candidate Audit

## Intent

Preserve rejected import candidates and review notes before active staging queues
are cleaned up.

## Changes

- Add `duchy-import rejected-report <manifest> <output.md>`.
- Emit rejected candidate ID, source ID, source URL, and notes.
- Require at least one rejected candidate so empty reports are not mistaken for
  useful review evidence.

## Review Boundary

Rejected reports are audit artifacts. They do not promote or accept source
records or facts.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- rejected-report data/staging/example.manifest <temp report>`
- `cargo fmt --check`
- `git diff --check`
