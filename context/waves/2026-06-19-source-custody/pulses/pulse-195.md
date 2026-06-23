# Pulse 195: Hesse Lippe Holstein Source Custody

## Intent

Add reviewed text support for existing Electorate of Hesse, Principality of
Lippe, and Duchy of Holstein parentage facts.

## Changes

- Add `src-wikipedia-electorate-hesse`.
- Attach it to Electorate of Hesse -> German Confederation for `1815..1866`.
- Add `src-wikipedia-principality-lippe`.
- Attach it to Principality of Lippe -> Holy Roman Empire for `1789..1806`.
- Attach it to Principality of Lippe -> German Confederation for `1815..1866`.
- Add `src-wikipedia-duchy-holstein`.
- Attach it to Duchy of Holstein -> Holy Roman Empire for `1474..1806`.
- Attach it to Duchy of Holstein -> German Confederation for `1815..1866`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Confederation of the Rhine, 1848-1849 German Empire episode,
Westphalian annexation, dynastic branches, North German Confederation, German
Empire, Danish personal union, Schleswig relation, treaty history, or 1866
annexation claims.

## Current State

- reviewed sources: 477
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
