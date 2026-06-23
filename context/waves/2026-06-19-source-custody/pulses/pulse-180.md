# Pulse 180: Brandenburg Pomerania HRE Source Custody

## Intent

Improve custody for two active HRE parentage facts that still relied on
structured source IDs only. This keeps the hierarchy unchanged while making
the accepted HRE claims easier to audit.

## Changes

- Add `src-wikipedia-duchy-pomerania`.
- Attach it to Duchy of Pomerania -> Holy Roman Empire for `1121..1637`.
- Add `src-wikipedia-margraviate-brandenburg`.
- Attach it to Margraviate of Brandenburg -> Holy Roman Empire for
  `1157..1806`.

## Boundary

This pulse adds source support only. It does not add parentage facts, supersede
facts, change spans, or import Pomerania's segmented vassalage periods,
Brandenburg's Bohemian Crown period, electoral dignity, Brandenburg-Prussia
union, or post-HRE successor claims.

## Current State

- reviewed sources: 445
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
