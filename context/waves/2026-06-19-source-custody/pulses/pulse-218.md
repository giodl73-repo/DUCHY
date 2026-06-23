# Pulse 218 - Third Accepted Relation Packet

Date: 2026-06-23

## Scope

- Continue promoting reviewed relation-model blockers that point to the current
  parent title and can be classified by the rank-skip relation report.
- Introduce the first real vassalage/suzerainty relation fact.

## Results

- Promoted 7 non-parentage relation facts using already reviewed source records.
- Added HRE imperial-state context for Brunswick-Wolfenbuttel, Bavaria, Berg,
  Burgundy, Geneva, and Trent.
- Added Byzantine vassalage/suzerainty context for Duchy of Amalfi.
- Regenerated relation and graph reports.

## Measurements

- Sources: 520.
- Facts: 1355.
- Parentage facts: 278.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.
- Relation facts: 25.
- Rank-skip rows: 223.
- Relation-explained rows: 25.
- Unexplained rank-skip rows: 198.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-batch-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-03.sources data/staging/relation-model-batch-03.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-batch-03-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-03.sources data/staging/relation-model-batch-03.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
```

## Notes

- This packet does not supersede or modify parentage facts.
- Girona/Carolingian and split-control rows remain held because their relation
  semantics need narrower source review before promotion.
