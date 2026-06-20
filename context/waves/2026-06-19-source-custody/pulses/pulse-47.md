# Pulse 47: CK3 Batch 001 Source-Resolution Leads

## Intent

Start converting the CK3 discovery queue into reviewable historical-source
work by identifying external entity leads for the first 50 county candidates.

## Changes

- Query Wikidata search for the first 50 CK3 Europe county candidates.
- Record top and alternate QID leads in a fixed-column TSV.
- Summarize exact-label, fuzzy-label, no-result, and error counts.

## Review Boundary

This pulse does not accept any Wikidata search hit as a historical source
record. Search results can point reviewers toward candidate entities, but they
do not prove that a CK3 county, historical title, modern place, and source-backed
DUCHY title are the same entity.

## Validation

- Inspect `data/staging/ck3-counties-batch-001-wikidata-research.tsv`.
- Inspect `data/staging/ck3-counties-batch-001-wikidata-research.md`.
- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo fmt --check`
- `git diff --check`
