# DUCHY Trace Matrix

| Requirement ID | Parent Need | Requirement | Specification Item | Design Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | Mission / CONOPS-003 | Stable title identity is separate from mutable title attributes. | SPEC-001 title identity contract | IF-001 `TitleTimeline` | CR-001 deterministic fixture behavior | WP-001 | `src/lib.rs`, fixtures | test / inspection | lineage scenario review | current unit tests; future fixture tests | partially implemented |
| REQ-002 | CONOPS-004 | De jure parentage and de facto holder/control remain distinct. | SPEC-002 parentage/control separation | IF-001, IF-002 | CR-001 | WP-001 / WP-002 | `Title`, `ControlSpan`, future answer object | test / review | holder vs claim scenario | current seed model; future query tests | partially implemented |
| REQ-003 | CONOPS-001 | Answer higher-title path for a year. | SPEC-003 yearly title-path query | IF-004 | CR-002 traceable answer shape | WP-003 | `title_path_in_year`, fixtures | test / demonstration | designer asks county-in-year question | pending | accepted |
| REQ-004 | CONOPS-002 | Answer transfers between duchies over a range. | SPEC-004 range transfer query | IF-005 | CR-002 | WP-004 | `transfers_between`, transfer fixtures | test / demonstration | designer asks moved-between-duchies question | pending | accepted |
| REQ-005 | CONOPS-003 | Expose ordered continuity events. | SPEC-005 lineage event query | IF-006 | CR-001 | WP-003 | `events_for_title`, future lineage answer | test / inspection | title-lineage scenario | current `events_for_title`; future answer trace | partially implemented |
| REQ-006 | CONOPS-005 | Label source class and confidence for historical data. | SPEC-006 source/confidence fields | IF-007, IF-008 | CR-003 no restricted-source copying | deferred source package | docs/source plan, future fixture schema | review / inspection | source-review scenario | pending | proposed |
| REQ-007 | CONOPS-005 | Distinguish empty, unsupported, unknown, and contested answers. | SPEC-007 answer status taxonomy | IF-007 | CR-002 | WP-005 | answer trace object, negative fixtures | test | negative-answer scenario | pending | accepted |
| REQ-008 | Mission constraints | Keep game mechanics out of lineage core. | SPEC-008 product boundary | README, PRODUCT_PLAN, roles | CR-004 boundary review | WP-001 / ongoing | docs and review gates | review | game-systems review | `.roles/game-systems-reviewer.md` | accepted |
| REQ-009 | CONOPS-001 / CONOPS-002 | Represent time-varying parentage as spans/events. | SPEC-009 temporal parentage contract | IF-002 | CR-001 | WP-002 | parentage spans/events and tests | test / inspection | transfer scenario review | pending | accepted |
| REQ-010 | CONOPS-002 / CONOPS-003 | Support area identity separately from title identity. | SPEC-010 area identity contract | IF-003 | CR-001 | WP-002 | area-title links and tests | test / review | territorial lineage review | pending | accepted |
| REQ-011 | CONOPS-002 / CONOPS-005 | Define no-transfer, single-transfer, and multi-transfer fixtures. | SPEC-011 transfer fixture baseline | fixtures/tests | CR-001, CR-002 | WP-002 / WP-004 | seed fixtures and query tests | test / inspection | query-interface review | pending | accepted |

## Code Rigor Constraints

| ID | Constraint | Applies To | Verification |
|---|---|---|---|
| CR-001 | The same fixture and query must produce deterministic results. | All query APIs and fixtures. | Unit tests. |
| CR-002 | Query answers must expose enough trace to diagnose why the answer was returned. | IF-004 through IF-007. | Tests and role review. |
| CR-003 | Historical source rows must not copy restricted source data or commercial-game data. | Source-backed fixtures. | Source Custody review. |
| CR-004 | Core lineage code must not absorb game mechanics without a later VTRACE gate. | Core model and query APIs. | Game Systems review. |
