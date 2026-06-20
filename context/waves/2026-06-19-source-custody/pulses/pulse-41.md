# Pulse 41: Additional German Parentage Packet

## Intent

Expand reviewed source-backed parentage coverage without adding new source
records or broad relation inference.

## Changes

- Add 23 parentage-only facts for accepted German Confederation, North German
  Confederation, and German Empire-era titles.
- Cite existing accepted Wikidata source records for both child and parent
  titles on each parentage fact.
- Extend `duchy-promote` so candidate facts validate against the merged
  accepted-plus-candidate source catalog, enabling parentage-only packets that
  cite already accepted source records.
- Preserve accepted source count at 319 while increasing accepted facts to 1004
  and reviewed parentage facts to 47.

## Review Boundary

This pulse authorizes the listed parentage facts only. It does not import
borders, holders, dynastic continuity, de facto control, successor/predecessor
relations, or new source records.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/german-parentage-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-02.sources data/staging/german-parentage-02.facts`
- `cargo run --bin duchy-promote -- --apply --report data/staging/german-parentage-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-02.sources data/staging/german-parentage-02.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
