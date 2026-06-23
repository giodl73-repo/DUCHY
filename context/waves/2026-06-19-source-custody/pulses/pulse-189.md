# Pulse 189: Saxe-Meiningen Source Custody

## Intent

Add reviewed text support for existing Saxe-Meiningen parentage facts.

## Changes

- Add `src-wikipedia-saxe-meiningen`.
- Attach it to Saxe-Meiningen -> Holy Roman Empire for `1675..1806`.
- Attach it to Saxe-Meiningen -> German Confederation for `1815..1866`.
- Attach it to Saxe-Meiningen -> German Empire for `1871..1918`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Saxe-Gotha partition detail, Saxe-Hildburghausen acquisition,
Ernestine genealogy, rulers, flag history, or Thuringia successor claims.

## Current State

- reviewed sources: 463
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
