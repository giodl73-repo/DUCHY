# Pulse 210 - Parentage Rank-Skip Review Batch 03

Date: 2026-06-23

## Scope

- Review the next ten still-unreviewed high-priority parentage rank-skip rows
  after county batches 01 and 02.
- Switch from county-only review to generic rank-skip review because the queue
  now contains duchy-rank titles.
- Check bridge candidates against accepted source custody.

## Results

- Rank-skip rows reviewed: 10.
- Importable replacement facts: 0.
- Relation-model blockers: 8.
- Source-custody tightening rows: 2.

## Decision

Do not import any replacement parentage facts from this batch. The reviewed
Kingdom of Bohemia and Kingdom of Bavaria bridge candidates are shared-parent
false positives, not immediate parents.

## Validation

```powershell
cargo test --quiet
Import-Csv data/staging/parentage-rank-skip-review-batch-03.tsv -Delimiter "`t" | Group-Object review_disposition
git diff --check
```

## Notes

- Accepted fixture facts remain unchanged.
- Archduchy of Austria under Austrian Empire and Austria-Hungary should receive
  text source-custody tightening before relation-model changes.
- The remaining high-priority rank-skip queue should be treated as relation
  semantics work, not a direct import queue.
