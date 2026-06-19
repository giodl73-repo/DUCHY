# DUCHY Interfaces

## Scope

Foundation interfaces for lineage and transfer queries. This file defines the
intended answer surfaces before CLI/API behavior is locked.

## Interface Items

| ID | Interface | Purpose | Status |
|---|---|---|---|
| IF-001 | `TitleTimeline` model | Store title identity, ranks, existence spans, parentage, control, and continuity events. | implemented foundation |
| IF-002 | `title_path_in_year(area_or_title, year)` | Return county -> duchy -> kingdom -> empire path for a year. | planned |
| IF-003 | `transfers_between(area_or_title, rank, start, end)` | Return ordered parentage/control transfers within a range. | planned |
| IF-004 | `lineage_for_title(title_id)` | Return ordered continuity and parentage/control events. | planned |
| IF-005 | answer trace object | Explain matched spans, events, source class, confidence, and gaps. | planned |
| IF-006 | fixture source/confidence fields | Mark seed, fictional, source-backed, contested, uncertain, or unsupported rows. | deferred to source package |

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
- Transfer queries must report intermediate changes and contested/split states
  rather than returning only the latest parent.
