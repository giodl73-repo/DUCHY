# DUCHY Verification Plan

## Scope

Verification for lineage and territorial-transfer query capability.

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | test / inspection | `cargo test --quiet` | Stable title identity tests pass. | partial pass | Current seed tests. |
| REQ-002 | test / review | `cargo test --quiet` plus role review | Parentage/control distinction remains visible. | partial pass | Current seed tests; WP-002 pending. |
| REQ-003 | test / demonstration | Future `title_path_in_year` tests | County/duchy/kingdom path returned for target year. | pending | WP-002. |
| REQ-004 | test / demonstration | Future transfer fixture tests | Range query returns ordered duchy transfers. | pending | WP-003. |
| REQ-005 | test / inspection | `cargo test --quiet` | Continuity events are ordered and queryable. | partial pass | Current `events_for_title` tests are implicit only; strengthen in WP-002. |
| REQ-006 | review / inspection | Source Custody review | Historical answers carry source class and confidence. | deferred | Source package. |
| REQ-007 | test | Negative and contested fixture tests | Answer statuses distinguish empty/unknown/unsupported/contested. | pending | WP-004. |
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

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| No title-path query yet. | Cannot directly answer "which duchy in year Y". | WP-002. |
| No transfer range query yet. | Cannot directly answer movement between duchies. | WP-003. |
| No answer trace/status object yet. | Hard to explain uncertain or negative answers. | WP-004. |
| No historical source custody package yet. | Real European data import remains blocked. | Defer to source VTRACE package. |
