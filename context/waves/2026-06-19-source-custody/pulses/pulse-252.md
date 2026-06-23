# Pulse 252: Endpoint Transition Relations 02

Date: 2026-06-23

## Scope

Promote a small second endpoint/subdivision relation packet only where existing
accepted source custody already supports the relation wording.

## Results

- Added `endpoint-transition-relation-batch-02` with three relation facts:
  - Kingdom of Serbia to Serbian Empire as a 1346 `rank_transition`.
  - County of La Marche to the Kingdom of France as a 1527 `rank_transition`.
  - Hordaland to Kingdom of Norway as a 1919..2019
    `subdivision_or_appanage` relation.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1549.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 186.
- Rank-skip rows: 231.
- Relation-explained rows: 181.
- Unexplained rank-skip rows: 50.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/endpoint-transition-relation-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/endpoint-transition-relation-batch-02.sources data/staging/endpoint-transition-relation-batch-02.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/endpoint-transition-relation-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/endpoint-transition-relation-batch-02.sources data/staging/endpoint-transition-relation-batch-02.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
