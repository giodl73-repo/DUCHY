# Pulse 234: HRE Relation Batch 05

Date: 2026-06-23

## Scope

Promote a small batch of source-backed HRE relation contexts for rank skips
whose direct parentage is already accepted and whose review notes exclude deeper
partition, successor, or dynastic detail.

## Results

- Added `hre-relation-batch-05` with two relation facts:
  - Duchy of Pomerania -> Holy Roman Empire as `vassalage_or_suzerainty` for
    1121..1637.
  - Duchy of Saxony -> Holy Roman Empire as `imperial_state` for 962..1296.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1417.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 54.
- Rank-skip rows: 231.
- Relation-explained rows: 50.
- Unexplained rank-skip rows: 181.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-05-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-05.sources data/staging/hre-relation-batch-05.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-05-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-05.sources data/staging/hre-relation-batch-05.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
