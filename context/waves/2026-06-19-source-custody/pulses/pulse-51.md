# Pulse 51: CK3 Batch 002 Source-Resolution Packet

## Intent

Continue CK3 discovery scale-up into the second 50-row shard while preserving
the source-custody rule that search results and bounded dates do not by
themselves determine DUCHY rank semantics.

## Changes

- Query Wikidata search for CK3 batch 002.
- Add a structured-claim screen for the batch 002 top Wikidata leads.
- Identify two bounded candidates: Principality of Arbanon and Béarn.
- Promote both as reviewed source-only records.
- Increase accepted sources from 327 to 329 while preserving accepted facts at
  1100.

## Review Boundary

No title facts are promoted in this pulse. Arbanon is explicitly a
principality, which DUCHY does not yet materialize as a title rank. Béarn is a
former province and needs rank/identity review before title materialization.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-002-source-resolution-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-002-source-resolution.sources data/staging/ck3-batch-002-source-resolution.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-002-source-resolution-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-002-source-resolution.sources data/staging/ck3-batch-002-source-resolution.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
