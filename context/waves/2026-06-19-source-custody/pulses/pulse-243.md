# Pulse 243: German State Relation Batch 03

Date: 2026-06-23

## Scope

Promote a third German state relation packet from accepted Saxe,
Schaumburg-Lippe, and Schwarzburg source-custody rows. Limit the packet to
active current-parent rows already present in the parentage report.

## Results

- Added `german-state-relation-batch-03` with twelve relation facts:
  - Saxe-Coburg and Gotha as `confederation_member` for the German
    Confederation.
  - Saxe-Coburg-Saalfeld, Saxe-Gotha-Altenburg, Saxe-Hildburghausen,
    Schaumburg-Lippe, and Schwarzburg-Sondershausen as `imperial_state` for
    the Holy Roman Empire and `confederation_member` for the German
    Confederation.
  - Schwarzburg-Rudolstadt as `confederation_member` for the German
    Confederation.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1499.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 136.
- Rank-skip rows: 231.
- Relation-explained rows: 132.
- Unexplained rank-skip rows: 99.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-state-relation-batch-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-03.sources data/staging/german-state-relation-batch-03.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-state-relation-batch-03-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-03.sources data/staging/german-state-relation-batch-03.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
