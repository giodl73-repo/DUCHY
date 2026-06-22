# Pulse 160: Rank Skip Bridge Review Shards

## Intent

Shard the compact rank-skip bridge queue and summarize high-leverage bridge
review clusters.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-shard`.
- Add `duchy-import parentage-rank-skip-bridge-report`.
- Generate `data/staging/parentage-rank-skip-bridge-shards/`.
- Generate `data/staging/parentage-rank-skip-bridge-report.md`.
- Document bridge shard/report commands and rollup findings in `README.md`.
- Add VTRACE evidence for the bridge review shard baseline.

## Boundary

This pulse packages bridge review leads only. It does not promote sources,
facts, titles, parentage spans, de facto relations, successor semantics, or
territorial geometry. Bridge rows still require source review before any
immediate parentage replacement can be promoted.

## Current State

- bridge queue rows: 162
- review shards: 7
- high-priority rows: 150
- medium-priority rows: 9
- low-priority rows: 3
- largest candidate-parent clusters: Kingdom of Bohemia, Kingdom of Bavaria,
  Kingdom of Italy
- largest current-parent cluster: Holy Roman Empire

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-shard data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-shards 25`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-report data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-report.md`
