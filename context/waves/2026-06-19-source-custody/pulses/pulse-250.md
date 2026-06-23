# Pulse 250: Vassal Client Relations 02

Date: 2026-06-23

## Scope

Promote bounded relation contexts for four remaining rank-skip rows with
accepted source notes that identify protectorate, client, personal-union, oath,
or vassal semantics against the current parent.

## Results

- Added `vassal-client-relation-batch-02` with four
  `vassalage_or_suzerainty` relation facts:
  - Albanian Kingdom under the Italian Empire for 1939.
  - Italian protectorate of Albania under the Italian Empire for 1939..1943.
  - Kingdom of Livonia under the Tsardom of Russia for 1570..1578.
  - Kingdom of Thessalonica under the Latin Empire for 1204..1224.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1542.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 179.
- Rank-skip rows: 231.
- Relation-explained rows: 174.
- Unexplained rank-skip rows: 57.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/vassal-client-relation-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/vassal-client-relation-batch-02.sources data/staging/vassal-client-relation-batch-02.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/vassal-client-relation-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/vassal-client-relation-batch-02.sources data/staging/vassal-client-relation-batch-02.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
