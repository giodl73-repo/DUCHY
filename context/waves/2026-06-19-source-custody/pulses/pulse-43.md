# Pulse 43: Holy Roman Empire Parentage Packet

## Intent

Expand reviewed source-backed parentage coverage for accepted Holy Roman Empire
member titles without adding new source records or broad relation inference.

## Changes

- Add 70 parentage-only facts for accepted child titles under the Holy Roman
  Empire.
- Cite existing accepted Wikidata source records for both child and parent
  titles on each parentage fact.
- Preserve accepted source count at 319 while increasing accepted facts to 1089
  and reviewed parentage facts to 132.

## Review Boundary

This pulse authorizes the listed parentage facts only. It does not import
borders, holders, dynastic continuity, de facto control, successor/predecessor
relations, or new source records.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/hre-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-parentage-01.sources data/staging/hre-parentage-01.facts`
- `cargo run --bin duchy-promote -- --apply --report data/staging/hre-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-parentage-01.sources data/staging/hre-parentage-01.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`

