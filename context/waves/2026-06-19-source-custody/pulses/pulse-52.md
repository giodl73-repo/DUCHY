# Pulse 52: CK3 Batch 003 Research Screen

## Intent

Continue the CK3 discovery workflow through the third 50-row shard and record
when automated top-lead screening finds no source records safe to promote.

## Changes

- Query Wikidata search for CK3 batch 003.
- Add a structured-claim screen for the batch 003 top Wikidata leads.
- Record that no top lead has bounded date claims suitable for immediate
  promotion.

## Review Boundary

No sources or facts are promoted in this pulse. Batch 003 remains pending until
manual review checks alternate candidate QIDs or independent historical sources.

## Validation

- `cargo run --quiet --bin duchy-import -- manifest data/staging/ck3-counties-500.manifest`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo fmt --check`
- `git diff --check`
