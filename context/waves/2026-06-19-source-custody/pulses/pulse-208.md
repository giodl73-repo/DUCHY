# Pulse 208 - County Rank-Skip Review Batch 01

Date: 2026-06-23

## Scope

- Review the first ten high-priority county rank-skip rows from the generated
  materialized parentage edge queue.
- Separate import-ready parentage replacements from false-positive bridge leads
  and relation-model blockers.

## Results

- County rank-skip rows reviewed: 10.
- Importable replacement facts: 0.
- Same-parent sibling false positives: 2.
- Child-level source blockers: 3.
- Relation-model blockers: 5.

## Decision

Do not import any replacement parentage facts from this batch. The reviewed
bridges are packet-planning leads, not historical claims. The first county
sample confirms that Duchy of Bavaria, Duchy of Ferrara, and Anhalt-Bernburg
are shared-parent sibling false positives for these rows, while the remaining
rows require bounded child-level sources or new relation models before they can
change the hierarchy.

## Validation

```powershell
cargo test --quiet
Import-Csv data/staging/county-rank-skip-review-batch-01.tsv -Delimiter "`t" | Group-Object review_disposition
git diff --check
```

## Notes

- Accepted fixture facts remain unchanged.
- The next scaling step should continue county rank-skip review batches and
  only promote rows with child-level source support for an immediate parent.
