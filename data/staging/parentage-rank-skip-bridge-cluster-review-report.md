# DUCHY Parentage Rank Skip Bridge Cluster Review Report

source_tsv: data/staging/parentage-rank-skip-bridge-cluster-review.tsv
review_rows: 20
pending_review: 0
not_inferred: 0
high_priority_rows: 148
medium_priority_rows: 9
low_priority_rows: 3

## Boundary

- Review rows are not import-ready parentage claims.
- `not_inferred` rows must remain blocked until child-to-candidate evidence is reviewed.
- Status or disposition changes should be made in review TSVs, then regenerated into this report.

## Status Counts

| Status | Rows |
|---|---:|
| reviewed | 20 |

## Disposition Counts

| Disposition | Rows |
|---|---:|
| mixed_cluster_requires_child_split | 1 |
| same_parent_sibling_false_positive | 19 |

## Evidence Counts

| Evidence Needed | Rows |
|---|---:|
| child_split_review_required | 1 |
| none | 19 |

## Review Rows

| Status | Disposition | Candidate Parent | Current Parent | Children | High | Medium | Low | Evidence Needed |
|---|---|---|---|---:|---:|---:|---:|---|
| reviewed | same_parent_sibling_false_positive | Kingdom of Bohemia | Holy Roman Empire | 49 | 49 | 0 | 0 | none |
| reviewed | mixed_cluster_requires_child_split | Kingdom of Italy | Holy Roman Empire | 29 | 29 | 0 | 0 | child_split_review_required |
| reviewed | same_parent_sibling_false_positive | Kingdom of Bavaria | German Confederation | 27 | 27 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Duchy of Bavaria | Holy Roman Empire | 12 | 12 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Bavaria | German Empire | 8 | 8 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Prussia | North German Confederation | 6 | 6 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Bohemia | Austria-Hungary | 4 | 4 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Crown of the Kingdom of Poland | Polish-Lithuanian Commonwealth | 4 | 0 | 4 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Confederation of the Rhine | First French Empire | 3 | 0 | 0 | 3 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Westphalia | Confederation of the Rhine | 3 | 0 | 3 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Anhalt-Bernburg | Holy Roman Empire | 2 | 3 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Bohemia | Austrian Empire | 2 | 2 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Italy | First French Empire | 2 | 2 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Duchy of Brittany | Kingdom of France | 2 | 0 | 2 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Archduchy of Austria | Austria-Hungary | 1 | 1 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Archduchy of Austria | Austrian Empire | 1 | 1 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Duchy of Ferrara | Papal States | 1 | 1 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Eastern Hungarian Kingdom | Ottoman Empire | 1 | 1 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Kingdom of Imereti | Russian Empire | 1 | 1 | 0 | 0 | none |
| reviewed | same_parent_sibling_false_positive | Margraviate of Brandenburg | Holy Roman Empire | 1 | 1 | 0 | 0 | none |
