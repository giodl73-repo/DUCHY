# Pulse 216 - First Accepted Relation Packet

Date: 2026-06-23

## Scope

- Start replaying reviewed relation-model blockers as accepted relation facts.
- Keep parentage counts stable while adding explanatory relation context.

## Results

- Promoted 6 non-parentage relation facts using already reviewed source records.
- Added relation-only rows for Anhalt confederation/federal membership and HRE
  imperial-state context for Brunswick-Luneburg, Austria, and Bohemia.
- Updated the promotion report to list candidate relation facts.
- Regenerated the parentage rank-skip relation report.

## Measurements

- Sources: 520.
- Facts: 1336.
- Parentage facts: 278.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.
- Relation facts: 6.
- Rank-skip rows: 223.
- Relation-explained rows: 6.
- Unexplained rank-skip rows: 217.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-01.sources data/staging/relation-model-batch-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-batch-01.sources data/staging/relation-model-batch-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
```

## Notes

- This packet does not supersede or modify parentage facts.
- The next relation packets should cover the remaining accepted HRE/confederation
  blockers before moving to split-control and subdivision/appanage cases.
