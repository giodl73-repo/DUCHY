# Pulse 53: CK3 Batch 004 Reviewed Packet

## Intent

Continue the CK3 discovery workflow through the fourth 50-row shard and promote
only the bounded candidates that map cleanly to materialized DUCHY ranks.

## Changes

- Query Wikidata search for CK3 batch 004.
- Add a structured-claim screen for batch 004 top Wikidata leads.
- Promote 3 reviewed source records:
  - Kingdom of Brycheiniog
  - Byzantium
  - Cetatea-Albă County
- Promote 6 title facts for Brycheiniog and Cetatea-Albă County.
- Increase accepted sources from 329 to 332, accepted facts from 1100 to 1106,
  and materialized titles from 320 to 322.

## Review Boundary

Byzantium remains source-only. It has bounded dates, but its structured entity
is an ancient city, and DUCHY does not yet materialize ancient-city rank
semantics as title facts. No parentage, holder, control, territory, or
continuity facts are promoted.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-004-reviewed-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-004-reviewed.sources data/staging/ck3-batch-004-reviewed.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-004-reviewed-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-004-reviewed.sources data/staging/ck3-batch-004-reviewed.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
