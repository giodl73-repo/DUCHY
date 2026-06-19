# Wave: Foundation Timeline Contract

## Goal

Create the DUCHY repository foundation and the first tested contract for
year-bounded political title timelines.

## Thesis

DUCHY should prove its core data shape before it imports maps, historical
sources, or game mechanics. A small validated timeline model gives later work a
stable place to attach title ranks, parentage, holder changes, contested claims,
and scenario packets.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | complete | Create repo skeleton, docs, skills, and first tested timeline contract. |
| 02 | Governance and VTRACE baseline | complete | Add role governance and VTRACE requirements for lineage/transfer questions. |
| 03 | Temporal area/title model | complete | Add stable area identity, temporal parentage spans, and transfer baseline fixtures. |
| 04 | Year title-path query | complete | Answer which higher title path contained an area/title in a year. |
| 05 | Duchy-transfer query | complete | Answer when areas/titles moved between duchies across a date range. |
| 06 | Answer trace and source status | complete | Distinguish answered, empty, unknown, unsupported, seed, and reserved contested/source-backed answers. |

## Success Criteria

- README explains DUCHY's purpose and first command.
- Product plan names waves, dependencies, and non-goals.
- Wave/pulse scaffolding exists.
- Repo-specific skills exist for future wave, pulse, and research work.
- Seed timeline validates in tests.
- `.roles/` governs lineage, source, game-system, and query-interface reviews.
- `docs/vtrace/` maps lineage/transfer questions to requirements and work
  packages.
- Validation commands pass.
