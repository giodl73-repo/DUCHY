# Pulse 247: German Structured Bridge Relations 02

Date: 2026-06-23

## Scope

Promote the remaining reviewed Duchy of Brunswick bridge relations authorized by
accepted source custody.

## Results

- Added `german-structured-bridge-relation-02` with three relation facts:
  - Duchy of Brunswick as a `confederation_member` relation to the German
    Confederation for 1815..1866.
  - Duchy of Brunswick as a `confederation_member` relation to the North
    German Confederation for 1867..1870.
  - Duchy of Brunswick as a `federal_state_member` relation to the German
    Empire for 1871..1918.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1522.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 159.
- Rank-skip rows: 231.
- Relation-explained rows: 154.
- Unexplained rank-skip rows: 77.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-structured-bridge-relation-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-structured-bridge-relation-02.sources data/staging/german-structured-bridge-relation-02.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-structured-bridge-relation-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-structured-bridge-relation-02.sources data/staging/german-structured-bridge-relation-02.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
