# Pulse 201: Five Hundred Source Milestone

## Intent

Reach 500 reviewed source records by adding reviewed text support for existing
Holy Roman Empire parentage facts.

## Changes

- Add `src-wikipedia-county-flanders`.
- Attach it to County of Flanders -> Holy Roman Empire for `962..1797`.
- Add `src-wikipedia-duchy-berg-hre`.
- Attach it to Duchy of Berg -> Holy Roman Empire for `1380..1806`.
- Add `src-wikipedia-duchy-guelders`.
- Attach it to Duchy of Guelders -> Holy Roman Empire for `1096..1795`.
- Add `src-wikipedia-kingdom-bohemia`.
- Attach it to Kingdom of Bohemia -> Holy Roman Empire for `1198..1803`.
- Add `src-wikipedia-landgraviate-hesse`.
- Attach it to Landgraviate of Hesse -> Holy Roman Empire for `1264..1567`.
- Add `src-wikipedia-landgraviate-hesse-darmstadt`.
- Attach it to Landgraviate of Hesse-Darmstadt -> Holy Roman Empire for
  `1567..1806`.
- Add `src-wikipedia-landgraviate-hesse-kassel`.
- Attach it to Landgraviate of Hesse-Kassel -> Holy Roman Empire for
  `1567..1803`.
- Add `src-wikipedia-hesse-marburg`.
- Attach it to Hesse-Marburg -> Holy Roman Empire for `1458..1604`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import French-fief split modeling, Low Countries successor regimes,
Julich/Cleves/Mark unions, Bohemian Crown lands, imperial-elector semantics,
Habsburg monarchy, Austrian Empire, Austria-Hungary, Hesse partitions beyond
the already accepted parentage rows, Hessian War claims, or any post-title
successor-state claims.

Several source pages carry citation or quality warnings. Those records are
therefore accepted only as bounded text support for already reviewed parentage
facts, not as authority for new graph edges.

## Current State

- reviewed sources: 500
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
