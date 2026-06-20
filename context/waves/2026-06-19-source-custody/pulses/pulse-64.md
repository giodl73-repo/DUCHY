# Pulse 64: Crown of Aragon Parentage Packet

## Intent

Close another parentage-gap shard 001 row that has a bounded composite crown
parent now that Crown rank is materialized.

## Changes

- Promote reviewed source `src-wikidata-q204920` for Crown of Aragon.
- Promote Crown of Aragon name, Crown rank, and existence facts.
- Promote `title-q1233672` County of Barcelona -> `title-q204920` Crown of
  Aragon parentage for 1162..1164.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Review Boundary

The packet reopens Crown of Aragon only for a reviewed shard-001 parentage
bridge. It does not infer all Crown of Aragon sub-realms or promote broad
claims from the earlier scope-deferred 500-source queue row.

## Current State

- sources: 336
- facts: 1132
- titles: 325
- parentage facts: 157
- titles with parentage: 114
- titles without parentage: 211

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/parentage-gap-001-aragon-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-001-aragon.sources data/staging/parentage-gap-001-aragon.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/parentage-gap-001-aragon-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-001-aragon.sources data/staging/parentage-gap-001-aragon.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
