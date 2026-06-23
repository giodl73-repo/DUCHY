# Pulse 226: Low Countries HRE Relations

Date: 2026-06-23

## Scope

Promote conservative current-parent relation facts for Low Countries and
Westphalian county rows that still need deeper intermediate-source review before
any parentage replacement. Keep the packet limited to `imperial_state`
relations to the existing HRE parent.

## Results

- Added `relation-model-low-countries-hre-01` with four relation facts.
- Promoted `imperial_state` relation facts for:
  - County of Holland -> Holy Roman Empire, 962..1432.
  - County of Luxembourg -> Holy Roman Empire, 963..1354.
  - County of Namur -> Holy Roman Empire, 981..1420.
  - County of Ravensberg -> Holy Roman Empire, 1140..1806.
- Regenerated rank-skip, relation, and graph reports.
- Updated WP-007 status and remaining-blocker audit.

## Measurements

- Sources: 532.
- Facts: 1368.
- Parentage facts: 278.
- Relation facts: 37.
- Rank-skip rows: 223.
- Relation-explained rows: 37.
- Unexplained rank-skip rows: 186.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-low-countries-hre-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-low-countries-hre-01.sources data/staging/relation-model-low-countries-hre-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-low-countries-hre-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-low-countries-hre-01.sources data/staging/relation-model-low-countries-hre-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
