# DUCHY Relation Model Remaining Blockers

source_inputs:

- `data/staging/county-rank-skip-review-batch-01.tsv`
- `data/staging/county-rank-skip-review-batch-02.tsv`
- `data/staging/parentage-rank-skip-review-batch-03.tsv`
- `data/staging/parentage-rank-skip-review-batch-04.tsv`
- `data/staging/trent-italy-hre-relation-review.tsv`

## Status

The first four accepted relation-model packets promote 28 non-parentage
relation facts. The generated relation report now explains 28 of 223 active
rank-skip rows while parentage facts remain unchanged.

Current measured baseline:

| Metric | Value |
|---|---:|
| sources | 526 |
| facts | 1358 |
| parentage facts | 278 |
| relation facts | 28 |
| rank-skip rows | 223 |
| relation-explained rows | 28 |
| unexplained rank-skip rows | 195 |
| temporal parent conflicts | 0 |

## Safe Relation Packets Promoted

| Packet | Relation facts | Main relation kinds |
|---|---:|---|
| `relation-model-batch-01` | 6 | `imperial_state`, `confederation_member`, `federal_state_member` |
| `relation-model-batch-02` | 12 | `imperial_state`, `confederation_member` |
| `relation-model-batch-03` | 7 | `imperial_state`, `vassalage_or_suzerainty` |
| `relation-model-batch-04` | 3 | `subdivision_or_appanage`, `imperial_state` |

## Held Rows

These rows should not be converted into relation facts without additional
source-custody work or a narrower modeling decision.

| Class | Rows / examples | Reason held |
|---|---|---|
| Structured-data-only context | Comtat Venaissin, County of Barcelona, Archduchy of Austria under Austrian Empire/Austria-Hungary | Current accepted fact support is not text-backed enough for new relation semantics. |
| Split-control / successor context | Flanders, Holland, Namur | Existing notes call for French, Habsburg, Dutch Republic, Batavian, Spanish/Austrian, or successor-state review before relation promotion. |
| Child-level intermediate source needed | Barcelona, Luxembourg, early Namur, Ravensberg, early Holland | The reviewed note explicitly forbids inferring the proposed intermediate without bounded child-level evidence. |
| Rank identity blocker | Duchy of Guelders | Accepted title name and rank disagree; fix rank identity before relation or parentage changes. |
| Broader relation semantics not yet source-backed | Burgundy/Arles layers, Geneva/Savoyard successor, HRE partitions, Welf partitions | Current accepted relations explain the direct current-parent context, but deeper intermediate structure remains unimported. |

## Next Source-Custody Targets

Prioritize packets that can unlock multiple held rows:

1. Flanders/Holland/Namur split-control and successor-state review.
2. Archduchy of Austria internal monarchy/crownland text custody for
   1804-1918 rows.
3. Barcelona/Empuries/Catalonia relation source review for Catalan Counties or
   Principality of Catalonia.
4. Guelders rank-identity review.
5. Luxembourg/Namur/Ravensberg child-level intermediate source review.

## Boundary

No new relation fact should be added from this remaining queue unless it passes
the same gate as the first four packets:

- source-backed subject and related title exist in accepted fixtures,
- relation points to the reviewed current parent or a separately reviewed
  related title,
- relation kind matches the reviewed wording,
- `duchy-promote --dry-run` and `duchy-promote --apply` validate the merged
  fixture,
- `parentage-rank-skip-relation-report` and `parentage-graph-report` are
  regenerated.

## Source-Custody Progress

`low-countries-successor-sources-01` adds six accepted source records for the
next Low Countries review pass:

- Habsburg Netherlands,
- Seventeen Provinces,
- Spanish Netherlands,
- Austrian Netherlands,
- Dutch Republic,
- Batavian Republic.

These records intentionally promote no facts yet. They provide narrower
source-custody input for later split-control and successor-state relation
packets covering Flanders, Holland, and Namur.
