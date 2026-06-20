# Pulse 79: Estonia Sweden Parentage

## Intent

Promote Sweden as a reviewed parent title and close the Duchy of Estonia
parentage gap with a reviewed Sweden relation.

## Changes

- Promote Q34 Sweden as a `Kingdom` parent title.
- Promote Q16835712 -> Q34 for 1561-1721.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Swedish crown constitutional history, Estonian provincial administration,
war transfers, border geometry, or post-1721 Russian Empire successor claim is
imported in this packet.

## Current State

- sources: 339
- facts: 1167
- titles: 328
- parentage facts: 183
- titles with parentage: 139
- titles without parentage: 189

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/estonia-sweden-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/estonia-sweden-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
