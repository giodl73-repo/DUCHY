# Pulse 81: Burgundian Netherlands State Parentage

## Intent

Promote Burgundian State as a reviewed parent title and close the Burgundian
Netherlands parentage gap with a reviewed relation.

## Changes

- Promote Q7882199 Burgundian State as a `Crown` parent title.
- Promote Q157109 -> Q7882199 for 1384-1482.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Burgundian inheritance mechanics, Low Countries component-title inventory,
Habsburg succession, administrative institutions, or border geometry is imported
in this packet.

## Current State

- sources: 341
- facts: 1175
- titles: 330
- parentage facts: 185
- titles with parentage: 141
- titles without parentage: 189

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/burgundian-netherlands-state-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/burgundian-netherlands-state-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
