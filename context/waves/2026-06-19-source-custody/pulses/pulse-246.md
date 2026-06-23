# Pulse 246: HRE Residual Relation Batch 11

Date: 2026-06-23

## Scope

Promote three residual relation contexts whose subject, parent, span, and text
source custody already exist in accepted fixtures.

## Results

- Added `hre-relation-batch-11` with three relation facts:
  - Holstein as a `confederation_member` relation to the German Confederation
    for 1815..1866.
  - Saxe-Lauenburg as a `confederation_member` relation to the German
    Confederation for 1815..1866.
  - Swabia as an `imperial_state` relation to the Holy Roman Empire for
    962..1313.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1519.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 156.
- Rank-skip rows: 231.
- Relation-explained rows: 151.
- Unexplained rank-skip rows: 80.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-11-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-11.sources data/staging/hre-relation-batch-11.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-11-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-11.sources data/staging/hre-relation-batch-11.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
