# DUCHY Parentage Rank Skip Bridge Cluster Review Report

source_tsv: data/staging/parentage-rank-skip-bridge-cluster-review.tsv
review_rows: 20
pending_review: 20
not_inferred: 20
high_priority_rows: 150
medium_priority_rows: 9
low_priority_rows: 3

## Boundary

- Review rows are not import-ready parentage claims.
- `not_inferred` rows must remain blocked until child-to-candidate evidence is reviewed.
- Status or disposition changes should be made in review TSVs, then regenerated into this report.

## Status Counts

| Status | Rows |
|---|---:|
| pending_review | 20 |

## Disposition Counts

| Disposition | Rows |
|---|---:|
| not_inferred | 20 |

## Evidence Counts

| Evidence Needed | Rows |
|---|---:|
| child_to_candidate_parentage_source | 20 |

## Review Rows

| Status | Disposition | Candidate Parent | Current Parent | Children | High | Medium | Low | Evidence Needed |
|---|---|---|---|---:|---:|---:|---:|---|
| pending_review | not_inferred | Kingdom of Bohemia | Holy Roman Empire | 49 | 49 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Italy | Holy Roman Empire | 31 | 31 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Bavaria | German Confederation | 27 | 27 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Duchy of Bavaria | Holy Roman Empire | 12 | 12 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Bavaria | German Empire | 8 | 8 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Prussia | North German Confederation | 6 | 6 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Bohemia | Austria-Hungary | 4 | 4 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Crown of the Kingdom of Poland | Polish-Lithuanian Commonwealth | 4 | 0 | 4 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Confederation of the Rhine | First French Empire | 3 | 0 | 0 | 3 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Westphalia | Confederation of the Rhine | 3 | 0 | 3 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Anhalt-Bernburg | Holy Roman Empire | 2 | 3 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Bohemia | Austrian Empire | 2 | 2 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Italy | First French Empire | 2 | 2 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Duchy of Brittany | Kingdom of France | 2 | 0 | 2 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Archduchy of Austria | Austria-Hungary | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Archduchy of Austria | Austrian Empire | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Duchy of Ferrara | Papal States | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Eastern Hungarian Kingdom | Ottoman Empire | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Kingdom of Imereti | Russian Empire | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
| pending_review | not_inferred | Margraviate of Brandenburg | Holy Roman Empire | 1 | 1 | 0 | 0 | child_to_candidate_parentage_source |
