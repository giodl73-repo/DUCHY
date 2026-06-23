# Pulse 228: Low Countries Successor Titles

Date: 2026-06-23

## Scope

Materialize title identities for the accepted Low Countries successor-state
source queue. Keep this pass to title nodes only so later parentage or relation
packets can connect counties to reviewed successor titles without guessing IDs.

## Results

- Added `low-countries-successor-titles-01` with six Wikidata source records.
- Promoted Crown-rank title identities for:
  - Habsburg Netherlands, 1482..1797.
  - Seventeen Provinces, 1549..1581.
  - Spanish Netherlands, 1581..1714.
  - Austrian Netherlands, 1714..1797.
  - Dutch Republic, 1581..1795.
  - Batavian Republic, 1795..1806.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage and relation facts unchanged.

## Measurements

- Sources: 538.
- Facts: 1388.
- Titles: 355.
- Parentage facts: 278.
- Relation facts: 39.
- Rank-skip rows: 223.
- Relation-explained rows: 39.
- Unexplained rank-skip rows: 184.
- Parentage edges: 284.
- Title edge fill: 72.93%.
- Weighted span coverage: 59.01%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/low-countries-successor-titles-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-titles-01.sources data/staging/low-countries-successor-titles-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/low-countries-successor-titles-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-titles-01.sources data/staging/low-countries-successor-titles-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
