# Pulse 172: Active Bridge Cluster Reconciliation

## Intent

Close the loop after the Kingdom of Italy replacement parentage packet changed
the active bridge/rank-skip surface. The accepted graph now excludes two
superseded direct HRE parentage facts, so the bridge-cluster reports need a
review reconciliation before the next scaling milestone.

## Changes

- Add `data/staging/parentage-rank-skip-bridge-cluster-active-reconciliation.md`.
- Confirm active bridge rows are down to 160 and active rank-skip rows are down
  to 220.
- Confirm the completed bridge-cluster review queue has no pending rows and no
  ready-for-packet rows.
- Reconcile the Kingdom of Italy / HRE cluster:
  - March of Turin and March of Tuscany are no longer active bridge children
    because their replacement facts were accepted.
  - Prince-Bishopric of Trent remains a held relation-model lead.
  - The remaining 28 active Italy/HRE children remain same-parent sibling false
    positives.

## Boundary

This pulse adds review reconciliation only. It does not import new facts,
promote sources, alter geometry, or reopen the closed bridge-cluster queue.

## Current State

- active rank-skip rows: 220
- active bridge rows: 160
- active bridge clusters: 20
- reviewed bridge-cluster rows: 20
- pending bridge-cluster rows: 0
- ready-for-packet rows: 0
- active Italy/HRE children: 29
- held Italy/HRE leads: 1

## Validation

- `Import-Csv data/staging/parentage-rank-skip-bridge-cluster-review.tsv -Delimiter "`t"` grouped by review status and disposition.
- `Import-Csv data/staging/kingdom-italy-hre-child-split-review.tsv -Delimiter "`t"` compared against the active Italy/HRE cluster child IDs.
- `cargo test --quiet`
- `git diff --check`
