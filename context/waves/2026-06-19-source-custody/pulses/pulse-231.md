# Pulse 231: Principality of Catalonia Title

Date: 2026-06-23

## Scope

Materialize the Principality of Catalonia title identity from the accepted
source-custody record. Keep this pass title-only because the accepted County of
Barcelona span ends before the Principality title span starts in the current
fixture.

## Results

- Added `principality-catalonia-title-01` with one Wikidata source record.
- Promoted Principality of Catalonia as a Duchy-rank title for 1173..1714.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage and relation facts unchanged.

## Measurements

- Sources: 540.
- Facts: 1413.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 50.
- Rank-skip rows: 231.
- Relation-explained rows: 47.
- Unexplained rank-skip rows: 184.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/principality-catalonia-title-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/principality-catalonia-title-01.sources data/staging/principality-catalonia-title-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/principality-catalonia-title-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/principality-catalonia-title-01.sources data/staging/principality-catalonia-title-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
