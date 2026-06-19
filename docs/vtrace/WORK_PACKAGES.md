# DUCHY Work Packages

## Scope

Work packages for lineage and territorial-transfer query capability.

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|
| WP-001 | Baseline governance and VTRACE package. | REQ-001, REQ-002, REQ-008 | `.roles/`, `docs/vtrace/`, README links | Foundation repo exists. | Roles and VTRACE files added; validation passes. | L0: docs inspect / L1: full current commands / L2: n/a | complete |
| WP-002 | Temporal area/title model and fixture baseline. | REQ-001, REQ-002, REQ-009, REQ-010, REQ-011 | `src/lib.rs`, seed fixtures/tests | WP-001 complete. | Time-varying parentage and area-title identity are represented; fixtures cover no-transfer, single-transfer, and multi-transfer cases. | L0: cargo test / L1: role review / L2: n/a | complete |
| WP-003 | Year title-path and lineage answer query. | REQ-001, REQ-002, REQ-003, REQ-005, REQ-009, REQ-010 | `src/lib.rs`, tests, README examples | WP-002 complete. | Query returns title path and ordered lineage events with tests. | L0: cargo test / L1: cargo run + tests / L2: role review | complete |
| WP-004 | Duchy-transfer range query. | REQ-004, REQ-009, REQ-010, REQ-011 | `src/lib.rs`, transfer fixtures/tests | WP-003 answer shape stable. | Query lists movements between duchies over date range. | L0: cargo test / L1: demo output / L2: role review | proposed |
| WP-005 | Answer trace and status taxonomy. | REQ-006, REQ-007, REQ-011 | answer types, negative fixtures, docs | WP-003 and WP-004 query semantics known. | Answers distinguish empty, unknown, unsupported, contested, seed, and source/confidence classes. | L0: cargo test / L1: fixture demos / L2: Source Custody review | proposed |
| WP-006 | Source-custody VTRACE package for real historical data. | REQ-006, deferred REQ-SRC-001 | `docs/vtrace/source-*`, future fixtures | Query surface stable. | Source policy, confidence model, and allowed import path accepted. | L0: docs review / L1: custody review / L2: source package gate | deferred |

## Work Package Details

### WP-001: Baseline Governance And VTRACE Package

Objective: create DUCHY role governance and define lineage-query requirements.

Parent requirement IDs: REQ-001, REQ-002, REQ-008.

Affected files/modules:

- `.roles/`
- `docs/vtrace/`
- `README.md`
- `PRODUCT_PLAN.md`

Exit criteria:

- Role files exist for timeline, territorial lineage, source custody, game
  systems, and query interface review.
- VTRACE mission, CONOPS, requirements, interfaces, trace, verification,
  validation, and work-package files exist.
- Current validation commands pass.

Verification commands:

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```

Status: complete.

### WP-002: Temporal Area/Title Model And Fixture Baseline

Objective: make the data model capable of representing areas that move between
duchies before implementing user-facing transfer queries.

Parent requirement IDs: REQ-001, REQ-002, REQ-009, REQ-010, REQ-011.

Affected files/modules:

- `src/lib.rs`
- seed fixture constructors
- model tests

Exit criteria:

- parentage can vary over time as spans or events,
- stable area/place identity can be linked to title identity over time,
- fixtures cover no-transfer, single-transfer, and multi-transfer cases,
- Timeline Steward and Territorial Lineage Reviewer findings are recorded.

Status: complete.

### WP-003: Year Title-Path And Lineage Answer Query

Objective: let a caller ask which higher-title path contained a county/area in a
year and inspect title continuity events.

Parent requirement IDs: REQ-001, REQ-002, REQ-003, REQ-005, REQ-009, REQ-010.

Affected files/modules:

- `src/lib.rs`
- `src/main.rs`
- README examples
- query tests

Exit criteria:

- `title_path_in_year` or equivalent returns ordered title path.
- lineage answer includes ordered continuity events.
- tests cover at least one holder change and one parentage path.
- Territorial Lineage Reviewer and Query Interface Reviewer findings are
  recorded in pulse notes or VTRACE review.

Status: complete.

### WP-004: Duchy-Transfer Range Query

Objective: let a caller ask when an area/title moved between duchies over a
date range.

Parent requirement IDs: REQ-004, REQ-009, REQ-010, REQ-011.

Affected files/modules:

- `src/lib.rs`
- transfer fixtures/tests
- CLI/demo output if useful

Exit criteria:

- transfer query returns ordered `from`, `to`, date/span, and event cause.
- tests cover at least two transfers and no-transfer range.
- contested or split transfer cases are deferred explicitly if not implemented.

Status: proposed.

### WP-005: Answer Trace And Status Taxonomy

Objective: make answers explainable and distinguish negative/uncertain cases.

Parent requirement IDs: REQ-006, REQ-007.

Exit criteria:

- answer type includes status and trace fields.
- seed fixtures are labeled as seed/non-authoritative.
- negative tests cover empty, unknown, unsupported, and not-existing states, or
  explicitly defer unsupported states.
- Source Custody Reviewer signs off before any source-backed historical data is
  added.

Status: proposed.

### WP-006: Source-Custody VTRACE Package For Real Historical Data

Objective: define how DUCHY can import real European title data safely.

Parent requirement IDs: REQ-006, REQ-SRC-001.

Exit criteria:

- source inventory exists,
- rights posture is recorded,
- citation/confidence schema is accepted,
- no restricted or commercial-game source data is copied,
- metadata-only rows are clearly distinguished from extracted data.

Status: deferred.

## Orphan Check

- [x] Every accepted `REQ-*` is assigned to a work package or dispositioned.
- [x] Every interface-changing work package names `IF-*` IDs in `TRACE.md` or
  `INTERFACES.md`.
- [x] Every work package has exit criteria and verification commands or a
  deferred disposition.
- [x] Required review lanes are named in `.roles/`.
- [x] No work package is only cleanup without parent IDs.
