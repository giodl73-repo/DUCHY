# Pulse 31: Baden And Hanover Import

## Intent

Continue reviewed historical intake with a narrow Baden and Hanover packet that
extends the German parentage catalog without changing source-custody policy.

## Changes

- Add reviewed Wikidata source metadata for Q186320 and Q164079.
- Add title facts for Grand Duchy of Baden and Kingdom of Hanover.
- Add Baden parentage spans under the German Confederation for 1815-1866 and
  the German Empire for 1871-1918.
- Add Hanover parentage under the German Confederation for 1815-1866.
- Promote the packet through `duchy-promote --dry-run --report` before applying
  it to accepted fixtures.

## Review Boundary

This pulse authorizes minimal structured facts and parentage spans only. It
does not import borders, holders, detailed sovereignty changes, full member
hierarchies, dynastic lineages, or contested territorial claims.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/baden-hanover-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/baden-hanover.sources data/staging/baden-hanover.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/baden-hanover.sources data/staging/baden-hanover.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
