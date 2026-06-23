# Pulse 236: HRE Relation Batch 07

Date: 2026-06-23

## Scope

Promote another source-backed HRE relation batch from accepted text notes that
already support the current parentage rows. Keep the packet relation-only and
avoid rows whose wording does not map cleanly to an existing relation kind.

## Results

- Added `hre-relation-batch-07` with nine relation facts:
  - Carniola, Luxembourg, Cleves, Freising, Liege, Minden, and Munster as
    `imperial_state` relations to the Holy Roman Empire.
  - Modena and Reggio plus Parma and Piacenza as
    `vassalage_or_suzerainty` relations to the Holy Roman Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1433.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 70.
- Rank-skip rows: 231.
- Relation-explained rows: 66.
- Unexplained rank-skip rows: 165.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-07-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-07.sources data/staging/hre-relation-batch-07.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-07-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-07.sources data/staging/hre-relation-batch-07.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
