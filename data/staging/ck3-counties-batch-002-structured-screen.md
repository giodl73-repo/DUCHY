# DUCHY CK3 Batch 002 Structured Claim Screen

source_research: `data/staging/ck3-counties-batch-002-wikidata-research.tsv`
screen_tsv: `data/staging/ck3-counties-batch-002-structured-screen.tsv`
rows_screened: 45

## Summary

- bounded_candidate: 2
- defer: 43

## Bounded Candidates

### Principality of Arbanon

- CK3 candidate: `c_arbanon`
- Wikidata item: `Q2454379`
- instance of: `Q208500`
- inception: `1190`
- dissolution: `1255`
- disposition: source-resolution only. Title facts remain deferred because
  DUCHY materialization currently has county, duchy, kingdom, and empire ranks,
  while this source lead is explicitly a principality.

### Béarn

- CK3 candidate: `c_bearn`
- Wikidata item: `Q213763`
- instance of: `Q209495`; `Q3502482`
- inception: `1620`
- dissolution: `1790`
- disposition: source-resolution only. Title facts remain deferred because the
  top lead is a former province and needs rank/identity review before it can be
  represented as a DUCHY title.

## Boundary

This screen records structured claim availability only. It does not resolve
identity, rank, parentage, holder, control, or territorial geometry.
