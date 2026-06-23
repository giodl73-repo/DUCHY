# Pulse 239: HRE Landgraviate And Margraviate Relations

Date: 2026-06-23

## Scope

Promote bounded landgraviate and margraviate relation contexts from accepted
current-parent rows. Keep rows without clean text-backed relation wording out of
this packet.

## Results

- Added `hre-landgraviate-margraviate-relation-01` with seven relation facts:
  - Hesse, Hesse-Darmstadt, Hesse-Kassel, and Brandenburg as `imperial_state`
    relations to the Holy Roman Empire.
  - Brabant, Austria, and Meissen as `vassalage_or_suzerainty` relations to the
    Holy Roman Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1457.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 94.
- Rank-skip rows: 231.
- Relation-explained rows: 90.
- Unexplained rank-skip rows: 141.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/hre-landgraviate-margraviate-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-landgraviate-margraviate-relation-01.sources data/staging/hre-landgraviate-margraviate-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/hre-landgraviate-margraviate-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-landgraviate-margraviate-relation-01.sources data/staging/hre-landgraviate-margraviate-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
