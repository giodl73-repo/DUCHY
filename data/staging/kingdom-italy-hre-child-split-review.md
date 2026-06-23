# DUCHY Kingdom of Italy / HRE Child Split Review

source_row: `title-q838931` Kingdom of Italy -> `title-q12548` Holy Roman Empire
source_tsv: `data/staging/parentage-rank-skip-bridge-cluster-review.tsv`
split_tsv: `data/staging/kingdom-italy-hre-child-split-review.tsv`

## Boundary

- This is not an import packet.
- The original bridge cluster remains blocked because row-level inference would
  promote false child-to-candidate parentage.
- Replacement parentage may only be proposed from child-level source review.

## Counts

| Split Status | Rows |
|---|---:|
| candidate_child_review_lead | 3 |
| same_parent_sibling_false_positive | 28 |

## Candidate Child Review Leads

| Child | Current Fact | Required Evidence | Reason |
|---|---|---|---|
| March of Turin | `fact-q426488-parent-q12548` | source-backed child parentage span | Medieval Italian march tied to kings of Italy / HRE context. |
| March of Tuscany | `fact-q1867844-parent-q12548` | source-backed child parentage span | Described as a march of the Kingdom of Italy and Holy Roman Empire. |
| Prince-Bishopric of Trent | `fact-q1231403-parent-q12548` | source-backed child parentage span | Italian-region ecclesiastical principality and HRE constituent state; Imperial Italy relation needs exact review. |

## Negative Split

The remaining 28 children are classified as same-parent sibling false positives:
German stem duchies, Low Countries territories, Lotharingian territories,
German electorates, German/Rhenish prince-bishoprics, and German marches. They
share the Holy Roman Empire parent but are not child-to-candidate parentage under
the Kingdom of Italy.

## Sources Checked

- Kingdom of Italy (Holy Roman Empire): constituent HRE kingdom and Imperial
  Italy context.
- March of Turin: medieval Italian march formed under kings of Italy, later in
  HRE context.
- March of Tuscany: march of the Kingdom of Italy and Holy Roman Empire.
- Prince-Bishopric of Trent: HRE ecclesiastical principality / constituent
  state; exact Imperial Italy relation remains a review question.

## Decision

`blocked_child_split_review`

The cluster is now split for review. No packet stubs should be emitted from the
row until one of the three lead children receives source-backed parentage with a
bounded span.
