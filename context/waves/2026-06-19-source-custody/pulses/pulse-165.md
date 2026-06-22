# Pulse 165: Rank Skip Bridge Cluster Review Report

## Intent

Add a progress report over the editable bridge-cluster review queue so packet
work can be audited without opening every shard.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-cluster-review-report`.
- Generate `data/staging/parentage-rank-skip-bridge-cluster-review-report.md`.
- Summarize review rows by status, disposition, evidence need, and priority
  totals.
- List each cluster review row with candidate parent, current parent, child
  count, priority counts, and evidence requirement.
- Document the report command in `README.md`.
- Add VTRACE evidence for the review-progress report.

## Boundary

This pulse reports review progress only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or territorial
geometry. Rows with `not_inferred` disposition remain blocked from import until
reviewers supply child-to-candidate parentage evidence.

## Current State

- review rows: 20
- pending_review rows: 20
- not_inferred rows: 20
- high-priority rows represented: 150
- medium-priority rows represented: 9
- low-priority rows represented: 3

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-report data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-report.md`
