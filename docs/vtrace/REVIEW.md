# DUCHY VTRACE Role Review

## Scope

Role review of the DUCHY governance and VTRACE baseline before implementing the
lineage-query work packages.

## Findings

| Role | Finding | Disposition |
|---|---|---|
| Timeline Steward | A single static parent field is not enough to prove areas moved between duchies over time. | Added REQ-009 and WP-002 for temporal parentage spans/events. |
| Territorial Lineage Reviewer | The stated user question is about areas moving between duchies, not only titles changing labels. | Added REQ-010 and WP-002 for area identity separate from title identity. |
| Query Interface Reviewer | Transfer behavior needs fixture coverage before API behavior is accepted. | Added REQ-011 and made WP-002 establish no-transfer, single-transfer, and multi-transfer fixtures. |
| Source Custody Reviewer | Real European title data remains blocked until source policy, confidence, and rights posture are reviewed. | Kept source import deferred to WP-006. |
| Game Systems Reviewer | The current package stays focused on lineage questions and avoids mechanics or clone behavior. | No additional game-mechanics work package accepted. |

## Work Package Completeness Decision

The original package was not complete enough to start transfer-query
implementation safely. It missed a prerequisite model/fixture package for
temporal parentage and stable area identity.

After this review, the needed pre-source work packages are:

1. WP-001 governance and VTRACE baseline.
2. WP-002 temporal area/title model and transfer fixture baseline.
3. WP-003 year title-path and lineage query.
4. WP-004 duchy-transfer range query.
5. WP-005 answer trace and status taxonomy.
6. WP-006 deferred source-custody package for real historical data.

This is sufficient to begin foundation implementation without importing real
historical data. It is not sufficient to claim real European historical coverage.

## Open Risks

| Risk | Owner | Disposition |
|---|---|---|
| Area identity may later need geometry or source-boundary IDs. | Territorial Lineage Reviewer | Defer map/GIS work until REQ-MAP-001 trigger. |
| Contested or split control may require richer relation types than simple parent spans. | Timeline Steward / Query Interface Reviewer | WP-005 explicitly reserves contested status and defers contested/split semantics until fixtures require them. |
| Source-backed answers may be mistaken for authoritative history. | Source Custody Reviewer | WP-006 remains required before real historical imports. |
