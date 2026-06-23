# DUCHY Interfaces

## Scope

Foundation interfaces for lineage and transfer queries. This file defines the
intended answer surfaces before CLI/API behavior is locked.

## Interface Items

| ID | Interface | Purpose | Status |
|---|---|---|---|
| IF-001 | `TitleTimeline` model | Store title identity, ranks, existence spans, parentage, control, and continuity events. | implemented foundation |
| IF-002 | temporal parentage relation | Store parent-child title/area relationships as date spans or events. | planned |
| IF-003 | area identity relation | Link a stable area/place identity to one or more titles over time. | planned |
| IF-004 | `title_path_in_year(area_or_title, year)` | Return county -> duchy -> kingdom -> empire path for a year. | planned |
| IF-005 | `transfers_between(area_or_title, rank, start, end)` | Return ordered parentage/control transfers within a range. | planned |
| IF-006 | `lineage_for_title(title_id)` | Return ordered continuity and parentage/control events. | planned |
| IF-007 | answer trace object | Explain matched spans, events, source class, confidence, and gaps. | planned |
| IF-008 | fixture source/confidence fields | Mark seed, fictional, source-backed, contested, uncertain, or unsupported rows. | implemented for first source-backed facts |
| IF-009 | source-backed title materializer | Convert reviewed name/rank/existence fact sets into `Title` records. | implemented first slice |
| IF-010 | source-backed title-path query | Return a traced answer envelope for source-backed title records. | implemented first slice |
| IF-011 | fact fixture parser | Parse reviewed source-backed fact records from dependency-free text fixtures. | implemented first slice |
| IF-012 | contested fact review packet | Group contested claims and block normal materialization until resolution. | implemented first slice |
| IF-013 | source-backed parentage materializer | Convert reviewed parentage fact records into temporal `ParentageSpan` relations. | implemented with first real parentage import |
| IF-014 | reviewed source fixture parser | Parse real reviewed source records that authorize fact fixtures. | implemented first slice |
| IF-015 | source-backed relation fact materializer | Convert reviewed non-parentage relation facts into typed temporal relation records that can appear in query traces without replacing parentage. | implemented materializer slice |

## Answer Shape

Lineage answers should eventually include:

| Field | Meaning |
|---|---|
| `query` | Normalized question inputs. |
| `status` | `answered`, `empty`, `unknown`, `unsupported`, or `contested`. |
| `year` / `range` | Date scope. |
| `title_path` | Ordered titles from local area/title to highest known parent. |
| `holder` | De facto holder/control where known. |
| `events` | Continuity or transfer events explaining the answer. |
| `trace` | Matched spans, fixture rows, source class, and confidence. |

## Boundary Rules

- Query interfaces must not imply historical authority without source evidence.
- Parentage-path queries and holder/control queries are separate operations even
  if a UI later displays them together.
- Transfer queries must report intermediate changes rather than returning only
  the latest parent. Contested/split states are reserved in the status taxonomy
  and remain deferred until fixtures express them.
- Area identity and title identity must remain separable so a place can move
  between duchies without losing its continuity.
- Non-parentage relation facts must not silently alter title paths. They may
  explain rank skips, dependency, membership, subdivision, or transition context
  in the trace only when source-backed and typed.
