# Pulse 45: Residual German Confederation Parentage Packet

## Intent

Expand reviewed source-backed parentage coverage for accepted German
Confederation-era titles that were not covered by the earlier German packets.

## Changes

- Add 4 parentage-only facts for accepted child titles under the German
  Confederation.
- Cite existing accepted Wikidata source records for both child and parent
  titles on each parentage fact.
- Preserve accepted source count at 319 while increasing accepted facts to 1097
  and reviewed parentage facts to 140.

## Review Boundary

This pulse authorizes the listed parentage facts only. It does not import
borders, holders, dynastic continuity, de facto control, successor/predecessor
relations, or new source records.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/german-parentage-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-03.sources data/staging/german-parentage-03.facts`
- `cargo run --bin duchy-promote -- --apply --report data/staging/german-parentage-03-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-03.sources data/staging/german-parentage-03.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`

