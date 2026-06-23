# Pulse 253: Commonwealth Union Subdivision Relations 01

Date: 2026-06-23

## Scope

Add reviewed text source custody and relation facts for remaining rank-skip rows
where the relation is a clear vassalage, composite component, or administrative
subdivision context.

## Results

- Added six reviewed Wikimedia text source records:
  - Duchy of Courland and Semigallia.
  - Duchy of Livonia.
  - Polish-Lithuanian Commonwealth.
  - Iberian Union.
  - Abauj County.
  - Burgundian Netherlands.
- Added six relation facts:
  - Courland and Livonia as `vassalage_or_suzerainty` contexts under the
    Polish-Lithuanian Commonwealth.
  - Grand Duchy of Lithuania and Kingdom of Portugal as
    `composite_crown_component` contexts under the Commonwealth and Iberian
    Union.
  - Abauj county as a Kingdom of Hungary `subdivision_or_appanage`.
  - Burgundian Netherlands as a Burgundian State
    `composite_crown_component`.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 546.
- Facts: 1555.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 192.
- Rank-skip rows: 231.
- Relation-explained rows: 187.
- Unexplained rank-skip rows: 44.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/commonwealth-union-subdivision-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-union-subdivision-relation-01.sources data/staging/commonwealth-union-subdivision-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/commonwealth-union-subdivision-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/commonwealth-union-subdivision-relation-01.sources data/staging/commonwealth-union-subdivision-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
