# Pulse 67: Commonwealth Children Parentage Packet

## Intent

Use the corrected Polish-Lithuanian Commonwealth rank to close two accepted
child-title gaps under the Commonwealth.

## Changes

- Promote Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth for
  1569..1795.
- Promote Grand Duchy of Lithuania -> Polish-Lithuanian Commonwealth for
  1569..1795.
- Preserve reviewed relation rows in
  `data/staging/commonwealth-children-parentage-01-relations.tsv`.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Boundary

The one-year Kingdom of Poland (Q8890160) relation at 1569 remains deferred.
This packet imports only multi-year parentage spans.

## Current State

- sources: 336
- facts: 1145
- titles: 325
- parentage facts: 170
- titles with parentage: 127
- titles without parentage: 198

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/commonwealth-children-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-children-parentage-01.sources data/staging/commonwealth-children-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/commonwealth-children-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-children-parentage-01.sources data/staging/commonwealth-children-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
