# Pulse 240: HRE Residual Relation Batch 09

Date: 2026-06-23

## Scope

Promote a compact residual relation packet from accepted current-parent rows
that already have text-backed wording for HRE, German Confederation, North
German Confederation, or German Empire context. Keep parentage unchanged.

## Results

- Added `hre-relation-batch-09` with ten relation facts:
  - Grand Duchy of Tuscany and Duchy of Florence as
    `vassalage_or_suzerainty` relations to the Holy Roman Empire.
  - Prince-Bishopric of Toul, Principality of Lippe,
    Prince-Archbishopric of Salzburg, and Saxe-Meiningen as `imperial_state`
    relations to the Holy Roman Empire.
  - Principality of Lippe, Duchy of Nassau, and Saxe-Meiningen as
    `confederation_member` relations to the German Confederation.
  - Saxe-Meiningen as a `federal_state_member` relation to the German Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1467.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 104.
- Rank-skip rows: 231.
- Relation-explained rows: 100.
- Unexplained rank-skip rows: 131.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-09-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-09.sources data/staging/hre-relation-batch-09.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-09-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-09.sources data/staging/hre-relation-batch-09.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
