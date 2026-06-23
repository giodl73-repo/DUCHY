# Pulse 237: HRE Relation Batch 08

Date: 2026-06-23

## Scope

Promote another bounded HRE relation batch from accepted text-backed
current-parent rows. Keep ambiguous imperial-creation-only rows out of this
packet.

## Results

- Added `hre-relation-batch-08` with nine relation facts:
  - Bremen-Verden and Mantua as `vassalage_or_suzerainty` relations to the Holy
    Roman Empire.
  - Wurttemberg, Westphalia, Osnabruck, Paderborn, Strasbourg, Utrecht, and
    Verdun as `imperial_state` relations to the Holy Roman Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1442.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 79.
- Rank-skip rows: 231.
- Relation-explained rows: 75.
- Unexplained rank-skip rows: 156.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-relation-batch-08-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-08.sources data/staging/hre-relation-batch-08.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-relation-batch-08-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-relation-batch-08.sources data/staging/hre-relation-batch-08.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
