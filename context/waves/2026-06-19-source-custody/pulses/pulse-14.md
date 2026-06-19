# Pulse 14: Reviewed Prussia Import Packet

## Intent

Scale the fixture-canonical import path with another reviewed historical title
packet.

## Changes

- Add reviewed Wikidata Q27306 source metadata.
- Add Kingdom of Prussia name, rank, and existence facts.
- Add Q27306 -> Q43287 parentage for 1871-1918.

## Review Boundary

This pulse authorizes one direct parentage path and minimal title facts only. It
does not import Prussian provincial structure, holders, borders, legal prose, or
a complete German Empire hierarchy.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
