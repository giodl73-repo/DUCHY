# DUCHY Parentage Rank-Skip Review Batch 04

source_queue: `data/staging/parentage-rank-skip-targets.tsv`
review_tsv: `data/staging/parentage-rank-skip-review-batch-04.tsv`

## Boundary

- This is not an import packet.
- The reviewed rows are the next ten still-unreviewed high-priority rank-skip
  rows after batch 03.
- These rows are duchy-rank titles whose accepted parentage skips the expected
  kingdom layer.

## Summary

| Disposition | Rows |
|---|---:|
| needs_relation_model | 10 |

## Decision

No new parentage facts are importable from this batch.

The generated bridge candidates remain shared-parent false positives. Kingdom
of Bohemia, Kingdom of Bavaria, Kingdom of Italy, and Kingdom of Prussia are not
importable immediate parents for these rows from current accepted evidence.

## Row Notes

| Child | Span | Disposition | Note |
|---|---:|---|---|
| Brunswick-Luneburg | 1235..1806 | needs_relation_model | HRE imperial-principality evidence is accepted; Welf partition detail remains out of scope. |
| Brunswick-Wolfenbuttel | 1269..1806 | needs_relation_model | The source says subdivision of Brunswick-Luneburg, but current import only supports HRE parentage. |
| Duchy of Amalfi | 839..958 | needs_relation_model | Byzantine vassalage evidence is accepted; no clean kingdom-rank intermediate is available. |
| Duchy of Anhalt | 1867..1870 | needs_relation_model | North German Confederation state membership is accepted; Prussia bridge is a false positive. |
| Duchy of Anhalt | 1863..1866 | needs_relation_model | German Confederation membership is accepted; Bavaria bridge is a false positive. |
| Duchy of Anhalt | 1871..1918 | needs_relation_model | German Empire state membership is accepted; Bavaria bridge is a false positive. |
| Duchy of Austria | 1156..1453 | needs_relation_model | HRE principality evidence is accepted; Bavarian detachment and Habsburg lands need relation modeling. |
| Duchy of Bavaria | 962..1805 | needs_relation_model | HRE state evidence is accepted; stem-duchy, electorate, and partition semantics remain unmodeled. |
| Duchy of Berg | 1380..1806 | needs_relation_model | HRE state evidence is weak but accepted; Julich-Cleves-Berg layers remain unimported. |
| Duchy of Bohemia | 1002..1198 | needs_relation_model | HRE principality evidence is accepted; kingdom elevation context remains a transition relation. |

## Resulting Queue State

| Status | Rows |
|---|---:|
| reviewed rows | 10 |
| importable replacement facts | 0 |
| relation-model blockers | 10 |

## Metadata Learning

The next high-priority rows further confirm that rank-skip review is now a
relation-semantics problem. Some accepted text sources already name useful
non-parentage relations: subdivision, vassalage, confederation membership,
federal-state membership, imperial principality, and rank elevation.

## Next Step

Stop treating the high-priority queue as a direct parentage import stream. The
next major milestone should define relation facts and query behavior for these
non-tree relations, then replay reviewed blockers into that model.
