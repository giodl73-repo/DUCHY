# Pulse 80: Schleswig Denmark Parentage

## Intent

Promote Denmark as a reviewed parent title and close the Duchy of Schleswig
parentage gap with a reviewed Denmark relation.

## Changes

- Promote Q35 Denmark as a `Kingdom` parent title.
- Promote Q26167 -> Q35 for 1058-1866.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Danish crown constitutional history, Schleswig-Holstein succession disputes,
war transfers, condominium mechanics, border geometry, or post-1866 successor
claim is imported in this packet.

## Current State

- sources: 340
- facts: 1171
- titles: 329
- parentage facts: 184
- titles with parentage: 140
- titles without parentage: 189

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/schleswig-denmark-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/schleswig-denmark-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
