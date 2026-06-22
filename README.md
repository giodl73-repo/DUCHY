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
  The accepted fixture catalog now contains 410 reviewed source records.

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
