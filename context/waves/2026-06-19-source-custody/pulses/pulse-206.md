# Pulse 206 - Italy HRE Split Reconciliation

Date: 2026-06-23

## Scope

- Refresh the Kingdom of Italy / Holy Roman Empire child split review after
  materialized partial replacement reports.
- Mark March of Turin and March of Tuscany as accepted replacement imports
  instead of still-blocked replacement candidates.
- Keep Prince-Bishopric of Trent held for relation-model review.

## Results

- Active Italy/HRE split rows: 29.
- Active candidate child review leads: 1.
- Active same-parent sibling false positives: 28.
- Accepted replacement imports recorded in the candidate review: 2.
- Held relation-model leads: 1.

## Validation

```powershell
Import-Csv data/staging/kingdom-italy-hre-child-split-review.tsv -Delimiter "`t" | Group-Object split_status
Import-Csv data/staging/kingdom-italy-hre-replacement-candidates.tsv -Delimiter "`t" | Group-Object candidate_status
```

## Notes

- The next importable work is not a broad Kingdom of Italy/HRE packet. It is a
  narrow Trent relation review, and only if a child-level source supports
  bounded parentage under the Kingdom of Italy.
- The closed bridge-cluster review queue should stay closed unless new
  source-backed child evidence changes a specific row.
