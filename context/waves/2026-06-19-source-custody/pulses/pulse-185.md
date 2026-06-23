# Pulse 185: Baden Wurttemberg Electorate Source Custody

## Intent

Add reviewed text support for two existing active Holy Roman Empire electorate
parentage facts.

## Changes

- Add `src-wikipedia-electorate-baden`.
- Attach it to Electorate of Baden -> Holy Roman Empire for `1803..1806`.
- Add `src-wikipedia-electorate-wurttemberg`.
- Attach it to Electorate of Wurttemberg -> Holy Roman Empire for `1803..1806`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import predecessor duchies, ruler biographies, mediatisation details,
territorial gains, Confederation of the Rhine context, or successor kingdoms
and grand duchies.

## Current State

- reviewed sources: 456
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
