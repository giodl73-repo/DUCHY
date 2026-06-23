# Pulse 241: German State Relation Batch 01

Date: 2026-06-23

## Scope

Promote text-backed German state membership relation contexts for Hohenzollern
principalities, Electorate of Hesse, and Oldenburg. Exclude structured-only
bridge imports from this packet.

## Results

- Added `german-state-relation-batch-01` with eight relation facts:
  - Hohenzollern-Hechingen and Hohenzollern-Sigmaringen as `imperial_state`
    relations to the Holy Roman Empire and `confederation_member` relations to
    the German Confederation.
  - Electorate of Hesse as a `confederation_member` relation to the German
    Confederation.
  - Grand Duchy of Oldenburg as `confederation_member` relations to the German
    Confederation and North German Confederation, plus `federal_state_member`
    to the German Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1475.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 112.
- Rank-skip rows: 231.
- Relation-explained rows: 108.
- Unexplained rank-skip rows: 123.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-state-relation-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-01.sources data/staging/german-state-relation-batch-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-state-relation-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-01.sources data/staging/german-state-relation-batch-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
