# Pulse 12: First Real Parentage Source Import

## Intent

Import the first real source-backed parentage span after source-custody review.

## Changes

- Add reviewed Wikidata Q43287 source metadata.
- Add German Empire name, rank, and existence facts.
- Add Q20135 -> Q43287 parentage for 1871-1918.
- Distinguish strict seed parentage from direct source-backed parentage so
  reviewed facts can represent documented rank skips.
- Extend the source-backed title-path query smoke to show Grand Duchy of Hesse
  -> German Empire in 1871.

## Review Boundary

This pulse authorizes one direct parentage path only. It does not import
intermediate constituent-state semantics, holder genealogy, borders, map
geometry, or a complete German Empire hierarchy.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `git diff --check`
