# DUCHY County Rank-Skip Review Batch 01

source_queue: `data/staging/parentage-rank-skip-targets.tsv`
review_tsv: `data/staging/county-rank-skip-review-batch-01.tsv`

## Boundary

- This is not an import packet.
- The reviewed rows are the first ten high-priority county-rank parentage edges
  that skip the expected duchy layer.
- The review checks whether current accepted evidence supports an immediate
  parent replacement or whether the row should remain blocked.

## Summary

| Disposition | Rows |
|---|---:|
| same_parent_sibling_false_positive | 2 |
| needs_child_level_intermediate_source | 3 |
| needs_relation_model | 3 |
| needs_split_control_relation_model | 2 |

## Decision

No new parentage facts are importable from this batch.

The bridge generator is useful for surfacing likely review clusters, but the
first county rows show that many candidate duchy parents are shared-parent
siblings rather than child parents. In particular, Duchy of Bavaria, Duchy of
Ferrara, and Anhalt-Bernburg are not importable intermediates for the reviewed
county rows from current evidence.

## Row Notes

| Child | Span | Disposition | Note |
|---|---:|---|---|
| Burgraviate of Nuremberg | 1105..1440 | same_parent_sibling_false_positive | Keep direct HRE parentage unless a child-level source supports an intermediate. |
| Comtat Venaissin | 1274..1791 | same_parent_sibling_false_positive | Keep Papal States direct parentage; papal enclave semantics need a relation model. |
| County of Barcelona | 1162..1164 | needs_child_level_intermediate_source | Crown of Aragon endpoint is accepted; Catalan intermediate remains unproven. |
| County of Burgundy | 982..1678 | needs_relation_model | HRE context is accepted; Arles/circle/personal-union layers remain out of scope. |
| County of Empuries | 1341..1402 | needs_child_level_intermediate_source | Crown of Aragon apanage is accepted; Principality of Catalonia is only a later annexation context in current evidence. |
| County of Flanders | 962..1383 | needs_split_control_relation_model | French/Imperial fief split remains held. |
| County of Flanders | 1483..1797 | needs_split_control_relation_model | Burgundian Netherlands replacement already covers 1384..1482; post-1482 split remains unmodeled. |
| County of Geneva | 1032..1401 | needs_relation_model | HRE county context is accepted; Arles/Savoy layers need separate review. |
| County of Girona | 800..887 | needs_relation_model | Spanish March/frontier semantics need a relation type before an intermediate can be imported. |
| County of Holland | 962..1432 | needs_child_level_intermediate_source | Burgundian Netherlands replacement starts at 1433; earlier Holland needs direct source review. |

## Resulting Queue State

| Status | Rows |
|---|---:|
| reviewed rows | 10 |
| importable replacement facts | 0 |
| blocked or relation-model rows | 10 |

## Next Step

Continue with source-backed county rank-skip review batches before importing new
intermediates. Reopen an individual row only with a bounded child-level source,
or after DUCHY adds relation types for marches, imperial fiefs, composite
crowns, papal enclaves, and successor institutions distinct from parentage.
