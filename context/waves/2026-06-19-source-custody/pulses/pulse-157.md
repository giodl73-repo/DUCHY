# Pulse 157: Rank Skip Review Shards

## Intent

Shard the rank-skip review queue so missing intermediate parentage work can be
assigned in small review batches.

## Changes

- Add `duchy-import parentage-rank-skip-shard`.
- Add `duchy-import parentage-rank-skip-report`.
- Generate `data/staging/parentage-rank-skip-shards/`.
- Generate `data/staging/parentage-rank-skip-report.md`.
- Document the commands and generated queue shape in `README.md`.
- Add VTRACE evidence for the sharded review baseline.

## Boundary

This pulse adds review packaging only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or territorial
geometry.

## Current State

- rank-skip rows: 222
- review shards: 9
- chunk size: 25
- high-priority rows: 160
- medium-priority rows: 23
- low-priority rows: 39
- missing immediate `Kingdom` rank rows: 148
- missing immediate `Duchy` rank rows: 35
- missing immediate `Crown` rank rows: 39
- largest current edge class: Duchy -> Empire, 137 rows

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-shard data/staging/parentage-rank-skip-targets.tsv data/staging/parentage-rank-skip-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-report data/staging/parentage-rank-skip-targets.tsv data/staging/parentage-rank-skip-report.md`
