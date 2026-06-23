# Pulse 233: Duchy of Savoy HRE Relation

Date: 2026-06-23

## Scope

Promote a bounded source-backed relation for Duchy of Savoy's direct Holy Roman
Empire context. Keep this relation-only because the accepted source explicitly
excludes County of Savoy predecessor detail, French occupations, Sardinian
transition, and territorial inventory.

## Results

- Added `duchy-savoy-hre-relation-01` with one `imperial_state` relation from
  Duchy of Savoy to the Holy Roman Empire for 1416..1806.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1415.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 52.
- Rank-skip rows: 231.
- Relation-explained rows: 48.
- Unexplained rank-skip rows: 183.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/duchy-savoy-hre-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/duchy-savoy-hre-relation-01.sources data/staging/duchy-savoy-hre-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/duchy-savoy-hre-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/duchy-savoy-hre-relation-01.sources data/staging/duchy-savoy-hre-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
