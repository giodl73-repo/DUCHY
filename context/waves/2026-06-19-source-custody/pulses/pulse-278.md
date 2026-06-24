# Pulse 278 - Bohemian Silesian and Polish Crown Land Vassal Relations

## Scope

Promoted `bohemia-poland-vassal-relation-01`, adding vassalage/suzerainty
relation context for four accepted Duchy parentage rows under two Kingdom-rank
crowns: Duchy of Silesia and Duchy of Nysa under the Kingdom of Bohemia, and
Duchy of Belz and Prince-Bishopric of Warmia under the Crown of the Kingdom of
Poland. This is consistent with the prior bohemian-silesian-vassal packet
(Duchy under Kingdom modeled as vassalage/suzerainty).

## Accepted Inputs

- `data/staging/bohemia-poland-vassal-relation-01.sources`
- `data/staging/bohemia-poland-vassal-relation-01.facts`

## Result

- Added 4 `vassalage_or_suzerainty` relation facts.
- Added no new source records; this packet uses already accepted source
  custody.
- Kept reviewed sources at 588.
- Raised accepted facts to 1660.
- Raised relation facts to 292.
- Kept parentage facts at 294.
- Kept title edge fill at 74.29%.
- Kept weighted span coverage at 59.73%.
- Kept rank-skip relation explanations at 232 of 232.
- Preserved 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/bohemia-poland-vassal-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/bohemia-poland-vassal-relation-01.sources data/staging/bohemia-poland-vassal-relation-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/bohemia-poland-vassal-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/bohemia-poland-vassal-relation-01.sources data/staging/bohemia-poland-vassal-relation-01.facts`
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
