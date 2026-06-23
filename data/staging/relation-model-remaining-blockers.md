# DUCHY Relation Model Remaining Blockers

source_inputs:

- `data/staging/county-rank-skip-review-batch-01.tsv`
- `data/staging/county-rank-skip-review-batch-02.tsv`
- `data/staging/parentage-rank-skip-review-batch-03.tsv`
- `data/staging/parentage-rank-skip-review-batch-04.tsv`
- `data/staging/trent-italy-hre-relation-review.tsv`

## Status

The first seven accepted relation-model packets promote 33 non-parentage
relation facts. The generated relation report now explains 33 of 223 active
rank-skip rows while parentage facts remain unchanged.

Current measured baseline:

| Metric | Value |
|---|---:|
| sources | 532 |
| facts | 1364 |
| parentage facts | 278 |
| relation facts | 33 |
| rank-skip rows | 223 |
| relation-explained rows | 33 |
| unexplained rank-skip rows | 190 |
| temporal parent conflicts | 0 |

## Safe Relation Packets Promoted

| Packet | Relation facts | Main relation kinds |
|---|---:|---|
| `relation-model-batch-01` | 6 | `imperial_state`, `confederation_member`, `federal_state_member` |
| `relation-model-batch-02` | 12 | `imperial_state`, `confederation_member` |
| `relation-model-batch-03` | 7 | `imperial_state`, `vassalage_or_suzerainty` |
| `relation-model-batch-04` | 3 | `subdivision_or_appanage`, `imperial_state` |
| `relation-model-low-countries-01` | 3 | `split_fief_or_control` |
| `barcelona-catalonia-relation-01` | 1 | `composite_crown_component` |
| `relation-model-guelders-01` | 1 | `imperial_state` |

## Held Rows

These rows should not be converted into relation facts without additional
source-custody work or a narrower modeling decision.

| Class | Rows / examples | Reason held |
|---|---|---|
| Structured-data-only context | Comtat Venaissin | Current accepted fact support is not text-backed enough for new relation semantics. Archduchy of Austria post-1804 edges now have text custody, but internal monarchy semantics remain unmodeled. |
| Split-control / successor context | Flanders, Holland, Namur | First Low Countries packet now explains the post-Burgundian current-parent rank skips with bounded `split_fief_or_control` relations; deeper successor parentage remains held. |
| Child-level intermediate source needed | Luxembourg, early Namur, Ravensberg, early Holland | The reviewed note explicitly forbids inferring the proposed intermediate without bounded child-level evidence. Barcelona now has bounded text custody and a current-parent relation explanation, but deeper Principality title modeling remains unimported. |
| Broader relation semantics not yet source-backed | Burgundy/Arles layers, Geneva/Savoyard successor, HRE partitions, Welf partitions | Current accepted relations explain the direct current-parent context, but deeper intermediate structure remains unimported. |

## Next Source-Custody Targets

Prioritize packets that can unlock multiple held rows:

1. Luxembourg/Namur/Ravensberg child-level intermediate source review.
2. Archduchy of Austria internal monarchy/crownland relation modeling for
   1804-1918 rows.
3. Deeper Low Countries successor-state title modeling for Habsburg
   Netherlands, Seventeen Provinces, Dutch Republic, Spanish Netherlands,
   Austrian Netherlands, and Batavian Republic.
4. Deeper Principality of Catalonia title modeling after the Barcelona
   composite-crown relation pass.
5. Burgundy/Arles, Geneva/Savoyard successor, HRE partition, and Welf
   partition relation semantics.

## Boundary

No new relation fact should be added from this remaining queue unless it passes
the same gate as the first five packets:

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

These records initially promoted no facts. `relation-model-low-countries-01`
uses them to promote three bounded `split_fief_or_control` relations for
Flanders, Holland, and Namur to the current HRE parent, while keeping deeper
successor-state title modeling out of scope.

`austria-internal-monarchy-sources-01` adds three accepted Wikimedia text
source records and attaches them to the existing Archduchy of Austria parentage
facts under Austrian Empire and Austria-Hungary:

- Archduchy of Austria crownland/internal monarchy context,
- Austrian Empire,
- Austria-Hungary.

This tightens source custody for the two 1804-1918 Archduchy rows without
adding relation facts or changing parentage structure. Crownland, Cisleithania,
and internal monarchy semantics remain a separate relation-model decision.

`barcelona-catalonia-relation-01` adds three accepted Wikimedia text source
records and promotes one bounded `composite_crown_component` relation for
County of Barcelona under the Crown of Aragon in 1162..1164. This explains the
current-parent rank skip without importing a Principality of Catalonia title or
changing parentage.

`guelders-rank-identity-01` supersedes the erroneous `County` rank for Duchy of
Guelders with a reviewed `Duchy` rank. `relation-model-guelders-01` then
promotes a bounded `imperial_state` relation to the current HRE parent for
1096..1795. Parentage remains unchanged.
