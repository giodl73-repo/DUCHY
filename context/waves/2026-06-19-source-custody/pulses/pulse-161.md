# Pulse 161: Rank Skip Bridge Clusters

## Intent

Convert the compact bridge queue into packet-planning clusters grouped by
candidate intermediate parent and current parent.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-clusters-tsv`.
- Generate `data/staging/parentage-rank-skip-bridge-clusters.tsv`.
- Group bridge rows by candidate parent, current parent, expected parent rank,
  and current parent rank.
- Export child counts, priority counts, span range, child IDs/names, skip fact
  IDs, and bridge fact IDs for each cluster.
- Document the command and cluster baseline in `README.md`.
- Add VTRACE evidence for the cluster planning baseline.

## Boundary

This pulse produces packet-planning metadata only. It does not decide that a
candidate parent is historically correct for any child, and it does not promote
sources, facts, titles, parentage spans, de facto relations, successor
semantics, or territorial geometry.

## Current State

- bridge rows: 162
- bridge clusters: 20
- largest clusters: Kingdom of Bohemia under Holy Roman Empire, Kingdom of
  Italy under Holy Roman Empire, and Kingdom of Bavaria under German
  Confederation

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-clusters-tsv data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-clusters.tsv`
