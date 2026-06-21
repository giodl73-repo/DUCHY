# County Parentage Scale Queue

Generated from `ck3-counties-500.tsv`, CK3 Wikidata research TSVs, CK3 structured-screen TSVs, and accepted `fixtures/first-real.facts`.

CK3 rows are search drivers only. Agents must use independent reviewed historical sources before proposing title or parentage facts.

## Status Counts

- accepted_parentage: 4
- rank_semantics_review: 6
- source_resolution_deferred: 462
- source_resolution_needed: 27
- title_needs_parentage_review: 1

## Shards

- county-scale-001: 50 rows; accepted_parentage=1, rank_semantics_review=1, source_resolution_deferred=46, source_resolution_needed=2
- county-scale-002: 50 rows; rank_semantics_review=2, source_resolution_deferred=43, source_resolution_needed=5
- county-scale-003: 50 rows; source_resolution_deferred=49, source_resolution_needed=1
- county-scale-004: 50 rows; accepted_parentage=1, rank_semantics_review=1, source_resolution_deferred=47, title_needs_parentage_review=1
- county-scale-005: 50 rows; source_resolution_deferred=47, source_resolution_needed=3
- county-scale-006: 50 rows; accepted_parentage=1, rank_semantics_review=1, source_resolution_deferred=43, source_resolution_needed=5
- county-scale-007: 50 rows; source_resolution_deferred=46, source_resolution_needed=4
- county-scale-008: 50 rows; accepted_parentage=1, source_resolution_deferred=47, source_resolution_needed=2
- county-scale-009: 50 rows; rank_semantics_review=1, source_resolution_deferred=45, source_resolution_needed=4
- county-scale-010: 50 rows; source_resolution_deferred=49, source_resolution_needed=1

## Next Agent Priority

1. `title_needs_parentage_review`: accepted historical title exists; find source-backed parentage.
2. `rank_semantics_review`: source lead has dates but rank/entity policy blocks promotion.
3. `source_resolution_deferred`: replace weak top lead with better independent historical source.
4. `source_resolution_needed`: no useful Wikidata lead yet.
5. `accepted_parentage`: metrics seed; do not re-import.
