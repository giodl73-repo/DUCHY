# Pulse 251: Endpoint Transition Relations 01

Date: 2026-06-23

## Scope

Promote endpoint transition contexts for remaining rank-skip rows whose
accepted text sources identify a successor, replacement, direct rule, or
conquest/fall at the current parent endpoint.

## Results

- Added `endpoint-transition-relation-batch-01` with four `rank_transition`
  relation facts:
  - County of Sicily to Kingdom of Sicily at 1130.
  - Polish-Lithuanian union to Polish-Lithuanian Commonwealth at 1569.
  - Duchy of Ferrara to direct Papal States rule at 1597.
  - Kingdom of Bosnia to Ottoman conquest/fall at 1463.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1546.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 183.
- Rank-skip rows: 231.
- Relation-explained rows: 178.
- Unexplained rank-skip rows: 53.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/endpoint-transition-relation-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/endpoint-transition-relation-batch-01.sources data/staging/endpoint-transition-relation-batch-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/endpoint-transition-relation-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/endpoint-transition-relation-batch-01.sources data/staging/endpoint-transition-relation-batch-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
