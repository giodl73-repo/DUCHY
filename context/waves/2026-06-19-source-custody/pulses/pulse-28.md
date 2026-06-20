# Pulse 28: Manifest TSV Export

## Intent

Give batch tooling a machine-readable candidate queue export without parsing
Markdown review reports.

## Changes

- Add `duchy-import manifest-tsv <manifest> <output.tsv>`.
- Validate the source manifest before export.
- Write fixed columns: `candidate_id`, `source_id`, `source_url`, `status`, and
  `notes`.
- Escape tabs, line breaks, carriage returns, and backslashes inside cells.

## Review Boundary

Manifest TSV exports are staging artifacts. They do not promote source records,
accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest <temp tsv>`
- inspect generated TSV
- `cargo fmt --check`
- `git diff --check`
