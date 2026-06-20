# Pulse 35: 500-Source Candidate Queue

## Intent

Scale DUCHY from 50 accepted source-backed records to 500 sources under review
without treating unreviewed Wikidata candidates as accepted historical facts.

## Changes

- Generate `data/staging/candidates-500.manifest` with 450 pending Wikidata
  candidate sources.
- Use Wikidata historical-country entities with English Wikipedia sitelinks and
  Europe continent metadata as the candidate query boundary.
- Exclude the 50 already accepted Wikidata source IDs from the staged queue.
- Add scale metadata to each candidate row: batch ID, import scope, rank basis,
  entity class, source claims used, confidence detail, parentage status, and
  query readiness.
- Generate a summary, manifest report, TSV export, duplicate URL report, and
  nine 50-row review shards.

## Review Boundary

This pulse stages candidates only. It does not add accepted sources, facts,
parentage, control, borders, holders, or lineage events. Automated entity-class
metadata is advisory until a source-custody reviewer promotes a shard or row.

## Validation

- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- manifest-report data/staging/candidates-500.manifest data/staging/candidates-500-report.md`
- `cargo run --bin duchy-import -- manifest-tsv data/staging/candidates-500.manifest data/staging/candidates-500.tsv`
- `cargo run --bin duchy-import -- duplicate-url-report data/staging/candidates-500.manifest data/staging/candidates-500-duplicate-urls.md`
- `cargo run --bin duchy-import -- shard-manifest data/staging/candidates-500.manifest data/staging/candidates-500-shards 50`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
