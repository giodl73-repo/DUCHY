# Pulse 147: Britain Union Parentage

## Intent

Start the next medium-priority parentage pass with a direct 1707 union endpoint
for accepted England, Scotland, and Great Britain titles.

## Changes

- Add `data/staging/britain-union-parentage-01.sources`.
- Add `data/staging/britain-union-parentage-01.facts`.
- Add `data/staging/britain-union-parentage-01-review.md`.
- Add `data/staging/rank-correction-great-britain-review.md`.
- Correct Kingdom of Great Britain from `Kingdom` to DUCHY's composite `Crown`
  rank.
- Promote Kingdom of England and Kingdom of Scotland endpoint parentage under
  the Kingdom of Great Britain for `1707..1707`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the 1707 successor endpoint relation. No parliamentary
mechanics, Treaty of Union articles, monarchy, Ireland, Wales, empire, Scottish
law/church settlement, representation, or post-1707 British state history is
promoted.

## Current State

- reviewed sources: 431
- accepted facts: 1304
- accepted titles: 346
- parentage facts: 266
- titles with parentage: 221
- titles without parentage: 125
- parentage gap rows: 125

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/britain-union-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/britain-union-parentage-01.sources data/staging/britain-union-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/britain-union-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/britain-union-parentage-01.sources data/staging/britain-union-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
