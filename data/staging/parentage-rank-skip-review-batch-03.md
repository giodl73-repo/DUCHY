# DUCHY Parentage Rank-Skip Review Batch 03

source_queue: `data/staging/parentage-rank-skip-targets.tsv`
review_tsv: `data/staging/parentage-rank-skip-review-batch-03.tsv`

## Boundary

- This is not an import packet.
- The reviewed rows are the first ten still-unreviewed high-priority rank-skip
  rows after county batches 01 and 02.
- These rows are duchy-rank titles that skip an expected kingdom layer.

## Summary

| Disposition | Rows |
|---|---:|
| needs_relation_model | 8 |
| needs_source_custody_tightening | 2 |

## Decision

No new parentage facts are importable from this batch.

The bridge candidates are shared-parent false positives. Kingdom of Bohemia is
not an importable parent for the reviewed HRE or Austrian monarchy rows, and
Kingdom of Bavaria is not an importable parent for the reviewed German
Confederation rows.

## Row Notes

| Child | Span | Disposition | Note |
|---|---:|---|---|
| Anhalt-Bernburg | 1252..1806 | needs_relation_model | HRE state evidence is accepted; Bohemia bridge is a false positive. |
| Anhalt-Bernburg | 1815..1863 | needs_relation_model | German Confederation membership is accepted; Bavaria bridge is a false positive. |
| Anhalt-Dessau | 1396..1806 | needs_relation_model | HRE state evidence is accepted; partition detail remains out of scope. |
| Anhalt-Dessau | 1815..1853 | needs_relation_model | German Confederation membership is accepted; Bavaria bridge is a false positive. |
| Anhalt-Kothen | 1396..1806 | needs_relation_model | HRE state evidence is accepted; partition detail remains out of scope. |
| Anhalt-Kothen | 1815..1863 | needs_relation_model | German Confederation membership is accepted; Bavaria bridge is a false positive. |
| Archduchy of Austria | 1358..1803 | needs_relation_model | HRE state evidence is accepted; hereditary lands and imperial-office semantics need a model. |
| Archduchy of Austria | 1804..1866 | needs_source_custody_tightening | Current Austrian Empire edge is structured-data backed and needs text custody before model changes. |
| Archduchy of Austria | 1867..1918 | needs_source_custody_tightening | Current Austria-Hungary edge is structured-data backed and needs text custody before model changes. |
| Bavaria-Munich | 1392..1505 | needs_relation_model | HRE constituent-state evidence is accepted; Wittelsbach partition semantics remain out of scope. |

## Resulting Queue State

| Status | Rows |
|---|---:|
| reviewed rows | 10 |
| importable replacement facts | 0 |
| blocked relation-model rows | 8 |
| source-custody tightening rows | 2 |

## Metadata Learning

The high-priority rank-skip queue is no longer primarily a missing-parentage
queue. It is now dominated by relation semantics: imperial states, confederation
members, crownlands, hereditary lands, and internal dynastic partitions. Large
scale agent work should classify those relation types first, then import only
rows with child-level evidence for a true parentage replacement.

## Next Step

Add relation-type design and source-custody tightening work packages before
treating the remaining high-priority rank skips as direct import candidates.
