# DUCHY Parentage Rank Skip Bridge Cluster Active Reconciliation

source_clusters: `data/staging/parentage-rank-skip-bridge-clusters.tsv`
review_queue: `data/staging/parentage-rank-skip-bridge-cluster-review.tsv`
italy_split: `data/staging/kingdom-italy-hre-child-split-review.tsv`
replacement_packet: `fixtures/first-real.facts`

## Purpose

Reconcile the completed bridge-cluster review queue after the Kingdom of Italy
replacement parentage packet changed the active graph. Superseded parentage
facts remain in accepted fixtures for audit, but active bridge/rank-skip reports
exclude them.

## Active Bridge Surface

| Metric | Count |
|---|---:|
| active bridge rows | 160 |
| active bridge clusters | 20 |
| clustered children | 159 |
| high priority bridge rows | 148 |
| medium priority bridge rows | 9 |
| low priority bridge rows | 3 |

## Review Queue State

| Review Disposition | Rows |
|---|---:|
| same_parent_sibling_false_positive | 19 |
| mixed_cluster_requires_child_split | 1 |
| pending_review | 0 |
| ready_for_packet | 0 |

The completed review queue remains closed. The regenerated active bridge
surface still contains packet-planning leads, but no reviewed cluster row is
ready for direct import.

## Kingdom of Italy / HRE Reconciliation

The active Kingdom of Italy / Holy Roman Empire bridge cluster now has 29
children instead of the earlier 31 because two child-level replacement facts
were accepted:

| Child | Result |
|---|---|
| March of Turin | accepted replacement parentage under Kingdom of Italy for `964..1091`; direct HRE parentage superseded |
| March of Tuscany | accepted replacement parentage under Kingdom of Italy for `962..1197`; direct HRE parentage superseded |

The remaining active Italy/HRE split rows are:

| Split Status | Active Rows |
|---|---:|
| same_parent_sibling_false_positive | 28 |
| candidate_child_review_lead | 1 |

The sole remaining lead is Prince-Bishopric of Trent. It remains held for
relation modeling because the current accepted source supports HRE
estate/state semantics, not accepted child parentage under the Kingdom of
Italy.

## Next Queue

The next scaling milestone should not reopen the closed bridge-cluster review
queue. Use the active rank-skip queue (`220` rows) to choose new source-backed
intermediate-parent packets, prioritizing rows where independent child-level
sources can replace direct empire/crown parentage without creating overlapping
active parents.
