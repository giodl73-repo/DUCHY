# Pulse 254: German Polish Client Relations 01

Date: 2026-06-23

## Scope

Add source-backed relation contexts for German Confederation, North German
Confederation, and World War I Polish client-state rank skips.

## Results

- Added four reviewed Wikimedia text source records:
  - German Confederation.
  - North German Confederation.
  - Kingdom of Saxony.
  - Kingdom of Poland 1917-1918.
- Added five relation facts:
  - Kingdom of Prussia and Kingdom of Saxony as German Confederation
    `confederation_member` contexts.
  - Kingdom of Prussia and Kingdom of Saxony as North German Confederation
    `federal_state_member` contexts.
  - Kingdom of Poland as a German Empire `vassalage_or_suzerainty` context.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 550.
- Facts: 1560.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 197.
- Rank-skip rows: 231.
- Relation-explained rows: 192.
- Unexplained rank-skip rows: 39.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-polish-client-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-polish-client-relation-01.sources data/staging/german-polish-client-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-polish-client-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-polish-client-relation-01.sources data/staging/german-polish-client-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
