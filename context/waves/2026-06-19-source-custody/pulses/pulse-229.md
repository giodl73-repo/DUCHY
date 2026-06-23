# Pulse 229: Low Countries Successor Parentage

Date: 2026-06-23

## Scope

Connect the first reviewed Low Countries successor title nodes to bounded
county spans for Flanders and Holland. Pair each County -> Crown parentage edge
with a matching `composite_crown_component` relation so rank-skip reports can
distinguish modeled composite membership from a missing duchy layer.

## Results

- Added `low-countries-successor-parentage-01` with seven parentage facts and
  seven relation facts.
- Replaced late HRE parentage spans for:
  - County of Flanders -> Habsburg Netherlands, 1483..1548.
  - County of Flanders -> Seventeen Provinces, 1549..1580.
  - County of Flanders -> Spanish Netherlands, 1581..1713.
  - County of Flanders -> Austrian Netherlands, 1714..1797.
  - County of Holland -> Habsburg Netherlands, 1483..1548.
  - County of Holland -> Seventeen Provinces, 1549..1580.
  - County of Holland -> Dutch Republic, 1581..1795.
- Regenerated rank-skip, relation, and graph reports.

## Measurements

- Sources: 538.
- Facts: 1402.
- Titles: 355.
- Parentage facts: 285.
- Relation facts: 46.
- Rank-skip rows: 228.
- Relation-explained rows: 44.
- Unexplained rank-skip rows: 184.
- Parentage edges: 289.
- Title edge fill: 72.93%.
- Weighted span coverage: 59.01%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/low-countries-successor-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-parentage-01.sources data/staging/low-countries-successor-parentage-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/low-countries-successor-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-parentage-01.sources data/staging/low-countries-successor-parentage-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
