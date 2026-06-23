# Pulse 259 - Low Countries Crown HRE Parentage

## Scope

Promoted `low-countries-crown-hre-parentage-01`, adding source-backed Holy
Roman Empire parentage for four Low Countries crown-layer titles.

## Accepted Inputs

- `data/staging/low-countries-crown-hre-parentage-01.sources`
- `data/staging/low-countries-crown-hre-parentage-01.facts`

## Result

- Added 4 reviewed text source records.
- Added 4 parentage facts.
- Added 4 matching `composite_crown_component` relation facts.
- Raised accepted sources to 584 and accepted facts to 1609.
- Raised parentage facts to 294 and relation facts to 241.
- Improved title edge fill to 74.29%.
- Improved weighted span coverage to 59.73%.
- Kept rank-skip relation explanations at 232 of 232.
- Preserved 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/low-countries-crown-hre-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-crown-hre-parentage-01.sources data/staging/low-countries-crown-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/low-countries-crown-hre-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-crown-hre-parentage-01.sources data/staging/low-countries-crown-hre-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
