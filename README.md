# DUCHY

DUCHY is a historical-title timeline lab for game design. It tracks counties,
duchies, kingdoms, and empires across years so strategy and world-building
projects can ask CK-style questions about title rank, continuity, control, and
legitimacy without copying a commercial game's data or rules.

The first foundation is a Rust model for title snapshots:

- title ranks from county through empire, including specialized forms such as
  free cities,
- year-bounded existence and control spans,
- de jure parentage separate from de facto holder/control,
- continuity events such as creation, conquest, inheritance, partition, and
  extinction,
- validation that title hierarchies and spans are internally coherent.

## First Command

```powershell
cargo test --quiet
cargo run --quiet
cargo run --bin duchy-import -- status
cargo run --bin duchy-import -- manifest data/staging/example.manifest
cargo run --bin duchy-import -- source-stubs data/staging/example.manifest data/staging/generated.sources
cargo run --bin duchy-import -- rejected-report data/staging/example.manifest data/staging/rejected.md
cargo run --bin duchy-import -- active-manifest data/staging/example.manifest data/staging/active.manifest
cargo run --bin duchy-import -- archive-manifest data/staging/example.manifest data/staging/archive.manifest
cargo run --bin duchy-import -- manifest-report data/staging/example.manifest data/staging/manifest-report.md
cargo run --bin duchy-import -- duplicate-url-report data/staging/example.manifest data/staging/duplicate-urls.md
cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest data/staging/manifest.tsv
cargo run --bin duchy-import -- manifest-from-tsv data/staging/manifest.tsv data/staging/from-tsv.manifest
cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest data/staging/shards 2
cargo run --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md
cargo run --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo run --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --bin duchy-import -- parentage-rank-skip-candidates fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-candidates.md
cargo run --bin duchy-import -- parentage-rank-skip-bridges-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-bridges.tsv
cargo run --bin duchy-import -- parentage-rank-skip-bridge-shard data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-shards 25
cargo run --bin duchy-import -- parentage-rank-skip-bridge-report data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-report.md
cargo run --bin duchy-import -- parentage-rank-skip-bridge-clusters-tsv data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-clusters.tsv
cargo run --bin duchy-import -- parentage-rank-skip-bridge-cluster-report data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-report.md
cargo run --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-tsv data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-review.tsv
cargo run --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-report data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-report.md
cargo run --bin duchy-import -- parentage-rank-skip-bridge-cluster-packet-stubs data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md
cargo run --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-shard data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-shards 5
cargo run --bin duchy-import -- parentage-rank-skip-shard data/staging/parentage-rank-skip-targets.tsv data/staging/parentage-rank-skip-shards 25
cargo run --bin duchy-import -- parentage-rank-skip-report data/staging/parentage-rank-skip-targets.tsv data/staging/parentage-rank-skip-report.md
```

## Product Shape

DUCHY starts as a data/model repo, not a map renderer. The near-term product is
an inspectable timeline contract that can support historical strategy fixtures,
fictional realm generators, and design comparison packets.

The first vertical slice uses tiny hand-authored fixtures. Real historical data
requires source custody, citation policy, and rights review before import.

## Governance And VTRACE

- `.roles/` defines review authority for timeline semantics, territorial
  lineage, source custody, game-system boundaries, and query interfaces.
- `docs/vtrace/` defines the lineage-query mission, CONOPS, requirements,
  interfaces, trace matrix, verification, validation, and work packages.
- `docs/vtrace/source-custody/` defines the gate for real historical sources.
  The accepted fixture catalog now contains 520 reviewed source records.

The first accepted capability target is the ability to ask which higher title
contained an area in a year, how an area moved between duchies over a date
range, and which continuity events explain a title lineage.

The current source-custody slice validates metadata-only source records, review
decisions, the source-backed fact gate, the first minimal real title facts, and
materialization of those facts into a `Title`. It can also answer a traced
source-backed title-path query for the reviewed mini-catalog, including bounded
Q20135 -> Q43287, Q158445 -> Q43287, Q27306 -> Q43287, and Q153015 -> Q43287
parentage paths for 1871-1918. The first real facts now live in
`fixtures/first-real.facts`, and contested fact groups are blocked from normal
materialization until reviewed. Reviewed real source metadata lives in
`fixtures/first-real.sources`. The German Confederation bridge packet adds
Q151624, Q150981, Q154195, and Q159631, giving the reviewed mini-catalog
1815-1866, 1867-1870, and 1871-1918 parentage spans for selected German
kingdoms and grand duchies. The Rust crate treats those fixtures as canonical
for reviewed historical imports; source-backed historical IDs, names, and spans
must not be duplicated as Rust literals.
The title-identity queue promotions and follow-on parentage packets extend the
accepted fixture set to 410 reviewed sources and 1260 reviewed facts. Title
promotion adds title identity, rank, and existence facts only; parentage remains
a separate review step.
The 500-source candidate queue is now closed for unsupported rows: 167
relation-heavy, non-title, free-city, administrative-region, and otherwise
unsupported candidates are archived as `scope_deferred` rejections, leaving only
the fourteen date-problem title candidates active.
Those fourteen date-problem title candidates are also closed: thirteen are
`quality_blocked` because the structured Wikidata date gate could not provide a
complete span, and one is `date_conflict` because the extracted span was
reversed. The 500-source candidate queue now has no active pending rows.
The second German parentage packet adds 23 reviewed parentage spans for
additional accepted German Confederation, North German Confederation, and German
Empire-era titles. The first Austrian parentage packet adds 15 reviewed spans
for Austrian Empire and Austria-Hungary-era titles. The first Holy Roman Empire
parentage packet adds 70 reviewed pre-1807 spans. The first Kingdom of France
parentage packet adds 4 reviewed spans. The residual German Confederation
packet adds 4 reviewed spans, with later relation, crown, Commonwealth,
theocratic-state, German Empire, Russian Empire, Gorizia and Gradisca Austrian,
Duchy of Urbino Papal States, Illyrian Provinces French Empire, and Electoral
Palatinate HRE packets, followed by the Free Imperial City of Aachen HRE,
Burgraviate of Nuremberg HRE, Courland Commonwealth, and Estonia Sweden
packets, followed by the Schleswig Denmark, Burgundian Netherlands State,
Hispanic Monarchy crown, Kalmar Union Sweden, Prince-Bishopric of Trent HRE,
County of Savoy HRE, Duchy of Warsaw French Empire, Confederation of the Rhine
Westphalia and French Empire, Prince-Bishopric of Augsburg HRE, Kingdom of
Imereti Russian Empire, Italian protectorate of Albania Italian Empire, Francia
Carolingian Empire, Duchy of Prussia Polish Crown, County of Empuries Crown of
Aragon, Grand Duchy of Tuscany HRE, Duchy of Parma and Piacenza Empire, Kingdom
of Naples Crown of Aragon, Grand Principality of Moscow Golden Horde, and
County of Astarac Duchy of Gascony packets, followed by the Duchy of Bohemia
HRE, Duchy of Florence HRE, and Prince-Bishopric of Freising HRE packets,
followed by the Prince-Bishopric of Basel HRE and Prince-Bishopric of Toul HRE
packets, then the Margraviate of Austria HRE, Grand Duchy of Berg Rhine
Confederation, Duchy of Modena and Reggio HRE, County of Wurttemberg HRE,
Duchy of Austria HRE, Duchy of Athens Thessalonica, Kingdom of Thessalonica
Latin Empire, and Duchy of Cleves HRE packets, raising reviewed real parentage
coverage to 217 facts, followed by the County of Luxembourg HRE packet and the
first high-priority HRE county/duchy closure batch, and the second
high-priority HRE county/duchy closure batch, raising reviewed real parentage
coverage to 225 facts. The third high-priority mixed parentage batch adds
Jülich HRE, Würzburg Rhine Confederation, Frankfurt Rhine Confederation, and
Normandy Kingdom of France spans, raising reviewed real parentage coverage to
229 facts. The fourth high-priority Bohemian/Silesian batch adds Opava,
Silesia, Teschen, Bytom, and Oswiecim Kingdom of Bohemia spans, raising
reviewed real parentage coverage to 234 facts. The fifth high-priority mixed
batch adds Nassau-Siegen HRE spans and Grand Principality of Vladimir Golden
Horde parentage, raising reviewed real parentage coverage to 237 facts. The
sixth high-priority mixed batch adds Belz Polish Crown, Lotharingia East
Francia, Serbia Ottoman, United Principalities Ottoman, and Prince-Bishopric of
Montenegro Ottoman spans, raising reviewed real parentage coverage to 242 facts.
The seventh high-priority mixed batch adds Courland German Empire, Moravian
Serbia Ottoman, Amalfi Byzantine, Ferrara Papal States, and Gascony Kingdom of
France endpoint/vassalage spans, raising reviewed real parentage coverage to
247 facts. The eighth high-priority mixed batch adds County of Sicily Kingdom
of Sicily and County of Nice Kingdom of Sardinia endpoint spans, raising
reviewed real parentage coverage to 249 facts.
The ninth endpoint batch adds Duchy of Poland Kingdom of Poland and
Polish-Lithuanian union Commonwealth endpoint spans, raising reviewed real
parentage coverage to 251 facts; the three remaining high-priority rows are
tracked as explicit blockers because they need modern Norway coverage or
contested/de facto relation modeling.
The tenth high-priority Norway batch adds modern Kingdom of Norway coverage and
Hordaland parentage, raising the accepted catalog to 336 titles and reviewed
real parentage coverage to 252 facts. The high-priority parentage queue now has
no active rows; Albania and Montenegro remain visible as
`blocked_parentage_review` because they need contested/de facto relation
modeling before safe parentage import.
The first medium-priority parentage batch adds Kingdom of Holland, Kingdom of
Etruria, and Napoleonic Kingdom of Italy client-state spans under the First
French Empire, raising reviewed real parentage coverage to 255 facts.
The second medium-priority parentage batch adds a Kingdom of Sicily span under
the Crown of Aragon, raising reviewed real parentage coverage to 256 facts.
The third medium-priority parentage batch adds Eastern Hungarian Kingdom
parentage under the Ottoman Empire, raising reviewed real parentage coverage to
257 facts.
The fourth medium-priority parentage batch promotes Tsardom of Russia and adds
Kingdom of Livonia parentage under it, raising the catalog to 337 titles and
reviewed real parentage coverage to 258 facts.
The fifth medium-priority parentage batch adds a Kingdom of Bosnia endpoint
span under the Ottoman Empire, raising reviewed real parentage coverage to 259
facts.
The sixth medium-priority parentage batch adds an Albanian Kingdom endpoint
span under the Italian Empire, raising reviewed real parentage coverage to 260
facts.
The county scaling pilot adds a parentage change report over accepted fixtures;
the current baseline has 215 titles with parentage, 34 titles with modeled
parent changes, 44 modeled parent changes, 30 county parentage titles, and 2
county titles with modeled parent changes.
The first county scaling queue classifies 500 CK3 Europe county search-driver
rows into agent-ready shard statuses: 4 accepted parentage seeds, 1 accepted
title needing parentage review, 6 rank-semantics review rows, 462 deferred
source-resolution rows, and 27 rows needing fresh source resolution.
The first farmed shard review covers `county-scale-004`: 50 rows reviewed with
1 already accepted seed, 3 rank-policy blockers, 31 source-resolution blockers,
and 15 rejected bad leads. No accepted fixtures changed in that shard review.
The next priority farm reviews cover shards `001`, `002`, `006`, and `009`,
bringing reviewed county-scale shard rows to 250. Across those reviewed shards,
agents recorded 5 already accepted seeds, 6 ready title/title-follow-up
candidates, 18 rank-policy blockers, 178 source-resolution blockers, and 43
rejected bad leads.
The remaining shard reviews cover `003`, `005`, `007`, `008`, and `010`,
completing the 500-row farm pass. The aggregate review set now has 13 accepted
seeds, 21 ready title/title-follow-up candidates, 50 rank-policy blockers, 329
source-resolution blockers, and 87 rejected bad leads, with zero ready
parentage packets.
The first county title harvest promotes three reviewed title-only packets from
that farm: Dorohoi County, County Palatine of Durham, and County of Girona. The
accepted catalog now has 422 reviewed sources, 1280 facts, 340 titles, and 260
parentage facts; parentage remains intentionally unchanged until direct
relation sources are reviewed.
The second county title harvest promotes four more reviewed title-only packets:
Duchy of Benevento, Taifa of Denia, Kingdom of Desmond, and Princely Abbey of
Fulda. The accepted catalog now has 426 reviewed sources, 1292 facts, 344
titles, and 260 parentage facts; the parentage gap queue now has 129 rows split
across six shards.
The third county title harvest promotes Free imperial city of Dortmund with a
corrected `1220..1803` span from Britannica. The accepted catalog now has 427
reviewed sources, 1295 facts, 345 titles, and 260 parentage facts; the parentage
gap queue now has 130 rows.
The fourth county title harvest promotes County of La Marche with a corrected
`958..1527` span. The accepted catalog now has 428 reviewed sources, 1298 facts,
346 titles, and 260 parentage facts; the parentage gap queue now has 131 rows.
The county title harvest closure reconciles all 21 ready title/title-follow-up
candidates from the 500-row farm: 7 were already accepted, 9 were promoted by
harvest batches 01-04, and 5 remain deferred pending better bounded source or
rank/span policy.
The first post-harvest parentage packet adds Dorohoi County under the Kingdom of
Romania for `1881..1947`. The accepted catalog now has 428 reviewed sources,
1299 facts, 346 titles, and 261 parentage facts; county titles with parentage
rise to 31.
The next post-harvest parentage packet adds Duchy of Benevento under the Kingdom
of the Lombards for `577..774`. The accepted catalog now has 428 reviewed
sources, 1300 facts, 346 titles, and 262 parentage facts; 217 titles now have
reviewed parentage.
The next post-harvest endpoint packet adds County of La Marche under the
Kingdom of France for `1527..1527`. The accepted catalog now has 429 reviewed
sources, 1301 facts, 346 titles, and 263 parentage facts; 218 titles now have
reviewed parentage and 128 titles remain without parentage.
The next post-harvest parentage packet adds County of Girona under the
Carolingian Empire for `800..887`. The accepted catalog now has 430 reviewed
sources, 1302 facts, 346 titles, and 264 parentage facts; 219 titles now have
reviewed parentage, 33 county titles have parentage, and the generated parentage
gap queue has no active high-priority rows.
The next medium-priority endpoint packet corrects Kingdom of Great Britain to
DUCHY's composite `Crown` rank and adds Kingdom of England and Kingdom of
Scotland endpoint parentage under it for `1707..1707`. The accepted catalog now
has 431 reviewed sources, 1304 facts, 346 titles, and 266 parentage facts; 221
titles now have reviewed parentage and the parentage gap queue has 125 rows
split across five shards.
The next medium-priority parentage packet adds Free imperial city of Dortmund
under the Holy Roman Empire for `1220..1803`. The accepted catalog now has 432
reviewed sources, 1305 facts, 346 titles, and 267 parentage facts; 222 titles
now have reviewed parentage and 124 titles remain without parentage.
The next medium-priority parentage packet adds Princely Abbey of Fulda under the
Holy Roman Empire for `1221..1802`. The accepted catalog now has 432 reviewed
sources, 1306 facts, 346 titles, and 268 parentage facts; 223 titles now have
reviewed parentage and 123 titles remain without parentage.
The next medium-priority successor packet promotes United Kingdom of Great
Britain and Ireland as a bounded `Crown` title for `1801..1922` and adds Kingdom
of Ireland endpoint parentage under it for `1801..1801`. The accepted catalog now
has 434 reviewed sources, 1310 facts, 347 titles, and 269 parentage facts; 224
titles now have reviewed parentage and the parentage gap queue remains at 123
rows because the new UKGBI crown title itself needs successor-context review.
The next medium-priority parentage packet adds Denmark under the Kalmar Union
for `1397..1523` and medieval Norway under the Kalmar Union for `1397..1397`.
The accepted catalog now has 434 reviewed sources, 1312 facts, 347 titles, and
271 parentage facts; 226 titles now have reviewed parentage and 121 titles
remain without parentage.
The next medium-priority endpoint packet promotes Serbian Empire as a bounded
`Empire` title for `1346..1371` and adds medieval Kingdom of Serbia endpoint
parentage under it for `1346..1346`. The accepted catalog now has 436 reviewed
sources, 1316 facts, 348 titles, and 272 parentage facts; 227 titles now have
reviewed parentage and the parentage gap queue remains at 121 rows because the
new Serbian Empire title itself needs root-or-successor review.
The next medium-priority endpoint packet adds Jagiellon Kingdom of Poland
endpoint parentage under the Polish-Lithuanian Commonwealth for `1569..1569`.
The accepted catalog now has 436 reviewed sources, 1317 facts, 348 titles, and
273 parentage facts; 228 titles now have reviewed parentage and 120 titles
remain without parentage.
The next medium-priority parentage packet promotes Western Roman Empire as a
bounded `Empire` title for `395..476` and adds clipped Kingdom of the
Burgundians parentage under it for `443..476`. The accepted catalog now has 438
reviewed sources, 1321 facts, 349 titles, and 274 parentage facts; 229 titles
now have reviewed parentage and the parentage gap queue remains at 120 rows
because the new Western Roman Empire title itself needs root-or-successor
review.
The parentage graph audit now treats hierarchy as a temporal forest instead of
one timeless duchy tree. The current graph report covers 349 titles, 278 active
parentage facts, 308 parentable titles, 229 titles with parentage, 74.35%
title-level parentage fill, 59.44% weighted span coverage, 220 valid rank-skip
facts, no temporal parent conflicts, and no snapshot cycles across 544 generated
snapshot years.
The Kingdom of Italy replacement parentage packet adds two reviewed text
sources and two replacement facts, superseding direct HRE parentage for March of
Turin and March of Tuscany while keeping the active graph conflict-free. The
accepted catalog now has 440 reviewed sources, 1323 facts, 349 titles, 274
active parentage facts, and 2 superseded parentage facts.
The early Bavaria parentage packet adds Duchy of Bavaria under East Francia for
`907..961`, extending the accepted catalog to 1324 facts and 275 active
parentage facts without changing the active rank-skip row count.
The early Saxony parentage packet adds one reviewed text source and parents
Duchy of Saxony under East Francia for `843..961`, extending the accepted
catalog to 441 reviewed sources, 1325 facts, and 276 active parentage facts.
The early Lower Lotharingia parentage packet uses the existing reviewed Lower
Lotharingia source to add East Francia parentage for `959..961`, extending the
accepted catalog to 1326 facts and 277 active parentage facts.
The early Lorraine parentage packet adds one reviewed text source and parents
Duchy of Lorraine under East Francia for `959..961`, extending the accepted
catalog to 442 reviewed sources, 1327 facts, and 278 active parentage facts.
The HRE parentage source-custody tightening pass adds a reviewed Margraviate of
Meissen text source and attaches reviewed text sources to the existing Lorraine,
Saxony, and Meissen HRE parentage facts, extending the catalog to 443 reviewed
sources without changing hierarchy metrics.
The Brandenburg/Pomerania HRE source-custody pass adds two reviewed text
sources and attaches them to the existing Margraviate of Brandenburg and Duchy
of Pomerania HRE parentage facts, extending the catalog to 445 reviewed sources
without changing hierarchy metrics.
The Holland HRE source-custody pass adds one reviewed text source and attaches
it to the existing County of Holland HRE parentage fact, extending the catalog
to 446 reviewed sources. County of Flanders remains held for split
French/Imperial parentage modeling instead of being flattened into a single
source-custody claim.
The Mantua/Savoy HRE source-custody pass adds two reviewed text sources and
attaches them to the existing Duchy of Mantua and Duchy of Savoy HRE parentage
facts, extending the catalog to 448 reviewed sources. Duchy of Milan remains
held for Italian/Habsburg split review rather than using a full-span shortcut.
The Palatinate/Mainz/Cologne HRE source-custody pass adds three reviewed text
sources and attaches them to the existing Electoral Palatinate, Electorate of
Mainz, and Electorate of Cologne HRE parentage facts, extending the catalog to
451 reviewed sources without changing hierarchy metrics.
The Bavaria/Hanover/Saxony electorate source-custody pass adds three reviewed
text sources and attaches them to the existing Electorate of Bavaria,
Electorate of Hanover, and Electorate of Saxony HRE parentage facts, extending
the catalog to 454 reviewed sources without changing hierarchy metrics.
The Baden/Wurttemberg electorate source-custody pass adds two reviewed text
sources and attaches them to the existing Electorate of Baden and Electorate
of Wurttemberg HRE parentage facts, extending the catalog to 456 reviewed
sources without changing hierarchy metrics.
The Duchy of Anhalt source-custody pass adds reviewed text support for its
existing German Confederation, North German Confederation, and German Empire
parentage facts, extending the catalog to 457 reviewed sources without
changing hierarchy metrics.
The Oldenburg/Brunswick source-custody pass adds reviewed text support for
their existing German Confederation, North German Confederation, and German
Empire parentage facts, extending the catalog to 459 reviewed sources without
changing hierarchy metrics.
The Mecklenburg/Hesse source-custody pass adds reviewed text support for
existing Mecklenburg-Schwerin, Mecklenburg-Strelitz, and Hesse confederation
and empire parentage facts, extending the catalog to 462 reviewed sources
without changing hierarchy metrics.
The Saxe-Meiningen source-custody pass adds reviewed text support for existing
Holy Roman Empire, German Confederation, and German Empire parentage facts,
extending the catalog to 463 reviewed sources without changing hierarchy
metrics.
The Saxe-Altenburg/Schaumburg-Lippe source-custody pass adds reviewed text
support for their existing Holy Roman Empire and German Confederation parentage
facts, extending the catalog to 465 reviewed sources without changing
hierarchy metrics.
The Saxe-Weimar-Eisenach/Schwarzburg-Sondershausen source-custody pass adds
reviewed text support for existing German Confederation and North German
Confederation spans, plus the existing Schwarzburg-Sondershausen HRE span,
extending the catalog to 467 reviewed sources without changing hierarchy
metrics.
The Saxe-Coburg and Gotha/Schwarzburg-Rudolstadt source-custody pass adds
reviewed text support for existing German Confederation parentage facts,
extending the catalog to 469 reviewed sources without changing hierarchy
metrics.
The Saxe-Coburg-Saalfeld/Saxe-Gotha-Altenburg/Saxe-Hildburghausen
source-custody pass adds reviewed text support for existing HRE and German
Confederation parentage facts, extending the catalog to 472 reviewed sources
without changing hierarchy metrics.
The Nassau/Saxe-Lauenburg source-custody pass adds reviewed text support for
existing German Confederation parentage and Saxe-Lauenburg HRE parentage,
extending the catalog to 474 reviewed sources without changing hierarchy
metrics.
The Hesse/Lippe/Holstein source-custody pass adds reviewed text support for
existing German Confederation parentage plus the accepted Lippe and Holstein
HRE parentage facts, extending the catalog to 477 reviewed sources without
changing hierarchy metrics.
The Anhalt-Bernburg/Hohenzollern source-custody pass adds reviewed text
support for existing Anhalt-Bernburg, Hohenzollern-Hechingen, and
Hohenzollern-Sigmaringen HRE and German Confederation parentage facts,
extending the catalog to 480 reviewed sources without changing hierarchy
metrics.
The Anhalt-Dessau/Anhalt-Kothen/Brunswick-Luneburg source-custody pass adds
reviewed text support for existing Anhalt HRE and German Confederation facts
plus Brunswick-Luneburg HRE parentage, extending the catalog to 483 reviewed
sources without changing hierarchy metrics.
The Brunswick-Wolfenbuttel/Burgundy/Nassau source-custody pass adds reviewed
text support for existing HRE parentage facts, extending the catalog to 486
reviewed sources without changing hierarchy metrics.
The Brabant/Bremen-Verden/Carniola source-custody pass adds reviewed text
support for existing HRE parentage facts, extending the catalog to 489
reviewed sources without changing hierarchy metrics.
The Austria/Bavaria-Munich/Luxembourg source-custody pass adds reviewed text
support for existing HRE parentage facts, extending the catalog to 492
reviewed sources without changing hierarchy metrics.
The Flanders/Berg/Guelders/Bohemia/Hesse source-custody pass adds reviewed
text support for existing HRE parentage facts, reaching the 500 reviewed
source milestone without changing hierarchy metrics.
The Swabia/Wurttemberg/ecclesiastical-principality source-custody pass adds
reviewed text support for fourteen more existing HRE parentage facts,
extending the catalog to 514 reviewed sources without changing hierarchy
metrics.
The Mecklenburg-Schwerin/Italy/Brabant/Mirandola/Sulzbach source-custody pass
adds reviewed text support for five more existing HRE parentage facts,
extending the catalog to 519 reviewed sources without changing hierarchy
metrics.
The Burgundian Netherlands replacement parentage packet adds one reviewed text
source and three partial replacement facts, routing County of Flanders
`1384..1482`, County of Holland `1433..1482`, and County of Namur `1421..1482`
through Burgundian Netherlands while keeping the older HRE facts auditable and
active outside those bounded spans. The catalog now has 520 reviewed sources,
1330 facts, 5 superseded parentage facts, and a valid timeline; active
fact-row rank skips drop from 220 to 217.
The report layer now materializes those partial supersessions before graph,
change, rank-skip, candidate, and bridge analysis. The current graph report has
284 parentage edges, 59.44% weighted span coverage, 223 rank-skip edge rows,
zero temporal parent conflicts, and zero snapshot cycle years across 547
generated snapshot years.
The rank-skip queue exports those 223 valid-but-incomplete hierarchy edges for
review. The largest class is duchy-to-empire parentage, which needs reviewed
kingdom-layer packets before DUCHY can treat those branches as a proper
immediate-rank tree.
The rank-skip queue is now sharded into nine 25-row review batches with an
index and Markdown rollup report, so intermediate-parent review can be assigned
without reopening the broader graph report.
The rank-skip candidate report cross-checks those rows against accepted titles:
218 of 223 rank-skip rows have an overlapping accepted title at the expected
immediate rank, and 163 rows have at least one stronger bridge candidate already
parented under the same current parent during an overlapping span.
The bridge TSV distills those 163 rows into a compact queue for targeted
source-review work. A bridge row is not proof of correct immediate parentage; it
only means DUCHY already has an accepted candidate title of the expected rank
under the same current parent.
The bridge queue is also sharded into seven review batches. Its rollup shows
the largest candidate-parent clusters are Kingdom of Bohemia, Kingdom of
Bavaria, and Kingdom of Italy, while Holy Roman Empire is the dominant current
parent for bridge review.
The bridge cluster TSV groups those leads into 20 candidate/current-parent
clusters covering 162 distinct skipped children, making the next source-review
packets visible as coherent child sets rather than individual skipped facts.
The bridge cluster report turns those clusters into review packets and keeps
the warning explicit: clusters are packet-planning leads, not import-ready
parentage claims.
The cluster review TSV makes that boundary operational and now preserves
existing review decisions when regenerated from refreshed materialized-edge
clusters. The current 20 cluster rows are all reviewed: 19 are marked
`same_parent_sibling_false_positive`, and the remaining Kingdom of Italy/HRE
cluster is held as `mixed_cluster_requires_child_split`.
The cluster review shards split those 20 rows into four five-row packets with an
index that preserves review dispositions and priority totals for assignment.
The cluster review report summarizes the editable review TSV by status,
disposition, evidence need, and row priority so packet progress can be audited
without opening every shard.
The packet-stub report emits import-planning stubs only for rows explicitly
marked `reviewed` and `ready_for_packet`; the current queue emits zero stubs
because all rows remain blocked.
The active Kingdom of Italy/HRE split review now has 29 rows: March of Turin
and March of Tuscany are removed from the active cluster because their Kingdom
of Italy replacement parentage was imported, leaving 28 same-parent sibling
false positives and one held Trent relation-model lead. The Trent review keeps
Prince-Bishopric of Trent directly under the Holy Roman Empire for `1027..1803`;
the current evidence supports HRE imperial-estate/state semantics, not a clean
Kingdom of Italy parentage replacement.
The first county rank-skip review batch covers ten high-priority county rows
and imports no replacements: two rows are shared-parent sibling false positives,
three need child-level intermediate sources, and five need split-control or
relation-model work before DUCHY can safely replace direct county-to-crown or
county-to-empire parentage.

Batch candidate imports go through `data/staging/` and must pass dry-run
promotion before accepted fixture rows are appended:

```powershell
cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

Reviewed batches can then be appended with `duchy-promote --apply`, which
rewrites accepted fixture files only after the same validation path passes.
Add `--report path.md` after the mode to emit a promotion review report.

## Relationship To Games Design

- BANISH can consume DUCHY-style political timelines when settlement pressure
  needs feudal control, law, tribute, or war context.
- QUEST can use title continuity for dynastic tabletop campaign scaffolds.
- TIGRIS can use title maps as board-state scenarios.
- COURT/RACKET/MUDDLE are future experience surfaces only after DUCHY has a
  stable timeline contract.

## Non-Goals

- No commercial-game data, mechanics, or clone behavior.
- No claim that the seed fixtures are authoritative history.
- No renderer or grand-strategy engine in the foundation wave.
- No raw source corpus import before source custody and citation policy exist.
- No shared framework changes until at least two repos need the same primitive.

## License

[MIT](LICENSE) - copyright 2026 Gio Della-Libera.
