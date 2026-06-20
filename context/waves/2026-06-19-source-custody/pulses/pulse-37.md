# Pulse 37: Batch 002 Title Promotion

## Intent

Continue promotion from the 500-source candidate queue by processing the clean
title-identity rows in the second 50-row review shard.

## Changes

- Review the `title_identity_only` rows in `candidates-500-shards/batch-002`.
- Fetch structured Wikidata inception and dissolution claims for the reviewed
  title candidates.
- Promote 26 source records and 78 title facts into accepted fixtures.
- Defer four title candidates whose structured Wikidata claims do not provide a
  complete inception/dissolution span.
- Mark the 26 promoted rows in the 500-source queue and regenerate reports,
  TSV, duplicate URL report, and shards.

## Review Boundary

This pulse authorizes title identity, coarse rank, and existence spans only. It
does not import parentage, borders, holders, dynastic continuity, control, or
successor/predecessor relations.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/batch-002-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-002-title.sources data/staging/batch-002-title.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-002-title.sources data/staging/batch-002-title.facts`
- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
