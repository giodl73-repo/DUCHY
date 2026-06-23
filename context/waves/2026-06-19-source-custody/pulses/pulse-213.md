# Pulse 213 - Relation Fact Materializer Slice

Date: 2026-06-23

## Scope

- Implement the first `WP-007` code slice.
- Add typed source-backed relation facts distinct from parentage.
- Prove relation facts can be materialized and queried without changing
  parentage title paths.

## Results

- Added `RelationKind` and `RelationSpan`.
- Added `claim_kind: relation` parsing.
- Added source-backed relation materialization from values of the form
  `relation_kind:related_title_id`.
- Added timeline storage and year lookup for relation spans.
- Added validation that relation spans reference materialized titles and remain
  inside title existence spans.
- Added tests for successful materialization and typed value/span validation.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
git diff --check
```

## Notes

- Relation facts do not alter `ParentageSpan` or title-path traversal.
- Rank-skip report integration and query trace display remain next-step
  `WP-007` work.
