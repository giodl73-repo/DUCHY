# Pulse 249: Napoleonic Client Relations 01

Date: 2026-06-23

## Scope

Promote bounded relation contexts for remaining Napoleonic and German client or
confederation rank-skip rows whose accepted sources already support the current
parent.

## Results

- Added `napoleonic-client-relation-batch-01` with seven relation facts:
  - Grand Duchy of Berg, Grand Duchy of Frankfurt, and Grand Duchy of Wurzburg
    as `confederation_member` contexts under the Confederation of the Rhine.
  - Kingdom of Holland, Kingdom of Etruria, and Napoleonic Kingdom of Italy as
    `vassalage_or_suzerainty` contexts under the First French Empire.
  - The 1918 Duchy of Courland and Semigallia as a
    `vassalage_or_suzerainty` context under the German Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1538.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 175.
- Rank-skip rows: 231.
- Relation-explained rows: 170.
- Unexplained rank-skip rows: 61.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/napoleonic-client-relation-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/napoleonic-client-relation-batch-01.sources data/staging/napoleonic-client-relation-batch-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/napoleonic-client-relation-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/napoleonic-client-relation-batch-01.sources data/staging/napoleonic-client-relation-batch-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
