# Pulse 203: Mecklenburg Italy Brabant Mirandola Sulzbach Source Custody

## Intent

Add reviewed text support for the remaining simple Holy Roman Empire parentage
facts while keeping split/replacement cases held.

## Changes

- Add `src-wikipedia-duchy-mecklenburg-schwerin` and attach it to Duchy of
  Mecklenburg-Schwerin -> Holy Roman Empire for `1701..1806`.
- Add `src-wikipedia-kingdom-italy-hre` and attach it to Kingdom of Italy ->
  Holy Roman Empire for `962..1806`.
- Add `src-wikipedia-landgraviate-brabant` and attach it to Landgraviate of
  Brabant -> Holy Roman Empire for `1085..1183`.
- Add `src-wikipedia-duchy-mirandola` and attach it to Duchy of Mirandola ->
  Holy Roman Empire for `1310..1710`.
- Add `src-wikipedia-palatinate-sulzbach` and attach it to
  Palatinate-Sulzbach -> Holy Roman Empire for `1557..1806`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Confederation of the Rhine, Grand Duchy of Mecklenburg-
Schwerin, Italian/Napoleonic successor states, Duchy of Brabant successor
claims, Mirandola dynastic/title-elevation detail, or Palatine branch
succession.

The remaining HRE-only rows are intentionally not cleared here:

- Duchy of Milan remains held for Italian/Habsburg split review.
- March of Turin and March of Tuscany remain held because active Kingdom of
  Italy replacement reviews already exist.
- Duchy of Massa and Carrara remains held because the available article supports
  the title history but not the full HRE parentage span cleanly.
- Landgraviate of Lower Alsace remains held pending a better source.

Several source pages carry citation or quality warnings. Those records are
accepted only as bounded text support for already reviewed parentage facts, not
as authority for new graph edges.

## Current State

- reviewed sources: 519
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
