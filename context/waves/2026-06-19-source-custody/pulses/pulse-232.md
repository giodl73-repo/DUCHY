# Pulse 232: Welf Relation Context

Date: 2026-06-23

## Scope

Allow source-backed relation facts to carry multiple simultaneous contexts for
the same title and span. Use that model to add Brunswick-Wolfenbuttel's Welf
subdivision context without replacing its direct Holy Roman Empire parentage or
imperial-state relation.

## Results

- Updated fact validation so accepted `relation` facts are multi-valued context
  claims rather than single-valued title attributes.
- Added `brunswick-wolfenbuttel-welf-relation-01` with one
  `subdivision_or_appanage` relation to Duchy of Brunswick-Luneburg.
- Kept Brunswick-Wolfenbuttel parentage under the Holy Roman Empire because the
  current parentage rank policy rejects Duchy-to-Duchy parent edges.
- Regenerated rank-skip, relation, and graph reports.

## Measurements

- Sources: 540.
- Facts: 1414.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 51.
- Rank-skip rows: 231.
- Relation-explained rows: 47.
- Unexplained rank-skip rows: 184.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/brunswick-wolfenbuttel-welf-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/brunswick-wolfenbuttel-welf-relation-01.sources data/staging/brunswick-wolfenbuttel-welf-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/brunswick-wolfenbuttel-welf-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/brunswick-wolfenbuttel-welf-relation-01.sources data/staging/brunswick-wolfenbuttel-welf-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
