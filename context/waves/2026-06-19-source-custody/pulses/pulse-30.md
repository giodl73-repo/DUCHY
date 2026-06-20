# Pulse 30: German Bridge Import

## Intent

Move from import tooling back to reviewed historical data by adding a small
bridge packet across the German Confederation, North German Confederation, and
German Empire eras.

## Changes

- Add reviewed Wikidata source metadata for Q151624, Q150981, Q154195, and
  Q159631.
- Add title facts for German Confederation, North German Confederation, Kingdom
  of Bavaria, and Kingdom of Wurttemberg.
- Add reviewed parentage spans for selected existing German titles in
  1815-1866 and 1867-1870.
- Add Bavaria and Wurttemberg parentage spans into the German Empire for
  1871-1918.
- Promote the packet through `duchy-promote --dry-run --report` before applying
  it to accepted fixtures.

## Review Boundary

This pulse authorizes minimal structured facts and bridge parentage only. It
does not import borders, holders, detailed constitutional structure, federal
organs, military alliances, legal prose, or a complete member-state hierarchy.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/german-bridge-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-bridge.sources data/staging/german-bridge.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/german-bridge.sources data/staging/german-bridge.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo fmt --check`
- `git diff --check`
