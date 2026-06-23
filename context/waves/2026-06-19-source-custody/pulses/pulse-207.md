# Pulse 207 - Trent Italy Relation Review

Date: 2026-06-23

## Scope

- Review the sole remaining active Kingdom of Italy / Holy Roman Empire split
  lead: Prince-Bishopric of Trent.
- Decide whether current source custody supports a replacement parentage edge
  from Trent to the Kingdom of Italy.

## Results

- Active split leads reviewed: 1.
- Importable replacement facts: 0.
- Blocked relation-model leads: 1.

## Decision

Keep Trent parented directly under the Holy Roman Empire for `1027..1803`.
Current evidence supports Trent as an HRE imperial estate/state, and Kingdom of
Italy context supports Italian imperial-state framing, but it does not establish
a clean bounded parentage replacement from Trent to the Kingdom of Italy.

## Validation

```powershell
cargo test --quiet
Import-Csv data/staging/trent-italy-hre-relation-review.tsv -Delimiter "`t" | Group-Object review_disposition
```

## Notes

- The Italy/HRE cluster remains closed and blocked with zero packet-ready rows.
- Future work should use a relation type other than parentage for imperial
  fiefs/circles if DUCHY needs to query that layer.
