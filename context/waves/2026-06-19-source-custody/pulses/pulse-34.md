# Pulse 34: Scale Metadata Manifest Gate

## Intent

Prepare DUCHY for a 500-source candidate wave by making source-candidate
manifests carry the metadata needed to classify, defer, reject, and promote
large batches safely.

## Changes

- Extend candidate manifests with review batch, import scope, rank basis,
  entity class, source claims used, confidence detail, parentage status, query
  readiness, and exclusion reason fields.
- Require all reviewed and promoted rows to carry the scale metadata before
  manifest validation passes.
- Require rejected rows to carry an exclusion reason.
- Preserve the scale metadata in generated active/archive manifests, shard
  manifests, reports, duplicate URL reports, and TSV export/import.
- Update the sample staging manifest to show pending, reviewed, and rejected
  rows under the new gate.

## Review Boundary

This pulse changes candidate metadata and tooling only. It does not import new
historical facts, add accepted sources, change parentage, or infer hierarchy
from candidate metadata.

## Validation

- `cargo run --bin duchy-import -- manifest data/staging/example.manifest`
- `cargo run --bin duchy-import -- manifest-report data/staging/example.manifest data/staging/manifest-report.md`
- `cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest data/staging/manifest.tsv`
- `cargo run --bin duchy-import -- manifest-from-tsv data/staging/manifest.tsv data/staging/from-tsv.manifest`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
