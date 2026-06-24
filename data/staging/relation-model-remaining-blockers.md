# DUCHY Relation Model Remaining Blockers

source_inputs:

- `data/staging/county-rank-skip-review-batch-01.tsv`
- `data/staging/county-rank-skip-review-batch-02.tsv`
- `data/staging/parentage-rank-skip-review-batch-03.tsv`
- `data/staging/parentage-rank-skip-review-batch-04.tsv`
- `data/staging/trent-italy-hre-relation-review.tsv`

## Status

The first fifty-two accepted relation-model packets promote 282 non-parentage
relation facts. The generated relation report explains all 232 active
rank-skip rows after the Catalonia and Low Countries crown parentage edges
added new explained rank-skip rows. Relation facts now allow
multiple simultaneous contexts for the same title and span, so a title can be
both an imperial state and a reviewed subdivision or appanage when sources
support both claims. `great-britain-ukgbi-relation-01` also records the 1801
Great Britain to United Kingdom of Great Britain and Ireland transition as a
relation-only packet because the parentage validator correctly rejects
Crown-to-Crown parentage. `british-isles-union-component-relation-01` then
adds matching composite-component relation context for the already accepted
England, Scotland, and Ireland endpoint parentage facts.
`kalmar-union-component-relation-01` adds text-backed component relation
context for accepted Sweden, Denmark, and Norway parentage under the Kalmar
Union. `commonwealth-polish-crown-component-relation-01` adds matching
component relation context for the accepted Crown of the Kingdom of Poland
parentage under the Polish-Lithuanian Commonwealth.
`poland-commonwealth-endpoint-relation-01` adds endpoint rank-transition
context for accepted Kingdom of Poland boundary-year parentage into the
Commonwealth. `hispanic-monarchy-component-relation-01` adds text-backed
component relation context for the accepted Crowns of Aragon and Castile
parentage under the Hispanic Monarchy. `prussia-polish-crown-relation-01`
adds matching vassalage/suzerainty context for the accepted Duchy of Prussia
parentage under the Crown of the Kingdom of Poland.
`rhine-confederation-french-empire-relation-01` adds matching
vassalage/suzerainty context for the accepted Confederation of the Rhine
parentage under the First French Empire. `athens-thessalonica-relation-01`
adds matching vassalage/suzerainty context for the accepted Duchy of Athens
parentage under the Kingdom of Thessalonica. `normandy-france-relation-01`
adds matching vassalage/suzerainty context for the accepted Duchy of Normandy
parentage under the Kingdom of France. `bohemian-silesian-vassal-relation-01`
adds four matching vassalage/suzerainty contexts for accepted Bohemian
Silesian duchy parentage. `sicily-crown-aragon-relation-01` adds matching
component context for accepted Kingdom of Sicily parentage under the Crown of
Aragon. `naples-crown-aragon-relation-01` adds matching component context for
accepted Kingdom of Naples parentage under the Crown of Aragon.
`aragon-component-relation-01` then closes the Crown of Aragon component set by
adding matching component context for accepted Kingdom of Aragon, Kingdom of
Valencia, Kingdom of Majorca, Kingdom of Sardinia, and County of Empuries
parentage under the Crown of Aragon. `castile-component-relation-01` mirrors
that packet for the second great Iberian composite crown, adding component
context for accepted Kingdom of Castile, Kingdom of Leon, Kingdom of Galicia,
Kingdom of Toledo, Kingdom of Murcia, Kingdom of Jaen, and Kingdom of Granada
parentage under the Crown of Castile.
`burgundian-netherlands-component-relation-01` fills the Burgundian Netherlands
node skipped in each Low Countries county's composite-crown successor chain,
adding component context for accepted Flanders, Holland, and Namur parentage
under the Burgundian Netherlands. `french-crown-vassal-relation-01` mirrors the
Normandy packet for the major French crown vassals, adding vassalage/suzerainty
context for accepted Duchy of Burgundy, Duchy of Brittany, Duchy of Aquitaine,
and Duchy of Gascony parentage under the Kingdom of France.

Current measured baseline:

| Metric | Value |
|---|---:|
| sources | 588 |
| facts | 1650 |
| titles | 356 |
| parentage facts | 294 |
| relation facts | 282 |
| rank-skip rows | 232 |
| relation-explained rows | 232 |
| unexplained rank-skip rows | 0 |
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
| `german-state-relation-batch-01` | 8 | `imperial_state`, `confederation_member`, `federal_state_member` |
| `german-state-relation-batch-02` | 12 | `imperial_state`, `confederation_member`, `federal_state_member` |
| `german-state-relation-batch-03` | 12 | `imperial_state`, `confederation_member` |
| `hre-relation-batch-10` | 8 | `imperial_state` |
| `german-structured-bridge-relation-01` | 9 | `confederation_member`, `federal_state_member` |
| `hre-relation-batch-11` | 3 | `imperial_state`, `confederation_member` |
| `german-structured-bridge-relation-02` | 3 | `confederation_member`, `federal_state_member` |
| `vassal-satellite-relation-batch-01` | 9 | `vassalage_or_suzerainty` |
| `napoleonic-client-relation-batch-01` | 7 | `confederation_member`, `vassalage_or_suzerainty` |
| `vassal-client-relation-batch-02` | 4 | `vassalage_or_suzerainty` |
| `endpoint-transition-relation-batch-01` | 4 | `rank_transition` |
| `endpoint-transition-relation-batch-02` | 3 | `rank_transition`, `subdivision_or_appanage` |
| `commonwealth-union-subdivision-relation-01` | 6 | `vassalage_or_suzerainty`, `composite_crown_component`, `subdivision_or_appanage` |
| `german-polish-client-relation-01` | 5 | `confederation_member`, `federal_state_member`, `vassalage_or_suzerainty` |
| `administrative-dependency-relation-01` | 6 | `subdivision_or_appanage`, `vassalage_or_suzerainty` |
| `austrian-monarchy-component-relation-01` | 16 | `crownland_component`, `composite_crown_component` |
| `residual-rank-skip-relation-01` | 17 | `subdivision_or_appanage`, `split_fief_or_control`, `vassalage_or_suzerainty`, `imperial_state`, `composite_crown_component`, `rank_transition` |
| `catalonia-crown-aragon-parentage-01` | 1 | `composite_crown_component` |
| `low-countries-crown-hre-parentage-01` | 4 | `composite_crown_component` |
| `great-britain-ukgbi-relation-01` | 1 | `composite_crown_component` |
| `british-isles-union-component-relation-01` | 3 | `composite_crown_component` |
| `kalmar-union-component-relation-01` | 3 | `composite_crown_component` |
| `commonwealth-polish-crown-component-relation-01` | 1 | `composite_crown_component` |
| `poland-commonwealth-endpoint-relation-01` | 2 | `rank_transition` |
| `hispanic-monarchy-component-relation-01` | 2 | `composite_crown_component` |
| `prussia-polish-crown-relation-01` | 1 | `vassalage_or_suzerainty` |
| `rhine-confederation-french-empire-relation-01` | 1 | `vassalage_or_suzerainty` |
| `athens-thessalonica-relation-01` | 1 | `vassalage_or_suzerainty` |
| `normandy-france-relation-01` | 1 | `vassalage_or_suzerainty` |
| `bohemian-silesian-vassal-relation-01` | 4 | `vassalage_or_suzerainty` |
| `sicily-crown-aragon-relation-01` | 1 | `composite_crown_component` |
| `naples-crown-aragon-relation-01` | 1 | `composite_crown_component` |
| `aragon-component-relation-01` | 5 | `composite_crown_component` |
| `castile-component-relation-01` | 7 | `composite_crown_component` |
| `burgundian-netherlands-component-relation-01` | 3 | `composite_crown_component` |
| `french-crown-vassal-relation-01` | 4 | `vassalage_or_suzerainty` |

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

`german-state-relation-batch-01` promotes eight bounded German state
membership contexts. Hohenzollern-Hechingen and Hohenzollern-Sigmaringen use
`imperial_state` for Holy Roman Empire spans and `confederation_member` for
German Confederation spans; Electorate of Hesse uses `confederation_member`;
and Oldenburg uses `confederation_member` for German and North German
Confederation spans plus `federal_state_member` for its German Empire span.

`german-state-relation-batch-02` promotes twelve additional bounded German
state membership contexts. Grand Duchy of Hesse, Mecklenburg-Schwerin,
Mecklenburg-Strelitz, Saxe-Altenburg, Saxe-Weimar-Eisenach, and Saxe-Lauenburg
use the same relation vocabulary for accepted German Confederation, North
German Confederation, German Empire, and Holy Roman Empire spans without
changing parentage.

`german-state-relation-batch-03` promotes twelve more source-backed relation
contexts for Saxe-Coburg and Gotha, Saxe-Coburg-Saalfeld,
Saxe-Gotha-Altenburg, Saxe-Hildburghausen, Schaumburg-Lippe,
Schwarzburg-Rudolstadt, and Schwarzburg-Sondershausen. The packet stays on
active current-parent rows and does not import extra North German
Confederation, German Empire, Weimar Republic, or Confederation of the Rhine
contexts that sources mention but current parentage does not yet model.

`hre-relation-batch-10` promotes eight residual HRE `imperial_state` relation
contexts from accepted text-backed rows: Kingdom of Bohemia, Free Imperial City
of Aachen, Hesse-Marburg, Lower Lotharingia, Mecklenburg-Gustrow, two
Nassau-Siegen spans, and Palatinate-Sulzbach. Lower Alsace and Austrian
structured-only rows remain unpromoted.

`german-structured-bridge-relation-01` promotes nine relation contexts from
accepted structured bridge reviews. It is limited to source records whose
review notes explicitly allow bridge relations: German Confederation or German
Empire relations for Bavaria, Wurttemberg, Baden, Hanover, Prussia, and Saxony.

`hre-relation-batch-11` promotes three residual current-parent relation
contexts: Holstein and Saxe-Lauenburg as German Confederation members for
1815..1866, and Swabia as a Holy Roman Empire imperial state for 962..1313.
The packet reuses already accepted source custody and leaves parentage
unchanged.

`german-structured-bridge-relation-02` promotes the three reviewed Duchy of
Brunswick bridge contexts authorized by accepted source custody: German
Confederation, North German Confederation, and German Empire membership for the
active 1815..1918 title span.

`vassal-satellite-relation-batch-01` promotes nine bounded
`vassalage_or_suzerainty` contexts from accepted text-backed rows: Duchy of
Warsaw under the First French Empire, Imereti under the Russian Empire, Moscow
and Vladimir under the Golden Horde, and five Ottoman vassal/suzerainty rows
for Eastern Hungarian Kingdom, Moravian Serbia, Prince-Bishopric of Montenegro,
Principality of Serbia, and the United Principalities.

`napoleonic-client-relation-batch-01` promotes seven source-backed relation
contexts: Berg, Frankfurt, and Wurzburg as Confederation of the Rhine member
contexts, Holland, Etruria, and Napoleonic Italy as First French Empire
client/suzerainty contexts, and the 1918 Duchy of Courland and Semigallia as a
German Empire client/suzerainty context.

`vassal-client-relation-batch-02` promotes four bounded
`vassalage_or_suzerainty` contexts for source-backed client, protectorate,
personal-union, and vassal wording: Albanian Kingdom and Italian protectorate
of Albania under the Italian Empire, Kingdom of Livonia under the Tsardom of
Russia, and Kingdom of Thessalonica under the Latin Empire.

`endpoint-transition-relation-batch-01` promotes four endpoint
`rank_transition` contexts from accepted text-backed rows: County of Sicily to
Kingdom of Sicily, Polish-Lithuanian union to the Polish-Lithuanian
Commonwealth, Duchy of Ferrara to direct Papal States rule, and Kingdom of
Bosnia to Ottoman conquest/fall.

`endpoint-transition-relation-batch-02` promotes three additional relation
contexts from already accepted source custody: medieval Kingdom of Serbia to
Serbian Empire as a 1346 rank transition, County of La Marche to French crown
confiscation as a 1527 rank transition, and Hordaland to modern Norway as a
bounded `subdivision_or_appanage` relation for 1919..2019. Structured-only
Romanian, Hungarian, Portuguese, and Commonwealth rows remain held for
relation-specific source review.

`commonwealth-union-subdivision-relation-01` adds six narrowly reviewed text
source records and promotes six bounded relation contexts: Courland and Livonia
as Commonwealth vassalage/suzerainty contexts, Grand Duchy of Lithuania and
Kingdom of Portugal as component contexts under the Polish-Lithuanian
Commonwealth and Iberian Union, Abauj county as a Kingdom of Hungary
subdivision, and Burgundian Netherlands as a Burgundian State component.

`german-polish-client-relation-01` adds four reviewed text source records and
promotes five bounded relation contexts: Prussia and Saxony as German
Confederation members, Prussia and Saxony as North German Confederation federal
state/member contexts, and the 1916..1918 Kingdom of Poland as a German Empire
client/suzerainty context.

`administrative-dependency-relation-01` adds five reviewed text source records
and promotes six bounded relation contexts: Cetatea-Alba County, Dorohoi County,
County of Nice, Grand Duchy of Finland, and Illyrian Provinces as
subdivision/appanage contexts under their accepted parents, plus Kingdom of the
Burgundians as a Western Roman dependency/suzerainty context.

`austrian-monarchy-component-relation-01` adds ten reviewed text source records
and promotes sixteen bounded Austrian monarchy component contexts. Carniola,
Gorizia and Gradisca, Bohemia, Galicia and Lodomeria, Illyria,
Lombardy-Venetia, Moravia, and Salzburg use `crownland_component` for accepted
Austrian Empire or Austria-Hungary parent spans. Croatia-Slavonia and the three
accepted Kingdom of Hungary title rows use `composite_crown_component` where
the source custody is broader kingdom/dual-monarchy component context rather
than narrow Cisleithanian crownland context. Parentage remains unchanged.

`residual-rank-skip-relation-01` adds fourteen reviewed text source records and
promotes the final seventeen active rank-skip relation contexts. The packet
covers Papal, Iberian, French, Prussian, HRE, Byzantine, Carolingian, and Lower
Alsace residual rows using bounded relation semantics rather than changing
parentage. The generated relation report now explains all active rank-skip rows
while preserving the measured temporal forest and zero temporal parent
conflicts.

`catalonia-crown-aragon-parentage-01` adds one reviewed text source record, one
parentage fact, and one matching `composite_crown_component` relation for the
Principality of Catalonia under the Crown of Aragon. This improves graph
coverage while keeping the rank-skip relation report fully explained.

`low-countries-crown-hre-parentage-01` adds four reviewed text source records,
four parentage facts, and four matching `composite_crown_component` relations
for Habsburg Netherlands, Seventeen Provinces, Spanish Netherlands, and
Austrian Netherlands under the Holy Roman Empire. Dutch Republic and Batavian
Republic remain deferred because their parentage semantics need successor or
de facto modeling rather than a simple de jure empire parent.
