# Pulse 36: Batch 001 Title Promotion

## Intent

Begin turning the 500-source candidate queue into accepted history by promoting
only clean title-identity rows from the first 50-row review shard.

## Changes

- Review the `title_identity_only` rows in `candidates-500-shards/batch-001`.
- Fetch structured Wikidata inception and dissolution claims for the reviewed
  title candidates.
- Promote 18 source records and 54 title facts into accepted fixtures.
- Mark those 18 rows as `promoted` in the 500-source candidate queue and
  regenerate queue reports, TSV, duplicate URL report, and shards.
- Leave deferred/unsupported rows in the shard pending.

## Review Boundary

This pulse authorizes title identity, coarse rank, and existence spans only. It
does not import parentage, borders, holders, dynastic continuity, control, or
successor/predecessor relations.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/batch-001-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-001-title.sources data/staging/batch-001-title.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-001-title.sources data/staging/batch-001-title.facts`
- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
