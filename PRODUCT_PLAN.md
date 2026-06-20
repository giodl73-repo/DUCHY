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
39. Next reviewed parentage packet.

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
