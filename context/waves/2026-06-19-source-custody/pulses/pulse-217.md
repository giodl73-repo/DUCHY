# Pulse 217 - Second Accepted Relation Packet

Date: 2026-06-23

## Scope

- Continue replaying reviewed HRE and confederation rank-skip blockers as
  accepted non-parentage relation facts.
- Avoid source-custody tightening rows, split-control rows, child-level-source
  blockers, and rank-identity blockers.

## Results

- Promoted 12 non-parentage relation facts using already reviewed source
  records.
- Covered remaining accepted Anhalt HRE/confederation rows plus Archduchy of
  Austria, Bavaria-Munich, Montbeliard, Nassau, Savoy, and Wurttemberg HRE
  imperial-state rows.
- Regenerated relation and graph reports.

## Measurements

- Sources: 520.
- Facts: 1348.
- Parentage facts: 278.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.
- Relation facts: 18.
- Rank-skip rows: 223.
- Relation-explained rows: 18.
- Unexplained rank-skip rows: 205.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-02.sources data/staging/relation-model-batch-02.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-02.sources data/staging/relation-model-batch-02.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
```

## Notes

- This packet does not supersede or modify parentage facts.
- Remaining relation work should move next into subdivision/appanage,
  vassalage/suzerainty, rank-transition, and split-control packets only where
  the accepted source wording supports the relation type.
