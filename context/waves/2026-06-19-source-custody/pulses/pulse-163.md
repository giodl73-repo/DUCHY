# Pulse 163: Rank Skip Bridge Cluster Review Gate

## Intent

Make the bridge-cluster safety boundary operational before using clusters to
drive scaled parentage imports.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-cluster-review-tsv`.
- Generate `data/staging/parentage-rank-skip-bridge-cluster-review.tsv`.
- Export one review row per bridge cluster with `pending_review` status and
  `not_inferred` disposition.
- Require `child_to_candidate_parentage_source` evidence before any cluster can
  become an import packet.
- Document the command and review-gate semantics in `README.md`.
- Add VTRACE evidence for the cluster review queue.

## Boundary

This pulse does not promote sources, facts, titles, parentage spans, de facto
relations, successor semantics, or territorial geometry. Same-parent bridge
overlap is treated as a lead only; it is not accepted as evidence that a child
belonged to the candidate intermediate parent.

## Current State

- bridge clusters: 20
- review rows: 20
- default status: pending_review
- default disposition: not_inferred
- required evidence: child_to_candidate_parentage_source

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-tsv data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-review.tsv`
