# Pulse 209 - County Rank-Skip Review Batch 02

Date: 2026-06-23

## Scope

- Review the next ten high-priority county rank-skip rows from the materialized
  parentage edge queue.
- Check bridge candidates against accepted source custody.
- Record any metadata issues that affect scaling assignment.

## Results

- County rank-skip rows reviewed: 10.
- Importable replacement facts: 0.
- Child-level source blockers: 3.
- Relation-model blockers: 4.
- Split-control relation-model blockers: 2.
- Rank-identity blockers: 1.

## Decision

Do not import any replacement parentage facts from this batch. All bridge
candidates in the reviewed rows are shared-HRE-parent false positives. Keep the
existing direct HRE parentage facts until a child-level source or a new relation
type supports a more precise edge.

## Validation

```powershell
cargo test --quiet
Import-Csv data/staging/county-rank-skip-review-batch-02.tsv -Delimiter "`t" | Group-Object review_disposition
git diff --check
```

## Notes

- Accepted fixture facts remain unchanged.
- `title-q152420` / Duchy of Guelders needs rank identity review because the
  current fact row has a Duchy label but a County rank.
- The repeated blockers support a relation-model work package before large
  county-scale parentage imports continue.
