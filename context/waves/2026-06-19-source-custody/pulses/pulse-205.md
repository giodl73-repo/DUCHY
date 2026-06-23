# Pulse 205 - Materialized Parentage Edge Reports

Date: 2026-06-23

## Scope

- Align graph, change, rank-skip, candidate, and bridge reports with the
  materialized parentage timeline produced by partial replacement facts.
- Split residual spans from superseded parentage facts before report metrics are
  counted, so a bounded replacement does not erase the still-active earlier and
  later spans from report analysis.

## Results

- Reviewed sources: 520.
- Facts: 1330.
- Parentage edges: 284.
- Titles with parentage: 229.
- Weighted span coverage: 59.44%.
- Rank-skip edge rows: 223.
- Rank-skip rows with candidates: 218.
- Rank-skip bridge rows: 163.
- Rank-skip bridge clusters: 20.
- Preserved reviewed bridge-cluster decisions: 20.
- Ready-for-packet bridge clusters: 0.
- Temporal parent conflicts: 0.
- Snapshot cycle years: 0.

## Validation

```powershell
cargo test --quiet
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-report data/staging/parentage-rank-skip-targets.tsv data/staging/parentage-rank-skip-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-candidates fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-candidates.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridges-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-bridges.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-report data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-clusters-tsv data/staging/parentage-rank-skip-bridges.tsv data/staging/parentage-rank-skip-bridge-clusters.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-report data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-tsv data/staging/parentage-rank-skip-bridge-clusters.tsv data/staging/parentage-rank-skip-bridge-cluster-review.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-report data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-packet-stubs data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-review-shard data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-review-shards 5
```

## Notes

- Raw accepted fact rows and materialized graph edges are now reported as
  distinct surfaces. The Burgundian replacements reduce accepted active
  fact-row skips, while the materialized report correctly counts residual split
  HRE spans as separate graph edges.
- Bridge-cluster review regeneration preserves existing human review decisions
  while refreshing counts and child/fact lists from materialized edges.
- This makes depth and connectivity metrics more faithful to the historical
  timeline DUCHY will query at scale.
