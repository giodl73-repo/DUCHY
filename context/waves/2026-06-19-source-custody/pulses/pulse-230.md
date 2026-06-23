# Pulse 230: Namur Successor Parentage

Date: 2026-06-23

## Scope

Close the remaining post-Burgundian Namur successor gap with a new bounded
source-custody record and successor parentage spans. Pair each County -> Crown
edge with `composite_crown_component` relation context.

## Results

- Added `namur-successor-parentage-01` with one source record.
- Promoted four parentage facts and four relation facts for County of Namur:
  - Habsburg Netherlands, 1483..1548.
  - Seventeen Provinces, 1549..1580.
  - Spanish Netherlands, 1581..1713.
  - Austrian Netherlands, 1714..1795.
- Regenerated rank-skip, relation, and graph reports.

## Measurements

- Sources: 539.
- Facts: 1410.
- Titles: 355.
- Parentage facts: 289.
- Relation facts: 50.
- Rank-skip rows: 231.
- Relation-explained rows: 47.
- Unexplained rank-skip rows: 184.
- Parentage edges: 292.
- Title edge fill: 72.93%.
- Weighted span coverage: 59.01%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/namur-successor-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/namur-successor-parentage-01.sources data/staging/namur-successor-parentage-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/namur-successor-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/namur-successor-parentage-01.sources data/staging/namur-successor-parentage-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
