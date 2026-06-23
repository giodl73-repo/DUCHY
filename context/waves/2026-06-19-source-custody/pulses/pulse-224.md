# Pulse 224: Barcelona Catalonia Relation

Date: 2026-06-23

## Scope

Promote a narrow Barcelona/Catalonia relation packet from the reviewed county
rank-skip queue. Explain the accepted County of Barcelona -> Crown of Aragon
1162..1164 rank skip without importing a Principality of Catalonia title or
changing parentage.

## Results

- Added `barcelona-catalonia-relation-01` with three source records and one
  relation fact.
- Promoted `fact-q1233672-relation-q204920-composite-crown-component`.
- Regenerated rank-skip relation and parentage graph reports.
- Updated WP-007 status and remaining-blocker audit.

## Measurements

- Sources: 532.
- Facts: 1362.
- Parentage facts: 278.
- Relation facts: 32.
- Rank-skip rows: 223.
- Relation-explained rows: 32.
- Unexplained rank-skip rows: 191.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/barcelona-catalonia-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/barcelona-catalonia-relation-01.sources data/staging/barcelona-catalonia-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/barcelona-catalonia-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/barcelona-catalonia-relation-01.sources data/staging/barcelona-catalonia-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
