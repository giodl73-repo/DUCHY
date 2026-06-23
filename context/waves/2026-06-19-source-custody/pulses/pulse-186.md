# Pulse 186: Duchy of Anhalt Source Custody

## Intent

Add reviewed text support for three existing active Duchy of Anhalt parentage
facts.

## Changes

- Add `src-wikipedia-duchy-anhalt`.
- Attach it to Duchy of Anhalt -> German Confederation for `1863..1866`.
- Attach it to Duchy of Anhalt -> North German Confederation for `1867..1870`.
- Attach it to Duchy of Anhalt -> German Empire for `1871..1918`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Ascanian genealogy, medieval partitions, constituent enclaves,
ruler lists, or post-1918 Free State claims.

## Current State

- reviewed sources: 457
- reviewed facts: 1327
- active parentage facts: 278
- superseded parentage facts: 2
- rank-skip rows: 220
- bridge rows: 160
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
