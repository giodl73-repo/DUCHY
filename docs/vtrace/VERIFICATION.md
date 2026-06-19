# DUCHY Verification Plan

## Scope

Verification for lineage and territorial-transfer query capability.

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | test / inspection | `cargo test --quiet` | Stable title identity tests pass. | partial pass | Current seed tests. |
| REQ-002 | test / review | `cargo test --quiet` plus role review | Parentage/control distinction remains visible. | pass | Current seed tests and temporal parentage model. |
| REQ-003 | test / demonstration | `cargo test --quiet`; `cargo run --quiet` | County/duchy/kingdom path returned for target year. | pass | Title-path tests and CLI demo on 2026-06-19. |
| REQ-004 | test / demonstration | `cargo test --quiet`; `cargo run --quiet` | Range query returns ordered duchy transfers. | pass | Transfer query tests and CLI demo on 2026-06-19. |
| REQ-005 | test / inspection | `cargo test --quiet` | Continuity events are ordered and queryable. | pass | `TitlePathAnswer.events` test on 2026-06-19. |
| REQ-009 | test / inspection | `cargo test --quiet` | Parentage changes are represented over time. | pass | `ParentageSpan` tests on 2026-06-19. |
| REQ-010 | test / review | `cargo test --quiet` | Area identity survives title/parent/holder changes. | pass | `AreaTitleSpan` test on 2026-06-19. |
| REQ-011 | test / inspection | `cargo test --quiet` | No-transfer, single-transfer, and multi-transfer cases exist. | pass | Transfer fixture test on 2026-06-19. |
| REQ-006 | review / inspection | `cargo test --quiet`; Source Custody review | Seed answers carry source class; real historical imports require source records and confidence labels. | pass for policy | `SourceClass::Seed` tests and `docs/vtrace/source-custody/`. |
| REQ-007 | test | `cargo test --quiet` | Answer statuses distinguish answered/empty/unknown/unsupported; contested remains reserved. | pass | Query envelope tests on 2026-06-19. |
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
| L2 | Readiness proof before claiming seed lineage-transfer capability. | L1 plus transfer fixtures, answer-trace examples, and role review ledger | pass for seed fixtures |

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | test | `cargo test --quiet` | Foundation model tests. | pass on 2026-06-19 |
| EVID-002 | demo | `cargo run --quiet` | Seed snapshot output. | pass on 2026-06-19 |
| EVID-003 | review artifact | `.roles/` | Governance baseline. | added |
| EVID-004 | VTRACE artifact | `docs/vtrace/` | Requirements and trace baseline. | added |
| EVID-005 | test | `cargo test --quiet` | Temporal parentage, area identity, and transfer fixture baseline. | pass on 2026-06-19 |
| EVID-006 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Year title-path and lineage answer query. | pass on 2026-06-19 |
| EVID-007 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Duchy-transfer range query. | pass on 2026-06-19 |
| EVID-008 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Answer status, seed source class, and trace codes. | pass on 2026-06-19 |
| EVID-009 | review artifact | `docs/vtrace/source-custody/` | Source inventory, import policy, confidence model, schema sketch, and review gate. | pass for policy on 2026-06-19 |
| EVID-010 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Metadata-only source catalog and review validation. | pass on 2026-06-19 |
| EVID-011 | test/demo | `cargo test --quiet`; `cargo run --quiet`; `fixtures/source-policy.sources` | Metadata source file parser and fixture validation. | pass on 2026-06-19 |
| EVID-012 | test/demo | `cargo test --quiet`; `cargo run --quiet` | Source-backed fact gate validation. | pass on 2026-06-19 |
| EVID-013 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `context/waves/2026-06-19-source-custody/pulses/pulse-04.md` | First reviewed real source-backed facts for Wikidata Q158445. | pass on 2026-06-19 |
| EVID-014 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `context/waves/2026-06-19-source-custody/pulses/pulse-05.md` | First source-backed title materializes from reviewed name/rank/existence facts. | pass on 2026-06-19 |
| EVID-015 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `context/waves/2026-06-19-source-custody/pulses/pulse-06.md` | First source-backed title-path query returns a traced answer envelope. | pass on 2026-06-19 |
| EVID-016 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `fixtures/first-real.facts`; `context/waves/2026-06-19-source-custody/pulses/pulse-07.md` | First real source-backed facts parse from fixture file and drive the query smoke. | pass on 2026-06-19 |
| EVID-017 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `context/waves/2026-06-19-source-custody/pulses/pulse-08.md` | Contested facts group for review and are blocked from normal materialization. | pass on 2026-06-19 |
| EVID-018 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `fixtures/first-real.facts`; `context/waves/2026-06-19-source-custody/pulses/pulse-09.md` | Second reviewed Wikidata title imports as source-backed facts and materializes. | pass on 2026-06-19 |
| EVID-019 | test/review | `cargo test --quiet`; `context/waves/2026-06-19-source-custody/pulses/pulse-10.md` | Reviewed parentage facts can materialize only with accepted sources, spans, and materialized titles. | pass on 2026-06-19 |
| EVID-020 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `fixtures/first-real.sources`; `context/waves/2026-06-19-source-custody/pulses/pulse-11.md` | Reviewed real source metadata parses from fixture and validates the real fact fixture. | pass on 2026-06-19 |
| EVID-021 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `fixtures/first-real.facts`; `fixtures/first-real.sources`; `context/waves/2026-06-19-source-custody/pulses/pulse-12.md` | First real source-backed parentage paths import Q20135 -> Q43287 and Q158445 -> Q43287 for 1871-1918. | pass on 2026-06-19 |
| EVID-022 | test/demo/review | `cargo test --quiet`; `cargo run --quiet`; `cargo fmt --check`; `git diff --check`; `context/waves/2026-06-19-source-custody/pulses/pulse-13.md` | Reviewed historical import helpers are fixture-canonical, with no source-backed historical data literals in Rust. | pass on 2026-06-19 |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Contested and split transfer query semantics are not modeled yet. | Transfer answers cover clear ordered parent changes only. | Fact-level contested packets exist; query-level contested answers remain deferred. |
| Only three concrete source records have passed the gate. | Historical import is real but intentionally tiny. | Expand through reviewed source packets only. |
| No real contested historical fixture has passed review yet. | The contested packet is proven with synthetic alternatives only. | Add real contested claims only through reviewed source packets. |
| Only two real parentage fixtures have passed review. | Source-backed title paths remain sparse and incomplete. | Expand parentage only through reviewed source packets. |
