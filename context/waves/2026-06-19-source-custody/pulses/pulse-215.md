# Pulse 215 - Rank-Skip Relation Report

Date: 2026-06-23

## Scope

- Continue `WP-007` after the relation trace slice.
- Add a generated report that classifies rank-skip rows explained by typed
  relation facts.

## Results

- Added `parentage-rank-skip-relation-report` to `duchy-import`.
- Added a conservative classifier: a rank-skip row is explained only when the
  child title has an overlapping source-backed relation fact to the current
  parent title.
- Kept relation facts separate from canonical parentage; the report explains
  rank-skip audit rows but does not alter title paths, graph density, or cycle
  checks.
- Generated `data/staging/parentage-rank-skip-relation-report.md`.

## Measurements

- Titles: 349.
- Relation facts: 0.
- Rank-skip rows: 223.
- Relation-explained rows: 0.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
```

## Notes

- The report surface is ready; the next data milestone is promoting reviewed
  relation facts from the blocker queue.
