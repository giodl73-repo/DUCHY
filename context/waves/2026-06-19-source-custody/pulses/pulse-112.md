# Pulse 112: Duchy of Cleves HRE Parentage

## Intent

Close Duchy of Cleves' parentage gap under the Holy Roman Empire for its
reviewed polity span.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Cleves parentage context.
- Promote Q641138 -> Q12548 for 1092-1795.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Lower Lotharingia predecessor detail, counts or dukes genealogy, union with
Mark, elevation to duchy detail, United Duchies of Julich-Cleves-Berg
continuation, Brandenburg succession, Grand Duchy of Berg transfer, Prussian
province, or post-1795 settlement is imported in this packet.

## Current State

- sources: 374
- facts: 1222
- titles: 335
- parentage facts: 217
- titles with parentage: 173
- titles without parentage: 162

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/cleves-duchy-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/cleves-duchy-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
