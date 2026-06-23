# Pulse 235: HRE Relation Batch 06

Date: 2026-06-23

## Scope

Promote a larger batch of source-backed HRE relation contexts where accepted
source notes already support the current parentage row but explicitly exclude
deeper partition, successor, dynastic, or territorial detail.

## Results

- Added `hre-relation-batch-06` with seven relation facts:
  - Holstein, Julich, Lorraine, Mecklenburg-Schwerin, Augsburg, and Basel as
    `imperial_state` relations to the Holy Roman Empire.
  - Mirandola as `vassalage_or_suzerainty` to the Holy Roman Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1424.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 61.
- Rank-skip rows: 231.
- Relation-explained rows: 57.
- Unexplained rank-skip rows: 174.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-06-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-06.sources data/staging/hre-relation-batch-06.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-06-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-06.sources data/staging/hre-relation-batch-06.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
