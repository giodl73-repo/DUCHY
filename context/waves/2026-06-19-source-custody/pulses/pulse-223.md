# Pulse 223: Austria Internal Monarchy Source Custody

Date: 2026-06-23

## Scope

Tighten text custody for the two reviewed Archduchy of Austria rank-skip rows
under Austrian Empire and Austria-Hungary. Keep this as a source-custody pass:
no parentage replacement, no new relation facts, and no crownland/Cisleithania
modeling yet.

## Results

- Added `austria-internal-monarchy-sources-01` with three reviewed Wikimedia
  text source records:
  - `src-wikipedia-archduchy-austria-crownland`,
  - `src-wikipedia-austrian-empire`,
  - `src-wikipedia-austria-hungary`.
- Attached those sources to:
  - `fact-q699964-parent-q131964`,
  - `fact-q699964-parent-q28513`.
- Regenerated graph and relation reports.
- Updated WP-007 and the remaining blocker audit.

## Measurements

- Sources: 529.
- Facts: 1361.
- Parentage facts: 278.
- Relation facts: 31.
- Rank-skip rows: 223.
- Relation-explained rows: 31.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/austria-internal-monarchy-sources-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/austria-internal-monarchy-sources-01.sources data/staging/austria-internal-monarchy-sources-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/austria-internal-monarchy-sources-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/austria-internal-monarchy-sources-01.sources data/staging/austria-internal-monarchy-sources-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
