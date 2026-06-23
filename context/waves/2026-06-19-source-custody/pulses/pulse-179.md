# Pulse 179: HRE Parentage Source Custody Tightening

## Intent

Improve source custody for existing active HRE parentage without changing the
accepted hierarchy. Some parentage facts still cited only structured Wikidata
even after reviewed text sources existed for the same bounded claim.

## Changes

- Attach `src-wikipedia-duchy-lorraine` to Duchy of Lorraine -> Holy Roman
  Empire for `962..1766`.
- Attach `src-wikipedia-duchy-saxony` to Duchy of Saxony -> Holy Roman Empire
  for `962..1296`.
- Add `src-wikipedia-margraviate-meissen`.
- Attach `src-wikipedia-margraviate-meissen` to Margraviate of Meissen -> Holy
  Roman Empire for `965..1423`.

## Boundary

This pulse adds source support only. It does not add parentage facts, supersede
facts, change spans, or import predecessor/successor narrative details.

## Current State

- reviewed sources: 443
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
