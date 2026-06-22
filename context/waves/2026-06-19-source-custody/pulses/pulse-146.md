# Pulse 146: Girona Carolingian Parentage

## Intent

Close the remaining post-title-harvest high-priority county parentage gap with a
bounded Carolingian Empire relation for the County of Girona.

## Changes

- Add `data/staging/girona-carolingian-parentage-01.sources`.
- Add `data/staging/girona-carolingian-parentage-01.facts`.
- Add `data/staging/girona-carolingian-parentage-01-review.md`.
- Promote County of Girona parentage under the Carolingian Empire for
  `800..887`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet promotes only the Carolingian Empire overlap span. No pre-800
Frankish kingdom parentage, Barcelona same-rank incorporation, later Catalan
autonomy, count-holder sequence, Spanish March territorial extent,
ecclesiastical jurisdiction, aprisio grants, or sovereignty claims are promoted.

## Current State

- reviewed sources: 430
- accepted facts: 1302
- accepted titles: 346
- parentage facts: 264
- titles with parentage: 219
- titles without parentage: 127
- parentage gap rows: 127

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/girona-carolingian-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/girona-carolingian-parentage-01.sources data/staging/girona-carolingian-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/girona-carolingian-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/girona-carolingian-parentage-01.sources data/staging/girona-carolingian-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
