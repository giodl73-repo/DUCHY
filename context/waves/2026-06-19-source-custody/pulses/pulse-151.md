# Pulse 151: Kalmar Denmark Norway Parentage

## Intent

Complete the currently modeled Kalmar Union child set by adding Denmark and
medieval Norway parentage under the already accepted Kalmar Union title.

## Changes

- Add `data/staging/kalmar-union-denmark-norway-parentage-01.sources`.
- Add `data/staging/kalmar-union-denmark-norway-parentage-01.facts`.
- Add `data/staging/kalmar-union-denmark-norway-parentage-01-review.md`.
- Promote Denmark parentage under Kalmar Union for `1397..1523`.
- Promote medieval Norway endpoint parentage under Kalmar Union for
  `1397..1397`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the Denmark and medieval Norway parentage bridge facts.
No Danish royal holders, Swedish union-break mechanics, Norwegian succession,
Scandinavian borders, Denmark-Norway successor claims, post-1523 history, or
modern Denmark/Norway continuity claims are promoted.

## Current State

- reviewed sources: 434
- accepted facts: 1312
- accepted titles: 347
- parentage facts: 271
- titles with parentage: 226
- titles without parentage: 121
- parentage gap rows: 121

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/kalmar-union-denmark-norway-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/kalmar-union-denmark-norway-parentage-01.sources data/staging/kalmar-union-denmark-norway-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/kalmar-union-denmark-norway-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/kalmar-union-denmark-norway-parentage-01.sources data/staging/kalmar-union-denmark-norway-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
