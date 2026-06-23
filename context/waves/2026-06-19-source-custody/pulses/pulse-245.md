# Pulse 245: German Structured Bridge Relations

Date: 2026-06-23

## Scope

Promote structured bridge relation contexts only where accepted source reviews
explicitly authorized bridge relations to the German Confederation, North
German Confederation, or German Empire.

## Results

- Added `german-structured-bridge-relation-01` with nine relation facts:
  - Prussia and Saxony as `federal_state_member` relations to the German
    Empire.
  - Bavaria, Wurttemberg, and Baden as `confederation_member` relations to the
    German Confederation and `federal_state_member` relations to the German
    Empire.
  - Hanover as a `confederation_member` relation to the German Confederation.
- Excluded Prussia and Saxony German/North German Confederation rows because
  their subject source review only authorizes the German Empire country
  relation.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1516.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 153.
- Rank-skip rows: 231.
- Relation-explained rows: 148.
- Unexplained rank-skip rows: 83.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-structured-bridge-relation-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-structured-bridge-relation-01.sources data/staging/german-structured-bridge-relation-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-structured-bridge-relation-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-structured-bridge-relation-01.sources data/staging/german-structured-bridge-relation-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
