# Pulse 190: Saxe-Altenburg Schaumburg-Lippe Source Custody

## Intent

Add reviewed text support for existing Saxe-Altenburg and Schaumburg-Lippe
parentage facts.

## Changes

- Add `src-wikipedia-saxe-altenburg`.
- Attach it to Saxe-Altenburg -> Holy Roman Empire for `1602..1806`.
- Attach it to Saxe-Altenburg -> German Confederation for `1815..1866`.
- Add `src-wikipedia-schaumburg-lippe`.
- Attach it to Schaumburg-Lippe -> Holy Roman Empire for `1643..1806`.
- Attach it to Schaumburg-Lippe -> German Confederation for `1815..1866`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import partitions, personal unions, inheritances, princely
elevations, ruler lists, flag history, territorial geography, or successor
state claims.

## Current State

- reviewed sources: 465
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
