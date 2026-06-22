# Pulse 152: Serbia Empire Parentage

## Intent

Promote the bounded Serbian Empire title and import the medieval Kingdom of
Serbia endpoint transition into it for 1346.

## Changes

- Add `data/staging/serbia-empire-parentage-01.sources`.
- Add `data/staging/serbia-empire-parentage-01.facts`.
- Add `data/staging/serbia-empire-parentage-01-review.md`.
- Promote Serbian Empire as an `Empire` title for `1346..1371`.
- Promote Kingdom of Serbia endpoint parentage under the Serbian Empire for
  `1346..1346`.
- Refresh parentage coverage, parentage gap queue/shards, shard reports, and
  parentage change metrics.

## Boundary

This packet imports only the Serbian Empire title identity and the 1346 endpoint
transition from the medieval Kingdom of Serbia. No territorial extent, rulers,
Dusan's Code, church hierarchy, imperial ideology, collapse process, successor
principalities, Ottoman relation, or later national claims are promoted.

## Current State

- reviewed sources: 436
- accepted facts: 1316
- accepted titles: 348
- parentage facts: 272
- titles with parentage: 227
- titles without parentage: 121
- parentage gap rows: 121

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/serbia-empire-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/serbia-empire-parentage-01.sources data/staging/serbia-empire-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/serbia-empire-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/serbia-empire-parentage-01.sources data/staging/serbia-empire-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-blockers.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
