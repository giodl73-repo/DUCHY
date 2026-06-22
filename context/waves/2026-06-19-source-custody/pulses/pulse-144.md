# Pulse 144: Benevento Lombards Parentage

## Intent

Continue the post-title-harvest parentage milestone with a directly sourced
Duchy of Benevento relation.

## Changes

- Add `data/staging/benevento-lombards-parentage-01.sources`.
- Add `data/staging/benevento-lombards-parentage-01.facts`.
- Add `data/staging/benevento-lombards-parentage-01-review.md`.
- Promote Duchy of Benevento parentage under the Kingdom of the Lombards for
  `577..774`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet promotes only the duchy's vassalage to the Kingdom of the Lombards
during the accepted duchy span. No Principality of Benevento continuation,
Frankish suzerainty, Byzantine predecessor, rump-state independence, rulers,
territorial expansion, or southern Lombard continuity claims are promoted.

## Current State

- reviewed sources: 428
- accepted facts: 1300
- accepted titles: 346
- parentage facts: 262
- titles with parentage: 217
- titles without parentage: 129
- parentage gap rows: 129

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/benevento-lombards-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/benevento-lombards-parentage-01.sources data/staging/benevento-lombards-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/benevento-lombards-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/benevento-lombards-parentage-01.sources data/staging/benevento-lombards-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
