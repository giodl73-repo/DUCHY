# DUCHY Mission

## Scope

| Field | Value |
|---|---|
| Repo | DUCHY |
| VTRACE stage | Mission / Need |
| VTRACE adoption scope | Define the lineage-query mission before expanding fixtures, query APIs, source imports, or game scenario packets. |
| Stage status | Foundation baseline for requirements and work packages. |

## Mission Need

DUCHY exists because CK-like strategy and world-building work needs political
geography that changes over time:

```text
area identity
  -> title hierarchy by year
  -> de jure and de facto parentage
  -> continuity events and transfers
  -> explainable lineage answer
```

The core mission is to answer questions such as:

- Which duchy contained this county in 1066?
- When did this area move from one duchy to another?
- Which kingdom claimed this duchy de jure while another ruler controlled it?
- What continuity events explain this title's lineage?
- Which areas changed higher-title parentage during a date range?

DUCHY is a Games Design repo. It should support historical-strategy design and
fictional realm generation without claiming historical authority or copying a
commercial game's data or rules.

## Users

| User | Need | Success Signal |
|---|---|---|
| Game designer | Ask how territorial hierarchy changes affect a scenario. | Can inspect transfers and lineage events without reading raw fixture rows. |
| World builder | Track fictional or source-backed counties, duchies, and kingdoms over time. | Can preserve identity across rename, transfer, partition, and extinction events. |
| Tabletop campaign author | Understand dynastic and territorial continuity for a date or range. | Can ask holder/parentage/history questions with explainable results. |
| Source reviewer | Separate historical claims from seed/demo data. | Answers carry source/confidence posture when real data exists. |
| Future agent | Resume work from documented requirements and trace rows. | Work packages name exact code, fixture, and validation surfaces. |

## Constraints

- Foundation fixtures may be fictional or non-authoritative only.
- Real European data requires source custody, citation, and confidence posture.
- De jure parentage and de facto control must remain distinct.
- Title identity must remain stable across holder changes, name changes, and
  rank/parentage changes where the model permits them.
- DUCHY must not copy commercial-game databases, maps, UI, rules, or language.
- Game mechanics such as war, economy, diplomacy, succession scoring, and
  balance remain out of scope until a later pulse explicitly owns them.

## Non-Goals

- No historical authority claim.
- No bulk historical corpus import in the lineage-query foundation.
- No map renderer or grand-strategy engine.
- No commercial-game clone.
- No shared-kernel extraction until repeated consumer needs prove it.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| DUCHY has stable requirements for lineage and transfer questions. | Role review and trace inspection. | `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/TRACE.md`. |
| Foundation queries distinguish parentage, control, and continuity events. | Unit tests and fixture review. | Future WP-001/WP-002 evidence. |
| Historical-source boundaries are explicit before import. | Source Custody review. | `.roles/source-custody-reviewer.md`, deferred source package. |
| Work can continue without chat history. | VTRACE work-package inspection. | `docs/vtrace/WORK_PACKAGES.md`. |
