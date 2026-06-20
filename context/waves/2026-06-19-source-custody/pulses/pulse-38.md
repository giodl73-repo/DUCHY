# Pulse 38: Remaining Title Queue Promotion

## Intent

Clear the promotable `title_identity_only` remainder from the 500-source
candidate queue without importing relation, border, holder, or contested
history claims.

## Changes

- Review 239 pending `title_identity_only` candidates from the remaining
  500-source queue.
- Fetch structured Wikidata labels, inception claims, and dissolution claims
  for the reviewed title candidates.
- Promote 225 source records and 675 title facts into accepted fixtures.
- Defer fourteen title candidates whose structured claims do not provide a
  valid inception/dissolution span.
- Mark the 225 promoted rows in the 500-source queue and regenerate reports,
  TSV, duplicate URL report, and review shards.

## Review Boundary

This pulse authorizes title identity, coarse rank, and existence spans only. It
does not import parentage, borders, holders, dynastic continuity, control, or
successor/predecessor relations.

## Validation

- `cargo run --bin duchy-promote -- --dry-run --report data/staging/remaining-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/remaining-title.sources data/staging/remaining-title.facts`
- `cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/remaining-title.sources data/staging/remaining-title.facts`
- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
