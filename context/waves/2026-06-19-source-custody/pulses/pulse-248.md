# Pulse 248: Vassal And Satellite Relations 01

Date: 2026-06-23

## Scope

Promote bounded relation contexts for remaining rank-skip rows whose accepted
text sources explicitly identify satellite, client, vassal, or suzerainty
semantics against the current parent.

## Results

- Added `vassal-satellite-relation-batch-01` with nine relation facts:
  - Duchy of Warsaw under the First French Empire for 1807..1815.
  - Kingdom of Imereti under the Russian Empire for 1804..1810.
  - Grand Principality of Moscow and Grand Principality of Vladimir under the
    Golden Horde.
  - Eastern Hungarian Kingdom, Moravian Serbia, Prince-Bishopric of
    Montenegro, Principality of Serbia, and the United Principalities under
    the Ottoman Empire.
- Used the existing `vassalage_or_suzerainty` relation kind for all rows. The
  schema does not currently define a separate client-state relation kind.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1531.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 168.
- Rank-skip rows: 231.
- Relation-explained rows: 163.
- Unexplained rank-skip rows: 68.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/vassal-satellite-relation-batch-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/vassal-satellite-relation-batch-01.sources data/staging/vassal-satellite-relation-batch-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/vassal-satellite-relation-batch-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/vassal-satellite-relation-batch-01.sources data/staging/vassal-satellite-relation-batch-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
