# Pulse 219 - Fourth Accepted Relation Packet

Date: 2026-06-23

## Scope

- Promote a small set of remaining reviewed current-parent relation blockers.
- Add the first real subdivision/appanage relation facts.

## Results

- Promoted 3 non-parentage relation facts using already reviewed source records.
- Added subdivision/appanage context for County of Empuries under the Crown of
  Aragon and County of Girona under the Carolingian Empire.
- Added HRE imperial-state context for Burgraviate of Nuremberg.
- Regenerated relation and graph reports.

## Measurements

- Sources: 520.
- Facts: 1358.
- Parentage facts: 278.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.
- Relation facts: 28.
- Rank-skip rows: 223.
- Relation-explained rows: 28.
- Unexplained rank-skip rows: 195.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-batch-04-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-04.sources data/staging/relation-model-batch-04.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-batch-04-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-04.sources data/staging/relation-model-batch-04.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
```

## Notes

- This packet does not supersede or modify parentage facts.
- Structured-data-only relation candidates remain held until text custody is
  available.
