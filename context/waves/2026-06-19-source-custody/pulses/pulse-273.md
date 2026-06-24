# Pulse 273 - Residual Crown of Aragon Component Relations

## Scope

Promoted `aragon-component-relation-01`, adding composite-crown component
relation context for the five remaining accepted Crown of Aragon parentage
children that lacked it: Kingdom of Aragon, Kingdom of Valencia, Kingdom of
Majorca, Kingdom of Sardinia, and the County of Empuries. Together with the
already-promoted Barcelona, Catalonia, Sicily, and Naples relations, every
accepted Crown of Aragon component now carries matching relation context.

## Accepted Inputs

- `data/staging/aragon-component-relation-01.sources`
- `data/staging/aragon-component-relation-01.facts`

## Result

- Added 5 `composite_crown_component` relation facts.
- Added no new source records; this packet uses already accepted source
  custody.
- Kept reviewed sources at 588.
- Raised accepted facts to 1636.
- Raised relation facts to 268.
- Kept parentage facts at 294.
- Kept title edge fill at 74.29%.
- Kept weighted span coverage at 59.73%.
- Kept rank-skip relation explanations at 232 of 232.
- Preserved 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/aragon-component-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/aragon-component-relation-01.sources data/staging/aragon-component-relation-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/aragon-component-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/aragon-component-relation-01.sources data/staging/aragon-component-relation-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`
