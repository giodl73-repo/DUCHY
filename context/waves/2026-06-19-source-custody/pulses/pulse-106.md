# Pulse 106: Grand Duchy of Berg Rhine Confederation Parentage

## Intent

Close Grand Duchy of Berg's parentage gap under the Confederation of the Rhine
for its reviewed Napoleonic grand-duchy span.

## Changes

- Add one reviewed Wikimedia text source for Grand Duchy of Berg parentage
  context.
- Promote Q249428 -> Q154741 for 1806-1813.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No earlier County or Duchy of Berg lineage, Jülich-Cleves-Berg union,
Wittelsbach rule, French annexation detail, rulers, Napoleonic administration
detail, or post-1813 Prussian settlement is imported in this packet.

## Current State

- sources: 368
- facts: 1216
- titles: 335
- parentage facts: 211
- titles with parentage: 167
- titles without parentage: 168

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/berg-grand-duchy-rhine-confederation-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/berg-grand-duchy-rhine-confederation-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
