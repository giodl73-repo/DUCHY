# Pulse 49: CK3 Batch 001 First Title Fact Packet

## Intent

Promote only the first clean CK3-derived historical title facts after source
resolution, proving the queue can move from game-data search lead to reviewed
source-backed title identity without over-claiming.

## Changes

- Add an Abaúj county fact packet using the already accepted Wikidata source
  record `src-wikidata-q1049854`.
- Promote name, county rank, and `1201..1881` existence facts.
- Increase accepted facts from 1097 to 1100 and materialized titles from 319 to
  320.

## Review Boundary

This pulse promotes no CK3 facts directly. It promotes only reviewed structured
claims from the accepted Wikidata source record. Parentage, holders, de facto
control, territory, and continuity remain deferred.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-001-title-facts-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-title-facts.sources data/staging/ck3-batch-001-title-facts.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-001-title-facts-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-title-facts.sources data/staging/ck3-batch-001-title-facts.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
