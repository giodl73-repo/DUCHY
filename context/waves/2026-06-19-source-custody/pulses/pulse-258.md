# Pulse 258 - Catalonia Crown Of Aragon Parentage

## Scope

Promoted `catalonia-crown-aragon-parentage-01`, a source-backed parentage and
relation packet for Principality of Catalonia under the Crown of Aragon.

## Accepted Inputs

- `data/staging/catalonia-crown-aragon-parentage-01.sources`
- `data/staging/catalonia-crown-aragon-parentage-01.facts`

## Result

- Added 1 reviewed text source record.
- Added 1 parentage fact.
- Added 1 matching `composite_crown_component` relation fact.
- Raised accepted sources to 580 and accepted facts to 1601.
- Raised parentage facts to 290 and relation facts to 237.
- Improved title edge fill to 73.02%.
- Improved weighted span coverage to 59.21%.
- Kept rank-skip relation explanations at 232 of 232.
- Preserved 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/catalonia-crown-aragon-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/catalonia-crown-aragon-parentage-01.sources data/staging/catalonia-crown-aragon-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/catalonia-crown-aragon-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/catalonia-crown-aragon-parentage-01.sources data/staging/catalonia-crown-aragon-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
