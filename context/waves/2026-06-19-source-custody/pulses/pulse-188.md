# Pulse 188: Mecklenburg Hesse Source Custody

## Intent

Add reviewed text support for existing Mecklenburg-Schwerin,
Mecklenburg-Strelitz, and Hesse parentage facts.

## Changes

- Add `src-wikipedia-grand-duchy-mecklenburg-schwerin`.
- Attach it to Mecklenburg-Schwerin -> German Confederation for `1815..1866`.
- Attach it to Mecklenburg-Schwerin -> North German Confederation for
  `1867..1870`.
- Attach it to Mecklenburg-Schwerin -> German Empire for `1871..1918`.
- Add `src-wikipedia-grand-duchy-mecklenburg-strelitz`.
- Attach it to Mecklenburg-Strelitz -> German Confederation for `1815..1866`.
- Attach it to Mecklenburg-Strelitz -> North German Confederation for
  `1867..1870`.
- Add `src-wikipedia-grand-duchy-hesse`.
- Attach it to Grand Duchy of Hesse -> German Confederation for `1815..1866`.
- Attach it to Grand Duchy of Hesse -> German Empire for `1871..1918`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import dynastic history, territorial geography, constitutional
episodes, military integration, ruler lists, or successor-state claims.

## Current State

- reviewed sources: 462
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
