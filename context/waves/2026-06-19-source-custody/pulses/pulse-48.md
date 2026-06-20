# Pulse 48: CK3 Batch 001 Reviewed Source-Resolution Packet

## Intent

Move the first CK3 research shard from search leads toward accepted source
custody without importing unsupported title facts.

## Changes

- Add 7 reviewed Wikidata source records selected from the first CK3 county
  research shard.
- Promote those source records through `duchy-promote` with an empty facts
  packet.
- Increase accepted reviewed source records from 319 to 326 while preserving
  accepted facts at 1097 and reviewed parentage facts at 140.

## Review Boundary

This pulse accepts source records only. It does not assert title identity,
existence spans, parentage, holders, de facto control, or territory geometry for
the CK3 counties or their Wikidata leads.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-001-source-resolution-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-source-resolution.sources data/staging/ck3-batch-001-source-resolution.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-001-source-resolution-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-source-resolution.sources data/staging/ck3-batch-001-source-resolution.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
