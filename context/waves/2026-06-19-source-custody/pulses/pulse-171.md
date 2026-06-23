# Pulse 171: Kingdom of Italy Replacement Parentage Packet

## Intent

Promote the reviewed Kingdom of Italy / Holy Roman Empire replacement
candidates into accepted fixtures without creating simultaneous active parents.
The packet refines two direct Holy Roman Empire parentage edges by routing them
through the accepted Kingdom of Italy title.

## Changes

- Add `supersedes_fact_id` to source-backed fact records.
- Validate replacement facts against the fact they supersede:
  - the superseded fact must exist;
  - the replacement cannot supersede itself;
  - contested facts cannot replace or be replaced through this path;
  - subject, claim kind, and span must match.
- Materialize active title and parentage facts after removing superseded facts.
- Route importer status, coverage, graph, rank-skip, gap, and change reports
  through active parentage facts.
- Add two reviewed Wikimedia text sources for March of Turin and March of
  Tuscany parentage context.
- Add two accepted replacement parentage facts:
  - March of Turin -> Kingdom of Italy for `964..1091`, superseding direct HRE
    parentage.
  - March of Tuscany -> Kingdom of Italy for `962..1197`, superseding direct
    HRE parentage.

## Boundary

This packet changes de jure hierarchy only. It does not import territorial
geometry, rulers, family inheritance, legal status details, Imperial Italy
relation modeling, or Prince-Bishopric of Trent parentage under the Kingdom of
Italy.

## Current State

- reviewed sources: 440
- reviewed facts: 1323
- materialized titles: 349
- active parentage facts: 274
- superseded parentage facts: 2
- parentage titles: 229
- titles without parentage: 120
- rank-skip facts: 220
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
