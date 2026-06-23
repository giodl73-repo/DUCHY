# Pulse 255: Administrative Dependency Relations 01

Date: 2026-06-23

## Scope

Add source-backed administrative and dependency relation contexts for remaining
rank-skip rows that are already accepted as parentage but lack relation
explanations.

## Results

- Added five reviewed Wikimedia text source records:
  - Cetatea-Alba County.
  - Dorohoi County.
  - Grand Duchy of Finland.
  - Illyrian Provinces.
  - Kingdom of the Burgundians.
- Added six relation facts:
  - Cetatea-Alba County and Dorohoi County as Kingdom of Romania
    `subdivision_or_appanage` contexts.
  - County of Nice as a Kingdom of Sardinia `subdivision_or_appanage` context.
  - Grand Duchy of Finland as a Russian Empire `subdivision_or_appanage`
    context.
  - Illyrian Provinces as a First French Empire `subdivision_or_appanage`
    context.
  - Kingdom of the Burgundians as a Western Roman Empire
    `vassalage_or_suzerainty` context.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 555.
- Facts: 1566.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 203.
- Rank-skip rows: 231.
- Relation-explained rows: 198.
- Unexplained rank-skip rows: 33.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/administrative-dependency-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/administrative-dependency-relation-01.sources data/staging/administrative-dependency-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/administrative-dependency-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/administrative-dependency-relation-01.sources data/staging/administrative-dependency-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
