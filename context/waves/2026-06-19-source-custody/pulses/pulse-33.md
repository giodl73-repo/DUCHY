# Pulse 33: Mass Title Source Scale-Up

## Intent

Scale the accepted source-backed title catalog from 13 to 50 reviewed sources
without expanding hierarchy claims faster than the current source-custody model
can review them.

## Changes

- Add 37 reviewed Wikidata source records for European historical title
  entities.
- Add minimal title identity, rank, and existence facts for those 37 entities.
- Promote the packet through `duchy-promote --dry-run --report` before applying
  it to accepted fixtures.
- Preserve existing parentage coverage unchanged at 24 reviewed relation facts.

## Review Boundary

This pulse authorizes title identity, coarse DUCHY rank mapping, and existence
spans only. It does not import borders, holders, parentage, control, dynastic
succession, administrative subunits, contested claims, or title equivalence
between predecessor and successor states.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/mass-title-50-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/mass-title-50.sources data/staging/mass-title-50.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/mass-title-50.sources data/staging/mass-title-50.facts`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
