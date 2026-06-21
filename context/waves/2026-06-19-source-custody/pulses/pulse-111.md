# Pulse 111: Kingdom of Thessalonica Latin Empire Parentage

## Intent

Close Kingdom of Thessalonica's parentage gap under the Latin Empire for its
reviewed short-lived crusader kingdom span.

## Changes

- Add one reviewed Wikimedia text source for Kingdom of Thessalonica parentage
  context.
- Promote Q325461 -> Q178897 for 1204-1224.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Fourth Crusade narrative, Byzantine predecessor, Boniface succession,
Lombard rebellion, Epirus conquest detail, titular claimants, or post-1224
successor claims are imported in this packet.

## Current State

- sources: 373
- facts: 1221
- titles: 335
- parentage facts: 216
- titles with parentage: 172
- titles without parentage: 163

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/thessalonica-latin-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/thessalonica-latin-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
