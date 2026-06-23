# Pulse 187: Oldenburg Brunswick Source Custody

## Intent

Add reviewed text support for existing Grand Duchy of Oldenburg and Duchy of
Brunswick parentage facts.

## Changes

- Add `src-wikipedia-grand-duchy-oldenburg`.
- Attach it to Grand Duchy of Oldenburg -> German Confederation for
  `1816..1866`.
- Attach it to Grand Duchy of Oldenburg -> North German Confederation for
  `1867..1870`.
- Attach it to Grand Duchy of Oldenburg -> German Empire for `1871..1918`.
- Add `src-wikipedia-duchy-brunswick`.
- Attach it to Duchy of Brunswick -> German Confederation for `1815..1866`.
- Attach it to Duchy of Brunswick -> North German Confederation for
  `1867..1870`.
- Attach it to Duchy of Brunswick -> German Empire for `1871..1918`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import dynastic histories, territorial components, succession
disputes, foreign relations, ruler lists, constitutional history, territorial
map claims, or Free State successor claims.

## Current State

- reviewed sources: 459
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
