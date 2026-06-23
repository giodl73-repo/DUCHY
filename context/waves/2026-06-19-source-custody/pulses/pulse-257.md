# Pulse 257 - Residual Rank-Skip Relation Clearance

## Scope

Promoted `residual-rank-skip-relation-01`, the final relation-only packet for
the active rank-skip explanation queue.

## Accepted Inputs

- `data/staging/residual-rank-skip-relation-01.sources`
- `data/staging/residual-rank-skip-relation-01.facts`

## Result

- Added 14 reviewed text source records.
- Added 17 bounded relation facts.
- Raised accepted sources to 579 and accepted facts to 1599.
- Raised relation facts to 236.
- Raised relation-explained rank-skip rows to 231 of 231.
- Reduced unexplained rank-skip rows to 0.
- Preserved 289 parentage facts and 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/residual-rank-skip-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/residual-rank-skip-relation-01.sources data/staging/residual-rank-skip-relation-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/residual-rank-skip-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/residual-rank-skip-relation-01.sources data/staging/residual-rank-skip-relation-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
