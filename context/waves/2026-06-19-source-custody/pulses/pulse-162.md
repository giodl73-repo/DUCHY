# Pulse 162: Rank Skip Bridge Cluster Report

## Intent

Turn the bridge-cluster TSV into readable Markdown review packets for scaling
intermediate-parent research.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-cluster-report`.
- Generate `data/staging/parentage-rank-skip-bridge-cluster-report.md`.
- Report cluster totals, unique clustered child entries, priority counts, span
  ranges, bridge fact IDs, skip fact IDs, child IDs, and child names.
- Keep the report boundary explicit: clusters are packet-planning leads, not
  import-ready parentage claims.
- Document the command in `README.md`.
- Add VTRACE evidence for the cluster packet report.

## Boundary

This pulse creates a review report only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or
territorial geometry. Each packet still needs child-to-candidate parentage
evidence before any new historical claim can enter accepted fixtures.

## Current State

- bridge clusters: 20
- clustered child entries: 161
- high-priority bridge rows: 150
- medium-priority bridge rows: 9
- low-priority bridge rows: 3

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-report data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-report.md`
