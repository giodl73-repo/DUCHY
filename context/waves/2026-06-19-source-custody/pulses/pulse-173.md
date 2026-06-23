# Pulse 173: Kingdom of Germany Candidate Packet

## Intent

Start the next import milestone after closing the active bridge-cluster
reconciliation. The next promising class is German duchy-to-HRE rank skips
where the missing intermediate layer may be the Kingdom of Germany.

## Changes

- Add `data/staging/kingdom-germany-parentage-packet-candidates.tsv`.
- Add `data/staging/kingdom-germany-parentage-packet-candidates.md`.
- Add `data/staging/kingdom-germany-parentage-source-review.sources`.
- Identify three Bavaria-family replacement candidates and one Austrian held
  candidate.

## Boundary

This pulse does not promote accepted fixtures. Realm-level Kingdom of Germany
sources are not enough for replacement facts; child-level source custody is
still required for each edge.

## Current State

- accepted sources: 440
- accepted facts: 1323
- active parentage facts: 274
- active rank-skip rows: 220
- candidate packet rows: 4
- source-backed replacement candidates ready for accepted fixture import: 0

## Validation

- `Import-Csv data/staging/kingdom-germany-parentage-packet-candidates.tsv -Delimiter "`t"` parses 4 candidate rows.
- `cargo test --quiet`
- `git diff --check`
