# Pulse 222: Low Countries Split-Control Relations

Date: 2026-06-23

## Scope

Promote the first fact-bearing packet from the Low Countries successor source
custody pass. Keep the packet limited to relations that explain current-parent
rank skips without replacing parentage or introducing new successor-state
titles.

## Results

- Added `relation-model-low-countries-01` with three relation facts.
- Promoted bounded `split_fief_or_control` relations for:
  - County of Flanders -> Holy Roman Empire, 1483..1797.
  - County of Holland -> Holy Roman Empire, 1483..1795.
  - County of Namur -> Holy Roman Empire, 1483..1795.
- Regenerated the rank-skip relation and parentage graph reports.
- Updated WP-007 status and remaining-blocker audit.

## Measurements

- Sources: 526.
- Facts: 1361.
- Parentage facts: 278.
- Relation facts: 31.
- Rank-skip rows: 223.
- Relation-explained rows: 31.
- Unexplained rank-skip rows: 192.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-low-countries-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-low-countries-01.sources data/staging/relation-model-low-countries-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-low-countries-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-low-countries-01.sources data/staging/relation-model-low-countries-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
