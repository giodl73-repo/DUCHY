# DUCHY Product Plan

## Thesis

Strategy games and world-building tools need political geography that changes
over time: a county can be held by one ruler, belong de jure to one duchy, be
contested by another kingdom, split under inheritance, or disappear as a title.
DUCHY makes that timeline explicit and testable.

The first value is not a giant database. The first value is a small, rigorous
contract for tracking title identity, rank, dates, control, parentage, and
continuity events.

## Audience

- game designers prototyping CK-like political timelines,
- tabletop campaign builders who need dynastic title continuity,
- world-building simulators that need realms, vassalage, and border pressure,
- future knowledge-system consumers that need neutral political-geography facts.

## Wave 1: Foundation Timeline Contract

Outcome: a tested Rust model for title ranks, title spans, parentage, control,
and continuity events.

Pulses:

1. Workspace foundation: repo docs, Rust crate, and seed validation.
2. Fixture schema: hand-authored example titles and yearly snapshot reports.
3. Query surface: answer "what existed in year X" and "who controlled title Y".
4. Design packet: produce a small scenario packet for a game-design consumer.

VTRACE refinement: the first query surface is governed by
`docs/vtrace/REQUIREMENTS.md` and should prioritize:

- title path in year,
- lineage events for a title,
- areas/titles that moved between duchies over a range,
- answer trace and negative/uncertain answer status.

## Wave 2: Historical Source Custody

Outcome: a source plan for real European title data without bundling uncertain
or rights-unsafe material.

Pulses:

1. Source inventory and rights posture. Complete as a policy gate in
   `docs/vtrace/source-custody/`.
2. Citation and confidence model. Complete as a policy gate in
   `docs/vtrace/source-custody/`.
3. Import adapter for metadata-only source records. Rust metadata catalog,
   validation, and source file parser complete.
4. Source-backed fact gate. Complete for validation logic.
5. First real source-backed facts. Complete for the minimal Wikidata Q158445
   name/rank/existence slice.
6. Source-backed title materialization. Complete for one title record with no
   parentage/control claims.
7. Source-backed title query. Complete for the first real title path envelope.
8. Fact fixture import. Complete for `fixtures/first-real.facts`.
9. Review packet for contested/uncertain history. Complete for fact-level
   grouping and materialization blocking.
10. Second reviewed source import. Complete for Q20135 Grand Duchy of Hesse.
11. Source-backed parentage review. Complete for materialization support.
12. Reviewed source fixture import. Complete for `fixtures/first-real.sources`.
13. First real parentage source import. Complete for Q20135 -> Q43287 and
    Q158445 -> Q43287, 1871-1918.
14. Fixture-canonical import path. Complete: reviewed historical facts and
    source records live in fixtures, not Rust literals.
15. Reviewed Prussia import packet. Complete for Q27306 Kingdom of Prussia
    title facts and Q27306 -> Q43287, 1871-1918.
16. Reviewed Saxony import packet. Complete for Q153015 Kingdom of Saxony
    title facts and Q153015 -> Q43287, 1871-1918.
17. Batch import staging gate. Complete for CLI status, dry-run promotion, and
    duplicate/conflict validation.
18. Apply-mode promotion. Complete for validated staging-to-accepted fixture
    merge.
19. Promotion review reports. Complete for `duchy-promote --report`.
20. Candidate manifest queue. Complete for staging manifest parsing and status
    counts.
21. Source stub generation. Complete for blocked review-required source stubs
    from reviewed manifest rows.
22. Rejected candidate audit. Complete for rejected manifest report generation.
23. Active manifest cleanup. Complete for pending/reviewed manifest generation.
24. Archive manifest cleanup. Complete for promoted/rejected manifest
    generation.
25. Manifest sharding. Complete for fixed-size candidate review batches.
26. Shard index. Complete for per-shard review status counts.
27. Manifest review report. Complete for status-grouped candidate inspection.
28. Duplicate URL report. Complete for repeated candidate source URL hygiene.
29. Manifest TSV export. Complete for machine-readable candidate queues.
30. Manifest TSV import. Complete for fixed-column batch queue intake.
31. German bridge import. Complete for German Confederation, North German
    Confederation, Bavaria, and Wurttemberg source-backed parentage spans.
32. Baden and Hanover import. Complete for Q186320 Grand Duchy of Baden and
    Q164079 Kingdom of Hanover source-backed title facts and parentage spans.
33. Oldenburg and Brunswick import. Complete for Q693669 Grand Duchy of
    Oldenburg and Q326029 Duchy of Brunswick source-backed title facts and
    parentage spans.
34. Mass title source scale-up. Complete for 37 additional reviewed Wikidata
    title sources, bringing the accepted source catalog to 50.
35. Scale metadata manifest gate. Complete for 500-source readiness metadata:
    import scope, rank basis, entity class, claim usage, confidence detail,
    parentage status, query readiness, and exclusion reasons.
36. 500-source candidate queue. Complete for 450 staged Wikidata candidates,
    bringing accepted plus staged source records to 500 under review.
37. Batch 001 title promotion. Complete for 18 title-identity sources promoted
    from the 500-source candidate queue.
38. Batch 002 title promotion. Complete for 26 title-identity sources promoted
    from the 500-source candidate queue, with four title candidates deferred for
    missing date claims.
39. Remaining title queue promotion. Complete for 225 title-identity sources
    promoted from the remaining 500-source candidate queue, with fourteen title
    candidates deferred for missing or invalid date claims.
40. Unsupported queue closure. Complete for 167 unsupported or relation-heavy
    candidates archived as `scope_deferred`, leaving fourteen date-problem title
    candidates active.
41. Date-problem queue closure. Complete for the remaining fourteen
    title-identity candidates rejected as `quality_blocked` or `date_conflict`,
    closing the 500-source candidate queue with no pending rows.
42. Additional German parentage packet. Complete for 23 parentage-only facts
    using existing accepted source records and merged-catalog promotion
    validation.
43. Austrian parentage packet. Complete for 15 parentage-only facts under the
    Austrian Empire and Austria-Hungary using existing accepted source records.
44. Holy Roman Empire parentage packet. Complete for 70 parentage-only facts
    under the Holy Roman Empire using existing accepted source records.
45. Kingdom of France parentage packet. Complete for 4 parentage-only facts
    using existing accepted source records and excluding overlapping HRE spans.
46. Residual German Confederation parentage packet. Complete for 4
    parentage-only facts using existing accepted source records.
47. CK3 Europe county discovery queue. Complete for 500 pending county
    candidates generated from a saved CK3 wiki county list, filtered to
    Europe-facing CK3 empire buckets, with no accepted historical facts
    promoted.
48. CK3 batch 001 source-resolution leads. Complete for a 50-row Wikidata
    search packet that records exact, fuzzy, and no-result external leads
    without treating search hits as accepted historical source records.
49. CK3 batch 001 reviewed source-resolution packet. Complete for 7 reviewed
    Wikidata source records promoted from the first CK3 research shard, with
    facts explicitly deferred.
50. CK3 batch 001 first title fact packet. Complete for Abaúj county
    source-backed name, county rank, and 1201-1881 existence span from the
    reviewed Wikidata source record.
51. CK3 batch 001 structured screen and Ailech source record. Complete for
    structured-claim screening across first-shard Wikidata leads and one
    additional source-only Ailech record with title facts deferred for rank
    semantics.
52. CK3 batch 002 source-resolution packet. Complete for a second 50-row
    Wikidata search packet, structured screen, and 2 source-only records for
    Arbanon and Béarn with title facts deferred for rank semantics.
53. CK3 batch 003 research screen. Complete for a third 50-row Wikidata search
    packet and structured screen, with no bounded top-lead candidates promoted.
54. CK3 batch 004 reviewed packet. Complete for a fourth 50-row Wikidata
    search packet, structured screen, 3 reviewed source records, and 6 title
    facts for Brycheiniog and Cetatea-Albă County, with Byzantium title facts
    deferred.
55. CK3 batch 005 research screen. Complete for a fifth 50-row Wikidata search
    packet and structured screen, with no bounded top-lead candidates promoted.
56. CK3 batch 006 source-resolution packet. Complete for a sixth 50-row
    Wikidata search packet, structured screen, and one source-only Donji Kraji
    record, while recognizing Duklja as already accepted.
57. CK3 batch 007 research screen. Complete for a seventh 50-row Wikidata
    search packet and structured screen, with no bounded top-lead candidates
    promoted.
58. CK3 queue closure. Complete for batches 008-010 research screens, Hordaland
    title promotion, and closure of the 500 CK3 discovery rows with zero pending
    candidates.
59. Accepted relation bridges parentage packet. Complete for 4 parentage-only
    facts using already accepted source records and structured `P17`/`P361`
    relation review.
60. Second accepted relation bridges parentage packet. Complete for 11
    parentage-only facts using already accepted source records and non-
    overlapping structured `P17` relation review.
61. Parentage coverage report. Complete for a repeatable CLI report that
    summarizes parentage coverage by rank and lists title hierarchy gaps.
62. Parentage gap TSV queue. Complete for a machine-readable 211-row queue of
    unparented accepted titles with rank-based review priority.
63. Parentage gap sharding and reports. Complete for 9 fixed-size review
    batches and Markdown reports across the 211-row parentage gap queue.
64. Composite crown parentage packet. Complete for a `Crown` rank between
    kingdom and empire, plus the reviewed Kingdom of Murcia -> Crown of Castile
    parentage bridge from parentage-gap shard 001.
65. Crown of Aragon parentage packet. Complete for a reviewed County of
    Barcelona -> Crown of Aragon bridge from parentage-gap shard 001.
66. Crown bridge parentage packet. Complete for 10 parentage-only kingdom ->
    crown bridges using already accepted Crown of Aragon and Crown of Castile
    source records.
67. Polish-Lithuanian Commonwealth rank correction and Livonia parentage.
    Complete for correcting Q172107 to `Crown` and importing Duchy of Livonia
    -> Polish-Lithuanian Commonwealth for 1569-1621.
68. Commonwealth children parentage packet. Complete for Crown of the Kingdom
    of Poland and Grand Duchy of Lithuania under Polish-Lithuanian
    Commonwealth for 1569-1795.
69. Commonwealth boundary parentage packet. Complete for the year-granular
    Kingdom of Poland -> Polish-Lithuanian Commonwealth transition in 1569.
70. Theocratic state parentage packet. Complete for adding
    `TheocraticState` rank support, promoting Papal States, and importing
    Comtat Venaissin -> Papal States for 1274-1791.
71. Kingdom of Poland German Empire parentage packet. Complete for importing
    the reviewed 1916-1918 Kingdom of Poland -> German Empire relation.
72. Russian Empire Finland parentage packet. Complete for promoting Russian
    Empire and importing Grand Duchy of Finland -> Russian Empire for
    1809-1917.
73. Gorizia and Gradisca Austrian parentage packet. Complete for correcting
    Q692946 to County rank and importing Gorizia and Gradisca -> Austrian
    Empire for 1804-1866 and -> Austria-Hungary for 1867-1918.
74. Duchy of Urbino Papal States parentage packet. Complete for importing the
    reviewed Duchy of Urbino -> Papal States relation for 1443-1631.
75. Illyrian Provinces French Empire parentage packet. Complete for adding
    `Province` rank support, correcting Q699923 to Province, and importing
    Illyrian Provinces -> First French Empire for 1809-1815.
76. Electoral Palatinate HRE parentage packet. Complete for correcting Q22880
    to Duchy rank and importing Electoral Palatinate -> Holy Roman Empire for
    1085-1803.
77. Free Imperial City of Aachen HRE parentage packet. Complete for adding
    `FreeCity` rank support, correcting Q2629137 to FreeCity, and importing
    Free Imperial City of Aachen -> Holy Roman Empire for 1306-1801.
78. Burgraviate of Nuremberg HRE parentage packet. Complete for correcting
    Q568473 to County rank and importing Burgraviate of Nuremberg -> Holy Roman
    Empire for 1105-1440.
79. Courland Commonwealth parentage packet. Complete for importing Duchy of
    Courland and Semigallia -> Polish-Lithuanian Commonwealth for 1569-1795.
80. Estonia Sweden parentage packet. Complete for promoting Sweden and importing
    Duchy of Estonia -> Sweden for 1561-1721.
81. Schleswig Denmark parentage packet. Complete for promoting Denmark and
    importing Duchy of Schleswig -> Denmark for 1058-1866.
82. Burgundian Netherlands State parentage packet. Complete for promoting
    Burgundian State and importing Burgundian Netherlands -> Burgundian State
    for 1384-1482.
83. Hispanic Monarchy crown parentage packet. Complete for promoting Hispanic
    Monarchy and importing Crown of Aragon/Crown of Castile -> Hispanic
    Monarchy for 1479-1700.
84. Kalmar Union Sweden parentage packet. Complete for correcting Kalmar Union
    to composite Crown rank and importing Sweden -> Kalmar Union for 1397-1523.
85. Prince-Bishopric of Trent HRE parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Prince-Bishopric of Trent ->
    Holy Roman Empire for 1027-1803.
86. County of Savoy HRE parentage packet. Complete for adding a reviewed
    Wikimedia text source and importing County of Savoy -> Holy Roman Empire
    for 1003-1416.
87. Duchy of Warsaw French Empire parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Duchy of Warsaw -> First
    French Empire for 1807-1815.
88. Confederation of the Rhine Westphalia parentage packet. Complete for
    promoting Confederation of the Rhine and importing Kingdom of Westphalia ->
    Confederation of the Rhine for 1807-1813.
89. Confederation of the Rhine French Empire parentage packet. Complete for
    adding a reviewed Wikimedia text source and importing Confederation of the
    Rhine -> First French Empire for 1806-1813.
90. Prince-Bishopric of Augsburg HRE parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Prince-Bishopric of Augsburg ->
    Holy Roman Empire for 962-1803.
91. Kingdom of Imereti Russian Empire parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Kingdom of Imereti -> Russian
    Empire for 1804-1810.
92. Italian protectorate of Albania Italian Empire parentage packet. Complete
    for promoting Italian Empire and importing Italian protectorate of Albania ->
    Italian Empire for 1939-1943.
93. Francia Carolingian Empire parentage packet. Complete for adding a reviewed
    Wikimedia text source and importing Francia -> Carolingian Empire for
    800-843.
94. Duchy of Prussia Polish Crown parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Duchy of Prussia -> Crown of
    the Kingdom of Poland for 1525-1618.
95. County of Empuries Crown of Aragon parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing County of Empuries -> Crown of
    Aragon for 1341-1402.
96. Grand Duchy of Tuscany HRE parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Grand Duchy of Tuscany -> Holy
    Roman Empire for 1575-1801.
97. Duchy of Parma and Piacenza Empire parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Duchy of Parma and Piacenza ->
    Holy Roman Empire for 1748-1801.
98. Kingdom of Naples Crown of Aragon parentage packet. Complete for adding a
    reviewed Wikimedia text source and importing Kingdom of Naples -> Crown of
    Aragon for 1442-1458.
99. Grand Principality of Moscow Golden Horde parentage packet. Complete for
    promoting Golden Horde and importing Grand Principality of Moscow -> Golden
    Horde for 1263-1478.
100. County of Astarac Duchy of Gascony parentage packet. Complete for
     promoting Duchy of Gascony and importing County of Astarac -> Duchy of
     Gascony for 920-1300.
101. Duchy of Bohemia HRE parentage packet. Complete for adding a reviewed
     Wikimedia text source and importing Duchy of Bohemia -> Holy Roman Empire
     for 1002-1198.
102. Duchy of Florence HRE parentage packet. Complete for adding a reviewed
     Wikimedia text source and importing Duchy of Florence -> Holy Roman Empire
     for 1532-1569.
103. Prince-Bishopric of Freising HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Prince-Bishopric of Freising
     -> Holy Roman Empire for 1294-1802.
104. Prince-Bishopric of Basel HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Prince-Bishopric of Basel ->
     Holy Roman Empire for 1032-1803.
105. Prince-Bishopric of Toul HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Prince-Bishopric of Toul ->
     Holy Roman Empire for 1048-1648.
106. Margraviate of Austria HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Margraviate of Austria ->
     Holy Roman Empire for 976-1156.
107. Grand Duchy of Berg Rhine Confederation parentage packet. Complete for
     adding a reviewed Wikimedia text source and importing Grand Duchy of Berg
     -> Confederation of the Rhine for 1806-1813.
108. Duchy of Modena and Reggio HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Duchy of Modena and Reggio
     -> Holy Roman Empire for 1452-1796.
109. County of Wurttemberg HRE parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing County of Wurttemberg ->
     Holy Roman Empire for 1083-1495.
110. Duchy of Austria HRE parentage packet. Complete for adding a reviewed
     Wikimedia text source and importing Duchy of Austria -> Holy Roman Empire
     for 1156-1453.
111. Duchy of Athens Thessalonica parentage packet. Complete for adding a
     reviewed Wikimedia text source and importing Duchy of Athens -> Kingdom of
     Thessalonica for 1205-1224.
112. Kingdom of Thessalonica Latin Empire parentage packet. Complete for adding
     a reviewed Wikimedia text source and importing Kingdom of Thessalonica ->
     Latin Empire for 1204-1224.
113. Duchy of Cleves HRE parentage packet. Complete for adding a reviewed
     Wikimedia text source and importing Duchy of Cleves -> Holy Roman Empire
     for 1092-1795.
114. County of Luxembourg HRE parentage packet. Complete for adding a reviewed
     Wikimedia text source and importing County of Luxembourg -> Holy Roman
     Empire for 963-1354.
115. High-priority HRE county/duchy parentage batch 01. Complete for adding
     four reviewed Wikimedia text sources and importing Duchy of Westphalia,
     County of Ravensberg, County of Montbeliard, and County of Namur under the
     Holy Roman Empire.
116. High-priority HRE county/duchy parentage batch 02. Complete for adding
    Bavaria, Geneva, and Lower Lotharingia parentage under the Holy Roman
    Empire.
117. High-priority mixed parentage batch 03. Complete for adding Julich,
    Wurzburg, Frankfurt, and Normandy parentage spans under the Holy Roman
    Empire, Confederation of the Rhine, and Kingdom of France.
118. High-priority Bohemian/Silesian parentage batch 04. Complete for adding
    Opava, Silesia, Teschen, Bytom, and Oswiecim parentage spans under the
    Kingdom of Bohemia.
119. High-priority mixed parentage batch 05. Complete for adding Nassau-Siegen
    Holy Roman Empire spans and Grand Principality of Vladimir Golden Horde
    parentage.
120. High-priority mixed parentage batch 06. Complete for adding Belz,
    Lotharingia, Serbia, United Principalities, and Prince-Bishopric of
    Montenegro parentage spans.
121. High-priority mixed parentage batch 07. Complete for adding Courland,
    Moravian Serbia, Amalfi, Ferrara, and Gascony endpoint/vassalage parentage
    spans.
122. High-priority mixed parentage batch 08. Complete for adding County of
    Sicily Kingdom of Sicily and County of Nice Kingdom of Sardinia endpoint
    spans.
123. High-priority endpoint parentage batch 09. Complete for adding Duchy of
    Poland Kingdom of Poland and Polish-Lithuanian union Commonwealth endpoint
    spans.
124. High-priority Norway parentage batch 10. Complete for adding modern
    Kingdom of Norway coverage and Hordaland parentage.
125. Remaining high-priority blocker review. Complete for recording Albania and
    Montenegro blockers that need contested/de facto relation modeling.
126. Parentage gap blocker queue. Complete for adding a machine-readable
    blocker TSV and optional gap-queue input so reviewed blockers remain
    visible without counting as active high-priority parentage imports.
127. Medium-priority Napoleonic parentage batch 01. Complete for adding Kingdom
    of Holland, Kingdom of Etruria, and Napoleonic Kingdom of Italy parentage
    under the First French Empire.
128. Medium-priority Sicily Aragon parentage batch 02. Complete for adding
    Kingdom of Sicily parentage under the Crown of Aragon.
129. Medium-priority Eastern Hungary Ottoman parentage batch 03. Complete for
    adding Eastern Hungarian Kingdom parentage under the Ottoman Empire.
130. Medium-priority Livonia Russia parentage batch 04. Complete for promoting
    Tsardom of Russia and adding Kingdom of Livonia parentage under it.
131. Medium-priority Bosnia Ottoman parentage batch 05. Complete for adding a
    Kingdom of Bosnia endpoint parentage span under the Ottoman Empire.
132. Medium-priority Albanian Kingdom Italian Empire parentage batch 06.
    Complete for adding an Albanian Kingdom endpoint parentage span under the
    Italian Empire.
133. Parentage change metrics report. Complete for adding a source-backed
    report that counts parent changes by child rank and parent rank, including
    the county baseline needed before scaling county import.
134. County parentage scale queue. Complete for classifying 500 CK3 Europe
    county search-driver rows into agent-ready shards with source-custody
    statuses and shard worker instructions.
135. County-scale shard 004 farm review. Complete for recording the first
    farmed shard output: 50 rows reviewed, with Brycheiniog and Byzantion
    blocked by relation/rank policy and Cetatea-Alba retained as an accepted
    metrics seed.
136. County-scale priority shard farm review. Complete for recording shard
    reviews 001, 002, 006, and 009, raising farm-reviewed rows to 250 and
    surfacing six title/title-follow-up candidates.
137. County-scale full shard farm review. Complete for recording shard reviews
    003, 005, 007, 008, and 010, closing the 500-row farm pass with 21
    title/title-follow-up candidates and zero ready parentage packets.
138. County title harvest batch 01. Complete for promoting Dorohoi County,
    County Palatine of Durham, and County of Girona title-only facts from the
    county-scale farm while deferring conflicted or under-sourced candidates.
139. County title harvest batch 02. Complete for promoting Duchy of Benevento,
    Taifa of Denia, Kingdom of Desmond, and Princely Abbey of Fulda title-only
    facts while correcting spans to the opened source evidence.
140. County title harvest batch 03. Complete for promoting Free imperial city
    of Dortmund as a corrected title-only packet from Britannica evidence.
141. County title harvest batch 04. Complete for promoting County of La Marche
    as a corrected title-only packet using the first listed count for the
    bounded start and 1527 crown seizure for the endpoint.
142. County title harvest closure. Complete for reconciling all 21 ready
    title/title-follow-up candidates from the county-scale farm into already
    accepted, promoted, or deferred states.
143. Dorohoi Romania parentage. Complete for importing a reviewed
    `1881..1947` Dorohoi County parentage span under the Kingdom of Romania,
    using already accepted Dorohoi and Kingdom of Romania sources.
144. Benevento Lombards parentage. Complete for importing a reviewed
    `577..774` Duchy of Benevento parentage span under the Kingdom of the
    Lombards.
145. La Marche French crown parentage. Complete for importing a reviewed
    `1527..1527` County of La Marche endpoint parentage span under the Kingdom
    of France.
146. Girona Carolingian parentage. Complete for importing a reviewed
    `800..887` County of Girona parentage span under the Carolingian Empire,
    closing the active post-harvest high-priority parentage queue.
147. Britain Union parentage. Complete for correcting Kingdom of Great Britain
    to DUCHY's composite `Crown` rank and importing reviewed `1707..1707`
    endpoint parentage from the Kingdoms of England and Scotland to Great
    Britain.
148. Dortmund HRE parentage. Complete for importing a reviewed `1220..1803`
    Free imperial city of Dortmund parentage span under the Holy Roman Empire.
149. Fulda HRE parentage. Complete for importing a reviewed `1221..1802`
    Princely Abbey of Fulda parentage span under the Holy Roman Empire.
150. Ireland UKGBI parentage. Complete for promoting United Kingdom of Great
    Britain and Ireland as a bounded `Crown` title and importing Kingdom of
    Ireland endpoint parentage under it for `1801..1801`.
151. Kalmar Denmark Norway parentage. Complete for importing Denmark under the
    Kalmar Union for `1397..1523` and medieval Norway under the Kalmar Union for
    `1397..1397`.
152. Serbia Empire parentage. Complete for promoting the Serbian Empire as a
    bounded `Empire` title and importing medieval Kingdom of Serbia endpoint
    parentage under it for `1346..1346`.

Real historical title data may be imported only after the concrete source record
passes the source-custody review gate.

## Wave 3: Game Scenario Integration

Outcome: DUCHY exports game-ready political scenarios while keeping mechanics
local to the consuming game.

Pulses:

1. Fictional realm generator for safe test data.
2. BANISH/QUEST/TIGRIS scenario handoff candidates.
3. COURT snapshot candidate for title timeline inspection.
4. Balance and design rubric for playable political pressure.

## Dependency Placement

DUCHY is a Games Design product/data-model repo. It is not a primitive that
other repos should depend on during the foundation wave.

Planned later:

- PROOF for Markdown/report validation.
- CROP/PEBBLE/FLETCH for source-backed packs after source custody exists.
- RLINE only if repeated timeline or graph primitives prove product-neutral.
- COURT/RACKET/MUDDLE only after the timeline model has playable inspection
  needs.

Out of scope now:

- RPLAN/RCOUNT election contracts.
- METIS graph partitioning.
- runtime dependency on BANISH, QUEST, TIGRIS, or any product repo.

## Non-Goals

- DUCHY is not a historical authority.
- DUCHY is not a clone of Crusader Kings data, UI, rules, or simulation.
- DUCHY will not import bulk historical data without source review.
- DUCHY will not move game-specific scoring or war mechanics into the core
  timeline model.
