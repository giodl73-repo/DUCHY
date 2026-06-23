# Pulse 227: Austria Crownland Relations

Date: 2026-06-23

## Scope

Add a typed relation kind for crownland/internal-monarchy component rows and
promote the two already source-backed Archduchy of Austria rows under Austrian
Empire and Austria-Hungary. Keep parentage unchanged.

## Results

- Added `crownland_component` as a source-backed relation kind.
- Added `relation-model-austria-crownland-01` with two relation facts.
- Promoted bounded crownland-component relations for:
  - Archduchy of Austria -> Austrian Empire, 1804..1866.
  - Archduchy of Austria -> Austria-Hungary, 1867..1918.
- Regenerated rank-skip, relation, and graph reports.
- Updated WP-007 status and remaining-blocker audit.

## Measurements

- Sources: 532.
- Facts: 1370.
- Parentage facts: 278.
- Relation facts: 39.
- Rank-skip rows: 223.
- Relation-explained rows: 39.
- Unexplained rank-skip rows: 184.
- Parentage edges: 284.
- Weighted span coverage: 59.44%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-model-austria-crownland-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-austria-crownland-01.sources data/staging/relation-model-austria-crownland-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-model-austria-crownland-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-model-austria-crownland-01.sources data/staging/relation-model-austria-crownland-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
