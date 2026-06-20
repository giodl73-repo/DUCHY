# Pulse 29: Manifest TSV Import

## Intent

Allow batch tools and spreadsheets to feed candidate queues back into DUCHY
without bypassing manifest validation.

## Changes

- Add `duchy-import manifest-from-tsv <input.tsv> <output.manifest>`.
- Require the fixed TSV header used by `manifest-tsv`.
- Convert rows into manifest records, then parse and validate the generated
  manifest before writing it.
- Reject malformed rows and escaped line breaks that the line-oriented manifest
  format cannot represent safely.

## Review Boundary

Manifest TSV imports create staging manifests only. They do not promote source
records, accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest <temp tsv>`
- `cargo run --bin duchy-import -- manifest-from-tsv <temp tsv> <temp manifest>`
- `cargo run --bin duchy-import -- manifest <temp manifest>`
- `cargo fmt --check`
- `git diff --check`
