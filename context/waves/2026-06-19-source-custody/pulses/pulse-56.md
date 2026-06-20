# Pulse 56: CK3 Batch 007 Research Screen

## Intent

Continue CK3 discovery through the seventh 50-row shard and preserve the
negative automated screen result for audit.

## Changes

- Query Wikidata search for CK3 batch 007.
- Add a structured-claim screen for the batch 007 top Wikidata leads.
- Record that no top lead has bounded date claims suitable for immediate
  promotion.

## Review Boundary

No sources or facts are promoted in this pulse. Batch 007 remains pending until
manual review checks alternate candidate QIDs or independent historical sources.

## Validation

- `cargo run --quiet --bin duchy-import -- manifest data/staging/ck3-counties-500.manifest`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo fmt --check`
- `git diff --check`
