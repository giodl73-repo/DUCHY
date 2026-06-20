# Pulse 63: Composite Crown Parentage Packet

## Intent

Close the first parentage-gap shard row that needs a composite crown parent
without misclassifying the parent as an empire.

## Changes

- Add `TitleRank::Crown` between kingdom and empire.
- Parse `Crown`, `composite crown`, and `composite realm` rank facts.
- Include Crown rows in parentage gap labels, priorities, and notes.
- Promote reviewed source `src-wikidata-q217196` for Crown of Castile.
- Promote Crown of Castile name, rank, and existence facts.
- Promote `title-q1164500` Kingdom of Murcia -> `title-q217196` Crown of
  Castile parentage for 1258..1715.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Review Boundary

The packet reopens Crown of Castile only for a reviewed shard-001 parentage
bridge. It does not infer all Crown of Castile sub-realms or promote broad
claims from the earlier scope-deferred 500-source queue row.

## Current State

- sources: 335
- facts: 1128
- titles: 324
- parentage facts: 156
- titles with parentage: 113
- titles without parentage: 211

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/parentage-gap-001-castile-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-001-castile.sources data/staging/parentage-gap-001-castile.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/parentage-gap-001-castile-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-001-castile.sources data/staging/parentage-gap-001-castile.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
