# DUCHY County Rank-Skip Review Batch 02

source_queue: `data/staging/parentage-rank-skip-targets.tsv`
review_tsv: `data/staging/county-rank-skip-review-batch-02.tsv`

## Boundary

- This is not an import packet.
- The reviewed rows are the next ten high-priority county-rank parentage edges
  after batch 01.
- The review checks whether current accepted evidence supports an immediate
  parent replacement, or whether the row should remain blocked.

## Summary

| Disposition | Rows |
|---|---:|
| needs_child_level_intermediate_source | 3 |
| needs_relation_model | 4 |
| needs_split_control_relation_model | 2 |
| rank_identity_mismatch_blocker | 1 |

## Decision

No new parentage facts are importable from this batch.

The generated bridge candidates remain useful packet-planning signals, but all
bridge candidates in this batch are false positives from shared HRE parentage.
Duchy of Bavaria, Anhalt-Bernburg, and Margraviate of Brandenburg are not
importable immediate parents for these rows from current evidence.

## Row Notes

| Child | Span | Disposition | Note |
|---|---:|---|---|
| County of Holland | 1483..1795 | needs_split_control_relation_model | Burgundian Netherlands replacement covers 1433..1482; post-1482 successor context remains unmodeled. |
| County of Luxembourg | 963..1354 | needs_child_level_intermediate_source | Lotharingian or dynastic intermediates require child-level evidence. |
| County of Montbeliard | 1042..1793 | needs_relation_model | Princely county / imperial-state semantics need a relation type distinct from direct parentage. |
| County of Namur | 981..1420 | needs_child_level_intermediate_source | Early regional or dynastic layers need child-level evidence. |
| County of Namur | 1483..1795 | needs_split_control_relation_model | Burgundian Netherlands replacement covers 1421..1482; later successor context remains unmodeled. |
| County of Nassau | 1160..1806 | needs_relation_model | Imperial-state, partition, and princely-rank semantics remain out of parentage. |
| County of Ravensberg | 1140..1806 | needs_child_level_intermediate_source | Jülich-Cleves-Berg or Brandenburg-Prussian transitions need bounded source review. |
| County of Savoy | 1003..1416 | needs_relation_model | HRE state evidence is accepted; Burgundian kingdom or imperial-vassal semantics need a model. |
| County of Wurttemberg | 1083..1495 | needs_relation_model | HRE territory evidence is accepted; Swabia and successor-rank layers remain out of scope. |
| Duchy of Guelders | 1096..1795 | rank_identity_mismatch_blocker | Label and accepted rank disagree; review rank identity before parentage replacement. |

## Resulting Queue State

| Status | Rows |
|---|---:|
| reviewed rows | 10 |
| importable replacement facts | 0 |
| blocked or relation-model rows | 10 |

## Metadata Learning

`title-q152420` is a rank-identity blocker: the accepted name is Duchy of
Guelders while the accepted rank fact is `County`. That may be historically
reasonable for the earliest span, but the current single rank fact is too coarse
for the title's later duchy identity. Future scaling should flag label/rank
drift before assigning rows to county-only agents.

## Next Step

Continue county rank-skip review batches, but start a separate work package for
relation modeling. The repeated blockers now point to at least four relation
types DUCHY needs before it can answer duchy-tree questions cleanly: imperial
state/estate, composite-crown component, split-control fief, and successor or
rank-transition identity.
