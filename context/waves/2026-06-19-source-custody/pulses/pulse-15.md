# Pulse 15: Reviewed Saxony Import Packet

## Intent

Continue scaling reviewed historical imports through fixture-only source and
fact rows.

## Changes

- Add reviewed Wikidata Q153015 source metadata.
- Add Kingdom of Saxony name, rank, and existence facts.
- Add Q153015 -> Q43287 parentage for 1871-1918.

## Review Boundary

This pulse authorizes one direct parentage path and minimal title facts only. It
does not import Saxon administrative structure, holders, borders, legal prose,
or a complete German Empire hierarchy.

## Validation

- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
