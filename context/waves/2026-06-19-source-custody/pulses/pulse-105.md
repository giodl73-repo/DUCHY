# Pulse 105: Margraviate of Austria HRE Parentage

## Intent

Close Margraviate of Austria's parentage gap under the Holy Roman Empire for
the reviewed frontier-march span.

## Changes

- Add one reviewed Wikimedia text source for Margraviate of Austria parentage
  context.
- Promote Q283627 -> Q12548 for 976-1156.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Frankish frontier structures, Bavarian ducal subordination, Babenberg
genealogy, Hungarian border administration, Duchy of Austria continuation,
Privilegium Minus detail, March of Pannonia, Hungarian March, or later Austrian
state claims are imported in this packet.

## Current State

- sources: 367
- facts: 1215
- titles: 335
- parentage facts: 210
- titles with parentage: 166
- titles without parentage: 169

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/austria-margraviate-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/austria-margraviate-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
