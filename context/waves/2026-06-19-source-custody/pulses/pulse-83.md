# Pulse 83: Kalmar Union Sweden Parentage

## Intent

Correct Kalmar Union to a reviewed composite parent rank and close the Sweden
parentage gap with a reviewed union-membership relation.

## Changes

- Correct Q62623 Kalmar Union from `Kingdom` to DUCHY's composite `Crown` rank.
- Promote Q34 -> Q62623 for 1397-1523.
- Preserve the empty source packet, relation screen, dry-run report, and apply
  report in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Denmark or Norway union parentage, Danish royal holders, union-break
mechanics, Scandinavian borders, or post-union successor claims are imported in
this packet.

## Current State

- sources: 342
- facts: 1181
- titles: 331
- parentage facts: 188
- titles with parentage: 144
- titles without parentage: 187

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/kalmar-union-sweden-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/kalmar-union-sweden-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
