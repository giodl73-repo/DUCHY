# Pulse 166: Rank Skip Bridge Cluster Packet Stubs

## Intent

Add an import-planning gate that emits packet stubs only after a cluster review
row has been explicitly marked ready.

## Changes

- Add `duchy-import parentage-rank-skip-bridge-cluster-packet-stubs`.
- Generate `data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md`.
- Require both `review_status=reviewed` and
  `review_disposition=ready_for_packet` before a row appears as a packet stub.
- Report ready and blocked row counts.
- Document the packet-stub command in `README.md`.
- Add VTRACE evidence for the packet planning gate.

## Boundary

Packet stubs are not accepted facts. They only describe possible import packet
work after source-custody review has marked a row ready. The current review
queue emits zero stubs because every row remains `pending_review` and
`not_inferred`.

## Current State

- review rows: 20
- ready_for_packet rows: 0
- blocked rows: 20
- accepted source/fact/title counts: unchanged

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-bridge-cluster-packet-stubs data/staging/parentage-rank-skip-bridge-cluster-review.tsv data/staging/parentage-rank-skip-bridge-cluster-packet-stubs.md`
