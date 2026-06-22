# Pulse 167: Nassau Brandenburg Cluster Review

## Intent

Complete the first bridge-cluster review row and prove the review gate can
reject a false-positive lead without promoting parentage.

## Changes

- Review the `Margraviate of Brandenburg -> Holy Roman Empire` cluster row for
  child `County of Nassau`.
- Add `data/staging/nassau-brandenburg-cluster-review.md`.
- Mark the cluster row `reviewed` with disposition
  `same_parent_sibling_false_positive`.
- Regenerate the cluster review report, packet stubs, and review shards.
- Document the first completed negative cluster review in `README.md`.
- Add VTRACE evidence for the reviewed false-positive outcome.

## Boundary

This pulse changes review metadata only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or territorial
geometry. The review found that Nassau and Brandenburg were separate Holy Roman
Empire states, not a child-to-candidate parentage edge.

## Current State

- review rows: 20
- pending_review rows: 19
- reviewed false-positive rows: 1
- ready_for_packet rows: 0
- packet stubs: 0
- accepted source/fact/title counts: unchanged

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-report data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-packet-stubs data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-shard data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-shards 5`
