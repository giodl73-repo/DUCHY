# Pulse 256 - Austrian Monarchy Component Relations

## Scope

Promoted `austrian-monarchy-component-relation-01`, a relation-only packet for
remaining Austrian Empire and Austria-Hungary rank-skip explanations.

## Accepted Inputs

- `data/staging/austrian-monarchy-component-relation-01.sources`
- `data/staging/austrian-monarchy-component-relation-01.facts`

## Result

- Added 10 reviewed text source records.
- Added 16 bounded relation facts.
- Raised accepted sources to 565 and accepted facts to 1582.
- Raised relation facts to 219.
- Raised relation-explained rank-skip rows to 214 of 231.
- Reduced unexplained rank-skip rows to 17.
- Preserved 289 parentage facts and 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/austrian-monarchy-component-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/austrian-monarchy-component-relation-01.sources data/staging/austrian-monarchy-component-relation-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/austrian-monarchy-component-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/austrian-monarchy-component-relation-01.sources data/staging/austrian-monarchy-component-relation-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
