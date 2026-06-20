# Pulse 66: Commonwealth Rank Correction and Livonia Parentage

## Intent

Fix a pre-Crown rank artifact and use the corrected composite rank to close a
reviewed Duchy of Livonia parentage gap.

## Changes

- Correct `fact-q172107-rank` for Polish-Lithuanian Commonwealth from `Duchy`
  to `Crown`.
- Promote `title-q1352878` Duchy of Livonia -> `title-q172107`
  Polish-Lithuanian Commonwealth parentage for 1569..1621.
- Preserve the rank correction review and relation review artifacts in staging.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Current State

- sources: 336
- facts: 1143
- titles: 325
- parentage facts: 168
- titles with parentage: 125
- titles without parentage: 200

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/livonia-commonwealth-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/livonia-commonwealth-parentage-01.sources data/staging/livonia-commonwealth-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/livonia-commonwealth-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/livonia-commonwealth-parentage-01.sources data/staging/livonia-commonwealth-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
