# Pulse 211 - Parentage Rank-Skip Review Batch 04

Date: 2026-06-23

## Scope

- Review the next ten high-priority parentage rank-skip rows after batch 03.
- Check whether generated bridge candidates are importable immediate parents.
- Record relation semantics that should move out of parentage.

## Results

- Rank-skip rows reviewed: 10.
- Importable replacement facts: 0.
- Relation-model blockers: 10.

## Decision

Do not import any replacement parentage facts from this batch. The reviewed
Kingdom of Bohemia, Kingdom of Bavaria, Kingdom of Italy, and Kingdom of
Prussia bridge candidates are shared-parent false positives, not immediate
parents.

## Validation

```powershell
cargo test --quiet
Import-Csv data/staging/parentage-rank-skip-review-batch-04.tsv -Delimiter "`t" | Group-Object review_disposition
git diff --check
```

## Notes

- Accepted fixture facts remain unchanged.
- This batch identifies subdivision, vassalage, confederation membership,
  federal-state membership, imperial principality, and rank elevation as
  recurring non-parentage relation needs.
