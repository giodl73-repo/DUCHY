# Pulse 55: CK3 Batch 006 Source-Resolution Packet

## Intent

Continue the CK3 shard workflow through batch 006 while avoiding duplicate
source promotion and deferring region-rank title semantics.

## Changes

- Query Wikidata search for CK3 batch 006.
- Add a structured-claim screen for batch 006 top Wikidata leads.
- Identify two bounded candidates: Donji Kraji and Duklja.
- Promote Donji Kraji as a reviewed source-only record.
- Skip Duklja because `src-wikidata-q1252942` and its title facts are already
  accepted from the earlier candidate queue.
- Increase accepted sources from 332 to 333 while preserving accepted facts at
  1106.

## Review Boundary

Donji Kraji title facts remain deferred. Its structured entity is a historical
region / zemlja, and DUCHY does not yet materialize that rank. No parentage,
holder, control, territory, or continuity facts are promoted.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-006-source-resolution-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-006-source-resolution.sources data/staging/ck3-batch-006-source-resolution.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-006-source-resolution-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-006-source-resolution.sources data/staging/ck3-batch-006-source-resolution.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
