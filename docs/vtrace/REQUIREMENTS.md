# DUCHY Requirements

## Scope

Lineage and territorial-transfer question support for DUCHY.

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | DUCHY shall represent stable area/title identity separately from title name, holder, rank, and parentage. | Mission / CONOPS-001 / CONOPS-003 | Lineage queries need identity continuity across historical changes. | must | Timeline Steward | test / inspection | accepted |
| REQ-002 | DUCHY shall distinguish de jure parentage from de facto holder/control for every yearly answer where data exists. | CONOPS-004 | CK-like questions depend on claims and control not being collapsed. | must | Timeline Steward | test / review | accepted |
| REQ-003 | DUCHY shall answer which higher title path contained an area/title in a specified year. | CONOPS-001 | This is the core "where was this county/area in year Y" question. | must | Territorial Lineage Reviewer | test / demonstration | accepted |
| REQ-004 | DUCHY shall answer when an area/title moved between duchies across a date range. | CONOPS-002 | Transfer discovery is the user's stated target capability. | must | Territorial Lineage Reviewer | test / demonstration | accepted |
| REQ-005 | DUCHY shall expose ordered continuity events for a title lineage. | CONOPS-003 | Users need an explanation, not only a final state. | must | Timeline Steward | test / inspection | accepted |
| REQ-006 | DUCHY shall label answer source class and confidence once non-seed historical data is introduced. | CONOPS-005 | Historical claims require provenance and uncertainty posture. | must later | Source Custody Reviewer | review / inspection | proposed |
| REQ-007 | DUCHY shall distinguish empty, unsupported, unknown, and contested answers where the fixture expresses those states. | CONOPS-005 | Query consumers need reliable negative answers. | should | Query Interface Reviewer | test | accepted |
| REQ-008 | DUCHY shall keep game mechanics out of the core lineage model unless a later VTRACE package accepts them. | Mission constraints | Prevents clone risk and mechanics leakage. | must | Game Systems Reviewer | review | accepted |
| REQ-009 | DUCHY shall represent time-varying parentage as spans or events rather than overwriting a title's current parent. | CONOPS-001 / CONOPS-002 | Areas moving between duchies cannot be reconstructed from a single static parent field. | must | Timeline Steward | test / inspection | accepted |
| REQ-010 | DUCHY shall support area identity separately from title identity when a place, title, holder, or parentage changes independently. | CONOPS-002 / CONOPS-003 | The user asks about areas moving between duchies, not only title records changing labels. | must | Territorial Lineage Reviewer | test / review | accepted |
| REQ-011 | DUCHY shall define seed fixtures that include at least one no-transfer case, one single-transfer case, and one multi-transfer case before accepting range-transfer behavior. | CONOPS-002 / CONOPS-005 | Transfer queries need positive and negative fixtures before implementation claims. | must | Query Interface Reviewer | test / inspection | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| REQ-SRC-001 | Bulk real historical source import needs its own source-custody package. | Before adding non-seed European title datasets. |
| REQ-MAP-001 | Geometry/map rendering is outside the lineage-query foundation. | When DUCHY needs visual maps or GIS source handling. |
| REQ-GAME-001 | War, diplomacy, succession scoring, and economy are mechanics, not foundation lineage. | When a game consumer needs playable simulation behavior. |
