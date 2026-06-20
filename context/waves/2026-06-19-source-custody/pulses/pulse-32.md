# Pulse 32: Oldenburg And Brunswick Import

## Intent

Continue reviewed historical intake with another narrow German-state packet,
adding Oldenburg and Brunswick to the source-backed bridge catalog.

## Changes

- Add reviewed Wikidata source metadata for Q693669 and Q326029.
- Add title facts for Grand Duchy of Oldenburg and Duchy of Brunswick.
- Add Oldenburg parentage under the German Confederation for 1816-1866, the
  North German Confederation for 1867-1870, and the German Empire for
  1871-1918.
- Add Brunswick parentage under the German Confederation for 1815-1866, the
  North German Confederation for 1867-1870, and the German Empire for
  1871-1918.
- Promote the packet through `duchy-promote --dry-run --report` before applying
  it to accepted fixtures.

## Review Boundary

This pulse authorizes minimal structured facts and parentage spans only. It
does not import borders, holders, exclaves, administrative districts, dynastic
succession, constitutional history, or contested territorial claims.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/oldenburg-brunswick-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/oldenburg-brunswick.sources data/staging/oldenburg-brunswick.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/oldenburg-brunswick.sources data/staging/oldenburg-brunswick.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
