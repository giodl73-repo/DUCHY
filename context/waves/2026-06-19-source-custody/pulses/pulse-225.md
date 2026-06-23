# Pulse 225: Guelders Rank Identity And Relation

Date: 2026-06-23

## Scope

Resolve the Guelders rank-identity blocker and then promote the safe
current-parent relation that was previously blocked by the rank mismatch.

## Results

- Updated `duchy-promote` so candidate facts can supersede accepted fixture
  facts by validating in merged context and reporting candidate titles from the
  merged title catalog.
- Added `guelders-rank-identity-01` to supersede `fact-q152420-rank` with
  `fact-q152420-rank-duchy`.
- Added `relation-model-guelders-01` with one `imperial_state` relation to the
  current HRE parent.
- Regenerated rank-skip, relation, and graph reports.
- Updated WP-007 status and remaining-blocker audit.

## Measurements

- Sources: 532.
- Facts: 1364.
- Parentage facts: 278.
- Relation facts: 33.
- Rank-skip rows: 223.
- Relation-explained rows: 33.
- Unexplained rank-skip rows: 190.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/guelders-rank-identity-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/guelders-rank-identity-01.sources data/staging/guelders-rank-identity-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/guelders-rank-identity-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/guelders-rank-identity-01.sources data/staging/guelders-rank-identity-01.facts
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-guelders-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-guelders-01.sources data/staging/relation-model-guelders-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-guelders-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-guelders-01.sources data/staging/relation-model-guelders-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
