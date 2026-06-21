# Pulse 107: Duchy of Modena and Reggio HRE Parentage

## Intent

Close Duchy of Modena and Reggio's parentage gap under the Holy Roman Empire
for the reviewed pre-French-occupation imperial-fief phase.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Modena and Reggio
  parentage context.
- Promote Q252580 -> Q12548 for 1452-1796.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Ferrara papal succession, Este genealogy, Mantuan or Spanish Succession wars,
Mirandola absorption, Austria-Este restoration, Congress of Vienna settlement,
Italian unification, or post-1814 parentage is imported in this packet.

## Current State

- sources: 369
- facts: 1217
- titles: 335
- parentage facts: 212
- titles with parentage: 168
- titles without parentage: 167

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/modena-reggio-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/modena-reggio-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
