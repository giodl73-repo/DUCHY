# Pulse 168: One-Child Bridge Cluster Review Batch

## Intent

Scale bridge-cluster review by clearing the remaining one-child clusters that
resolve as same-parent sibling false positives.

## Changes

- Add `data/staging/one-child-bridge-cluster-review-batch-01.md`.
- Mark five additional cluster rows as `reviewed` with disposition
  `same_parent_sibling_false_positive`.
- Regenerate the cluster review report, packet stubs, and review shards.
- Document the updated review state in `README.md`.
- Add VTRACE evidence for the scaled one-child batch.

## Reviewed Rows

- Gorizia and Gradisca / Archduchy of Austria / Austria-Hungary.
- Gorizia and Gradisca / Archduchy of Austria / Austrian Empire.
- Comtat Venaissin / Duchy of Ferrara / Papal States.
- Prince-Bishopric of Montenegro / Eastern Hungarian Kingdom / Ottoman Empire.
- Grand Duchy of Finland / Kingdom of Imereti / Russian Empire.

## Boundary

This pulse changes review metadata only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or territorial
geometry. The reviewed rows remain blocked from import because none show
child-to-candidate parentage.

## Current State

- review rows: 20
- pending_review rows: 14
- reviewed false-positive rows: 6
- ready_for_packet rows: 0
- packet stubs: 0
- accepted source/fact/title counts: unchanged

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-report data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-packet-stubs data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md`
- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-shard data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-shards 5`
