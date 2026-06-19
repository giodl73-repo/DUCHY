# DUCHY Verification Plan

## Scope

Verification for lineage and territorial-transfer query capability.

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | test / inspection | `cargo test --quiet` | Stable title identity tests pass. | partial pass | Current seed tests. |
| REQ-002 | test / review | `cargo test --quiet` plus role review | Parentage/control distinction remains visible. | partial pass | Current seed tests; WP-002 pending. |
| REQ-003 | test / demonstration | `cargo test --quiet`; `cargo run --quiet` | County/duchy/kingdom path returned for target year. | pass | Title-path tests and CLI demo on 2026-06-19. |
| REQ-004 | test / demonstration | Future transfer fixture tests | Range query returns ordered duchy transfers. | pending | WP-004. |
| REQ-005 | test / inspection | `cargo test --quiet` | Continuity events are ordered and queryable. | pass | `TitlePathAnswer.events` test on 2026-06-19. |
| REQ-009 | test / inspection | `cargo test --quiet` | Parentage changes are represented over time. | pass | `ParentageSpan` tests on 2026-06-19. |
| REQ-010 | test / review | `cargo test --quiet` | Area identity survives title/parent/holder changes. | pass | `AreaTitleSpan` test on 2026-06-19. |
| REQ-011 | test / inspection | `cargo test --quiet` | No-transfer, single-transfer, and multi-transfer cases exist. | pass | Transfer fixture test on 2026-06-19. |
| REQ-006 | review / inspection | Source Custody review | Historical answers carry source class and confidence. | deferred | Source package. |
| REQ-007 | test | Negative and contested fixture tests | Answer statuses distinguish empty/unknown/unsupported/contested. | pending | WP-005. |
| REQ-008 | review | `.roles` and VTRACE inspection | No mechanics or clone behavior in core lineage work. | pass foundation | README, PRODUCT_PLAN, roles. |

## Commands

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast local sanity for active query/model work. | `cargo fmt --check`; `cargo test --quiet` | current pass |
| L1 | Full child-repo confidence before commit or push. | `cargo fmt --check`; `cargo test --quiet`; `cargo run --quiet`; role-file inspection | current pass for foundation |
| L2 | Readiness proof before claiming lineage-transfer capability. | L1 plus transfer fixtures, answer-trace examples, and role review ledger | pending |

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | test | `cargo test --quiet` | Foundation model tests. | pass on 2026-06-19 |
| EVID-002 | demo | `cargo run --quiet` | Seed snapshot output. | pass on 2026-06-19 |
| EVID-003 | review artifact | `.roles/` | Governance baseline. | added |
| EVID-004 | VTRACE artifact | `docs/vtrace/` | Requirements and trace baseline. | added |
| EVID-005 | test | `cargo test --quiet` | Temporal parentage, area identity, and transfer fixture baseline. | pass on 2026-06-19 |
| EVID-006 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Year title-path and lineage answer query. | pass on 2026-06-19 |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| No transfer range query yet. | Cannot directly answer movement between duchies. | WP-004. |
| No answer trace/status object yet. | Hard to explain uncertain or negative answers. | WP-005. |
| No historical source custody package yet. | Real European data import remains blocked. | Defer to source VTRACE package. |
