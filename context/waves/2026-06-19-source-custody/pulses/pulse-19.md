# Pulse 19: Candidate Manifest Queue

## Intent

Support larger historical import planning by tracking candidate sources before
source/fact extraction.

## Changes

- Add `CandidateRecord` and `CandidateStatus`.
- Add dependency-free candidate manifest parsing and validation.
- Reject duplicate candidate IDs and duplicate source IDs.
- Add `duchy-import manifest <file>` status reporting.
- Add synthetic `data/staging/example.manifest`.

## Review Boundary

Manifest rows are a work queue, not accepted history. They identify candidates
for later source-custody review and fact extraction.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- manifest data/staging/example.manifest`
- `cargo fmt --check`
- `git diff --check`
