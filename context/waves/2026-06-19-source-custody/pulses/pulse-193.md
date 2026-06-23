# Pulse 193: Ernestine Short-Span Source Custody

## Intent

Add reviewed text support for existing HRE and German Confederation parentage
facts for three short-span Ernestine duchies.

## Changes

- Add `src-wikipedia-saxe-coburg-saalfeld`.
- Attach it to Saxe-Coburg-Saalfeld -> Holy Roman Empire for `1699..1806`.
- Attach it to Saxe-Coburg-Saalfeld -> German Confederation for `1815..1825`.
- Add `src-wikipedia-saxe-gotha-altenburg`.
- Attach it to Saxe-Gotha-Altenburg -> Holy Roman Empire for `1680..1806`.
- Attach it to Saxe-Gotha-Altenburg -> German Confederation for `1815..1826`.
- Add `src-wikipedia-saxe-hildburghausen`.
- Attach it to Saxe-Hildburghausen -> Holy Roman Empire for `1680..1806`.
- Attach it to Saxe-Hildburghausen -> German Confederation for `1815..1826`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import French occupation, Confederation of the Rhine claims,
Ernestine succession arbitration, dynastic genealogy, successor allocations,
population/area statistics, constitutions, or modern Thuringia/district
claims.

## Current State

- reviewed sources: 472
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
