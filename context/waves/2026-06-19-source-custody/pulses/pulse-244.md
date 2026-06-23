# Pulse 244: HRE Residual Relation Batch 10

Date: 2026-06-23

## Scope

Promote residual HRE relation contexts from accepted text-backed rows. Exclude
structured-only rows and broader Austrian internal-monarchy rows whose current
source custody does not support title-specific relation facts.

## Results

- Added `hre-relation-batch-10` with eight `imperial_state` relation facts:
  - Kingdom of Bohemia.
  - Free Imperial City of Aachen.
  - Hesse-Marburg.
  - Lower Lotharingia.
  - Mecklenburg-Gustrow.
  - Nassau-Siegen in 1303..1328 and 1606..1743.
  - Palatinate-Sulzbach.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1507.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 144.
- Rank-skip rows: 231.
- Relation-explained rows: 139.
- Unexplained rank-skip rows: 92.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-10-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-10.sources data/staging/hre-relation-batch-10.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-10-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-10.sources data/staging/hre-relation-batch-10.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
