# Pulse 69: Theocratic State Parentage Packet

## Intent

Resolve the first source-backed theocratic-state rank case and use it to close
the Comtat Venaissin parentage gap.

## Changes

- Add `TheocraticState` rank support below empire-level roots for source-backed
  title materialization.
- Promote Papal States (Q170174) as `title-q170174` with 754..1870 existence.
- Promote `title-q1122980` Comtat Venaissin -> `title-q170174` Papal States
  parentage for 1274..1791.
- Preserve the rank decision, relation screen, dry-run report, and apply report
  in staging.
- Regenerate parentage coverage, gap TSV, gap shards, and gap reports.

## Boundary

This packet resolves only the Papal States `theocratic_state` case. It does not
reopen unrelated unsupported free-city, province, administrative-region, or
ecclesiastical-office rows.

## Current State

- sources: 337
- facts: 1150
- titles: 326
- parentage facts: 172
- titles with parentage: 129
- titles without parentage: 197

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/papal-states-comtat-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/papal-states-comtat-parentage-01.sources data/staging/papal-states-comtat-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/papal-states-comtat-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/papal-states-comtat-parentage-01.sources data/staging/papal-states-comtat-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
