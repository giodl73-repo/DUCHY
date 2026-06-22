# Pulse 164: Rank Skip Bridge Cluster Review Shards

## Intent

Split the cluster review queue into small assignable packets while preserving
the pending and not-inferred safety gate.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-cluster-review-shard`.
- Generate `data/staging/parentage-rank-skip-bridge-cluster-review-shards/`.
- Write four five-row TSV shards plus `INDEX.md`.
- Preserve review status, review disposition, evidence requirement, child IDs,
  child names, skip fact IDs, and bridge fact IDs in each shard.
- Summarize pending, not-inferred, high, medium, and low counts per shard.
- Document the shard command in `README.md`.
- Add VTRACE evidence for the assignable review packets.

## Boundary

This pulse only partitions the review queue. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or territorial
geometry. Each shard still requires source-custody review before any import
packet can be created.

## Current State

- review rows: 20
- shards: 4
- chunk size: 5
- default status: pending_review
- default disposition: not_inferred

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-shard data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-shards 5`
