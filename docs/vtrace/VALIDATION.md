# DUCHY Validation Plan

## Scope

Validation scenarios for user-facing lineage and territorial-transfer questions.

## Validation Scenarios

| Scenario ID | User / Actor | Need | Workflow | Success Criteria | Evidence Pointer | Result |
|---|---|---|---|---|---|---|
| VAL-001 | Game designer | Find which duchy contains a county in a year. | Ask title-path query for county C in year Y. | Answer returns county and duchy, plus higher titles where known. | `cargo test --quiet`; `cargo run --quiet` on 2026-06-19 | pass |
| VAL-002 | Game designer | Find areas that moved between duchies. | Ask transfer query for area/title over date range. | Answer lists ordered transfers with from/to titles and dates. | `cargo test --quiet`; `cargo run --quiet` on 2026-06-19 | pass |
| VAL-003 | World builder | Explain title lineage. | Ask lineage query for title T. | Answer shows continuity events and relevant parent/control changes. | `TitlePathAnswer.events` test on 2026-06-19 | pass |
| VAL-004 | Campaign author | Separate control from legal claim. | Ask holder and de jure parent question for title/year. | Answer shows de facto holder separately from de jure parent path. | WP-003 fixture/demo | pending |
| VAL-005 | Source reviewer | Know whether answer is source-backed. | Inspect answer trace for fixture/source class. | Historical data has source/confidence labels; seed data is labeled seed. | WP-005/WP-006 | deferred |

## Acceptance Evidence

| Evidence ID | Scenario ID | Evidence | Result |
|---|---|---|---|
| EVID-VAL-001 | VAL-001 | Future CLI/API example for county-in-year. | pending |
| EVID-VAL-002 | VAL-002 | Future transfer fixture and test output. | pending |
| EVID-VAL-003 | VAL-003 | Future lineage answer trace. | pending |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Real European historical title coverage. | Requires source custody, citation, and confidence model. | Without this, answers may be mistaken for authoritative history. | Before importing non-seed historical datasets. |
| Map/geometry validation. | Lineage foundation is title/time based, not GIS based. | Area identity may later need geometry or source-boundary IDs. | Before adding visual maps or GIS data. |
| Game-balance validation. | Mechanics are out of scope. | Designers might overread timeline data as playable balance. | Before adding simulation or strategy scoring. |
