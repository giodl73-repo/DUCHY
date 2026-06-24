# Pulse 277 - East Francia Stem Duchy Relations

## Scope

Promoted `east-francia-stem-duchy-relation-01`, adding vassalage/suzerainty
relation context for six accepted Duchy parentage rows under East Francia:
Duchy of Saxony, Duchy of Bavaria, Duchy of Swabia, Duchy of Lorraine,
Lotharingia, and Lower Lotharingia.

## Modeling Decision

East Francia (q153080) is a Kingdom that existed 843..962, before the Holy
Roman Empire. The stem duchies are therefore modeled as
`vassalage_or_suzerainty` under the East Frankish crown, mirroring the French
crown vassal packet (Duchy under Kingdom) rather than `imperial_state`, which is
reserved for Holy Roman Empire constituents. The `imperial_state:title-q12548`
relations these duchies already carry apply to their later Holy Roman Empire
spans and are unaffected; the new relation spans match the accepted East Francia
parentage and do not overlap.

## Accepted Inputs

- `data/staging/east-francia-stem-duchy-relation-01.sources`
- `data/staging/east-francia-stem-duchy-relation-01.facts`

## Result

- Added 6 `vassalage_or_suzerainty` relation facts.
- Added no new source records; this packet uses already accepted source
  custody.
- Kept reviewed sources at 588.
- Raised accepted facts to 1656.
- Raised relation facts to 288.
- Kept parentage facts at 294.
- Kept title edge fill at 74.29%.
- Kept weighted span coverage at 59.73%.
- Kept rank-skip relation explanations at 232 of 232.
- Preserved 0 temporal parent conflicts.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/east-francia-stem-duchy-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/east-francia-stem-duchy-relation-01.sources data/staging/east-francia-stem-duchy-relation-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/east-francia-stem-duchy-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/east-francia-stem-duchy-relation-01.sources data/staging/east-francia-stem-duchy-relation-01.facts`
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
