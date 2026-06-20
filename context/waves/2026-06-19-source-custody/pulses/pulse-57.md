# Pulse 57: CK3 Queue Closure

## Intent

Finish the first 500-row CK3 discovery queue so no candidate rows remain
pending, while preserving the boundary that CK3 rows are search leads rather
than accepted historical sources.

## Changes

- Query Wikidata search for CK3 batches 008, 009, and 010.
- Add structured-claim screens for all remaining top Wikidata leads.
- Promote Hordaland as a reviewed source-backed county title.
- Defer the Isle de France top hit as a false positive for Mauritius rather
  than the French county lead.
- Archive all 500 CK3 discovery rows as `scope_deferred`, leaving zero pending
  rows in `ck3-counties-500.manifest` and its regenerated shards.
- Increase accepted sources from 333 to 334, accepted facts from 1106 to 1109,
  and materialized titles from 322 to 323.

## Review Boundary

The direct CK3 rows are all closed as discovery-only records. Future promotion
must cite independent reviewed historical sources or already accepted source
records. No parentage, holder, de facto control, territory, or continuity facts
are promoted in this pulse.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-008-reviewed-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-008-reviewed.sources data/staging/ck3-batch-008-reviewed.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-008-reviewed-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-008-reviewed.sources data/staging/ck3-batch-008-reviewed.facts`
- `cargo run --quiet --bin duchy-import -- manifest data/staging/ck3-counties-500.manifest`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
