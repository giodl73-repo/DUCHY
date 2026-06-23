# DUCHY Relation Model Remaining Blockers

source_inputs:

- `data/staging/county-rank-skip-review-batch-01.tsv`
- `data/staging/county-rank-skip-review-batch-02.tsv`
- `data/staging/parentage-rank-skip-review-batch-03.tsv`
- `data/staging/parentage-rank-skip-review-batch-04.tsv`
- `data/staging/trent-italy-hre-relation-review.tsv`

## Status

The first twenty accepted relation-model packets promote 104 non-parentage
relation facts. The generated relation report explains 100 of 231 active
rank-skip rows while parentage facts remain unchanged. Relation facts now allow
multiple simultaneous contexts for the same title and span, so a title can be
both an imperial state and a reviewed subdivision or appanage when sources
support both claims.

Current measured baseline:

| Metric | Value |
|---|---:|
| sources | 540 |
| facts | 1467 |
| titles | 356 |
| parentage facts | 289 |
| relation facts | 104 |
| rank-skip rows | 231 |
| relation-explained rows | 100 |
| unexplained rank-skip rows | 131 |
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
| `relation-model-low-countries-hre-01` | 4 | `imperial_state` |
| `relation-model-austria-crownland-01` | 2 | `crownland_component` |
| `low-countries-successor-parentage-01` | 7 | `composite_crown_component` |
| `namur-successor-parentage-01` | 4 | `composite_crown_component` |
| `brunswick-wolfenbuttel-welf-relation-01` | 1 | `subdivision_or_appanage` |
| `duchy-savoy-hre-relation-01` | 1 | `imperial_state` |
| `hre-relation-batch-05` | 2 | `imperial_state`, `vassalage_or_suzerainty` |
| `hre-relation-batch-06` | 7 | `imperial_state`, `vassalage_or_suzerainty` |
| `hre-relation-batch-07` | 9 | `imperial_state`, `vassalage_or_suzerainty` |
| `hre-relation-batch-08` | 9 | `imperial_state`, `vassalage_or_suzerainty` |
| `hre-electorate-relation-01` | 8 | `imperial_state` |
| `hre-landgraviate-margraviate-relation-01` | 7 | `imperial_state`, `vassalage_or_suzerainty` |
| `hre-relation-batch-09` | 10 | `imperial_state`, `confederation_member`, `federal_state_member`, `vassalage_or_suzerainty` |

## Held Rows

These rows should not be converted into relation facts without additional
source-custody work or a narrower modeling decision.

| Class | Rows / examples | Reason held |
|---|---|---|
| Structured-data-only context | Comtat Venaissin | Current accepted fact support is not text-backed enough for new relation semantics. Archduchy of Austria post-1804 edges now have text custody and bounded crownland-component relations. |
| Split-control / successor context | Flanders, Holland, Namur | First successor parentage packets now replace the post-Burgundian HRE current-parent spans with bounded successor title edges and matching `composite_crown_component` relations. |
| Child-level intermediate source needed | Luxembourg, early Namur, Ravensberg, early Holland | Current-parent `imperial_state` relations now explain these rank skips, but the reviewed note still forbids inferring deeper intermediate parentage without bounded child-level evidence. |
| Broader relation semantics not yet source-backed | Burgundy/Arles layers, Geneva/Savoyard successor, HRE partitions, deeper Welf partitions | Current accepted relations explain the direct current-parent context, and Brunswick-Wolfenbuttel now carries Welf subdivision context, but deeper intermediate structure remains unimported. |

## Next Source-Custody Targets

Prioritize packets that can unlock multiple held rows:

1. Deeper Low Countries successor-state title modeling for Habsburg
   Netherlands, Seventeen Provinces, Dutch Republic, Spanish Netherlands,
   Austrian Netherlands, and Batavian Republic.
2. Catalonia edge modeling only after a bounded child-level source overlaps an
   accepted title span.
3. Burgundy/Arles, Geneva/Savoyard successor, HRE partition, and Welf
   partition relation semantics.
4. Deeper Luxembourg/Namur/Ravensberg/early Holland intermediate parentage only
   after bounded child-level evidence is available.

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

These records initially promoted no facts. `low-countries-successor-titles-01`
then materializes six Crown-rank title identities for Habsburg Netherlands,
Seventeen Provinces, Spanish Netherlands, Austrian Netherlands, Dutch Republic,
and Batavian Republic. `relation-model-low-countries-01` uses the accepted text
sources to promote three bounded `split_fief_or_control` relations for
Flanders, Holland, and Namur to the current HRE parent, while keeping successor
parentage edges separate. `low-countries-successor-parentage-01` then connects
Flanders and Holland to reviewed successor title nodes with seven bounded
parentage facts and seven matching `composite_crown_component` relation facts.
`namur-successor-parentage-01` adds a narrowed Namur successor source record and
connects Namur to Habsburg Netherlands, Seventeen Provinces, Spanish
Netherlands, and Austrian Netherlands with four bounded parentage facts and
four matching relation facts.

`austria-internal-monarchy-sources-01` adds three accepted Wikimedia text
source records and attaches them to the existing Archduchy of Austria parentage
facts under Austrian Empire and Austria-Hungary:

- Archduchy of Austria crownland/internal monarchy context,
- Austrian Empire,
- Austria-Hungary.

This tightened source custody for the two 1804-1918 Archduchy rows without
changing parentage structure. `relation-model-austria-crownland-01` then
promotes two bounded `crownland_component` relations to the current Austrian
Empire and Austria-Hungary parents. Cisleithanian administrative detail and
modern successor claims remain out of scope.

`barcelona-catalonia-relation-01` adds three accepted Wikimedia text source
records and promotes one bounded `composite_crown_component` relation for
County of Barcelona under the Crown of Aragon in 1162..1164. This explains the
current-parent rank skip without importing a Principality of Catalonia title or
changing parentage.

`principality-catalonia-title-01` materializes Principality of Catalonia as a
Duchy-rank title for 1173..1714. It intentionally does not replace the accepted
County of Barcelona -> Crown of Aragon parentage because the Barcelona title
span ends in 1164 in the current fixture, before the Principality title span
starts.

`guelders-rank-identity-01` supersedes the erroneous `County` rank for Duchy of
Guelders with a reviewed `Duchy` rank. `relation-model-guelders-01` then
promotes a bounded `imperial_state` relation to the current HRE parent for
1096..1795. Parentage remains unchanged.

`relation-model-low-countries-hre-01` promotes four bounded `imperial_state`
relations to the current HRE parent for early Holland, County of Luxembourg,
early Namur, and County of Ravensberg. These explain rank skips without
importing Lower Lotharingia, Ardennes, Jülich-Cleves-Berg, or successor
parentage.

`brunswick-wolfenbuttel-welf-relation-01` promotes one bounded
`subdivision_or_appanage` relation from Brunswick-Wolfenbuttel to
Brunswick-Luneburg for 1269..1806. Parentage remains under the Holy Roman
Empire because the current rank policy rejects Duchy-to-Duchy parent edges, and
the existing `imperial_state` relation remains valid as a simultaneous context.

`duchy-savoy-hre-relation-01` promotes one bounded `imperial_state` relation
from Duchy of Savoy to the Holy Roman Empire for 1416..1806. This explains the
current-parent rank skip without importing County of Savoy predecessor detail,
French occupation periods, Sardinian transition, or territorial inventory.

`hre-relation-batch-05` promotes two bounded relations against the current HRE
parent: Duchy of Pomerania as `vassalage_or_suzerainty` for 1121..1637 and
Duchy of Saxony as `imperial_state` for 962..1296. Both remain relation-only
because the accepted source notes exclude partition, successor, and dynastic
detail.

`hre-relation-batch-06` promotes seven bounded relations against the current HRE
parent for Holstein, Julich, Lorraine, Mecklenburg-Schwerin, Mirandola,
Augsburg, and Basel. Mirandola uses `vassalage_or_suzerainty` because the
accepted note says imperial suzerainty; the others use `imperial_state` from
state/principality wording. All remain relation-only and preserve the accepted
parentage tree.

`hre-relation-batch-07` promotes nine more bounded relations against the current
HRE parent. Carniola, Luxembourg, Cleves, Freising, Liege, Minden, and Munster
use `imperial_state` from state/estate/principality wording; Modena and Reggio
and Parma and Piacenza use `vassalage_or_suzerainty` from imperial fief or
imperial protection wording.

`hre-relation-batch-08` promotes nine bounded current-parent relation contexts.
Bremen-Verden and Mantua use `vassalage_or_suzerainty` from immediate fief or
imperial elevation wording; Wurttemberg, Westphalia, Osnabruck, Paderborn,
Strasbourg, Utrecht, and Verdun use `imperial_state` from state, territory, or
ecclesiastical-principality wording.

`hre-electorate-relation-01` promotes eight bounded electorate relations to the
current HRE parent. Bavaria and Mainz are narrowed to materialized title overlap
where source wording or the subject title outlives the accepted HRE or
electorate identity spans.

`hre-landgraviate-margraviate-relation-01` promotes seven bounded relations for
landgraviates and margraviates. Brabant, Austria, and Meissen use
`vassalage_or_suzerainty` from fief/frontier-march wording; Hesse,
Hesse-Darmstadt, Hesse-Kassel, and Brandenburg use `imperial_state` from
state/principality wording.

`hre-relation-batch-09` promotes ten residual bounded relation contexts from
already accepted source-custody rows. Tuscany and Florence use
`vassalage_or_suzerainty` from imperial-fief and imperial-determination
wording; Toul, Lippe, Salzburg, and Saxe-Meiningen use `imperial_state` for
Holy Roman Empire spans; Lippe, Nassau, and Saxe-Meiningen use
`confederation_member` for German Confederation spans; and Saxe-Meiningen uses
`federal_state_member` for its German Empire span.
