# Pulse 62: Parentage Gap Sharding and Reports

## Intent

Turn the parentage gap TSV into reviewable import batches. This makes the next
source-custody work unit a bounded shard instead of another open-ended search.

## Changes

- Add `duchy-import parentage-gap-shard input.tsv output-dir chunk-size`.
- Add `duchy-import parentage-gap-report input.tsv output.md`.
- Split `data/staging/parentage-gap-targets.tsv` into 9 fixed-size shard TSVs
  under `data/staging/parentage-gap-shards/`.
- Generate `INDEX.md` with per-shard row and priority counts.
- Generate Markdown reports for all 9 shard TSVs.

## Current Shards

- gap rows: 211
- chunk size: 25
- shard count: 9
- first shard: 25 rows, including 5 high-priority title gaps
- final shard: 11 rows

## Review Boundary

This pulse adds queue shaping and review reports only. It does not import source
records or historical facts.

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-gap-report data/staging/parentage-gap-shards/batch-001.tsv data/staging/parentage-gap-shards/batch-001-report.md`
- `cargo test --quiet -j 1`
- `cargo fmt --check`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
