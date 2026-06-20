# Pulse 50: CK3 Batch 001 Structured Screen And Ailech Source Record

## Intent

Screen the first CK3 research shard for structured date claims and continue
source custody without forcing ambiguous rank semantics into title facts.

## Changes

- Add a structured-claim screen for first-shard Wikidata leads.
- Identify two bounded candidates: Abaúj county and Ailech.
- Promote Ailech as a reviewed source-only record.
- Increase accepted sources from 326 to 327 while preserving accepted facts at
  1100.

## Review Boundary

Ailech title facts remain deferred. Its Wikidata source has structured
`600..780` dates, but its instance type is historical country while the CK3
discovery row is a county lead. DUCHY rank and title identity require manual
review before materialization.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/ck3-batch-001-source-resolution-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-source-resolution-02.sources data/staging/ck3-batch-001-source-resolution-02.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/ck3-batch-001-source-resolution-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/ck3-batch-001-source-resolution-02.sources data/staging/ck3-batch-001-source-resolution-02.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
