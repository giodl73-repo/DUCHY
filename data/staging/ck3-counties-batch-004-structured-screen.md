# DUCHY CK3 Batch 004 Structured Claim Screen

source_research: `data/staging/ck3-counties-batch-004-wikidata-research.tsv`
screen_tsv: `data/staging/ck3-counties-batch-004-structured-screen.tsv`
rows_screened: 50

## Summary

- bounded_candidate: 3
- defer: 47

## Bounded Candidates

### Kingdom of Brycheiniog

- CK3 candidate: `c_brecknockshire`
- Wikidata item: `Q954585`
- instance of: historical country; kingdom
- inception: `450`
- dissolution: `1045`
- disposition: promote as a DUCHY `Kingdom` title.

### Byzantium

- CK3 candidate: `c_byzantion`
- Wikidata item: `Q23725`
- instance of: ancient Greek city
- inception: `-667`
- dissolution: `330`
- disposition: source-resolution only. DUCHY rank semantics for ancient cities
  are not materialized as title facts.

### Cetatea-Albă County

- CK3 candidate: `c_cetatea_alba`
- Wikidata item: `Q8273263`
- instance of: county of the Kingdom of Romania
- inception: `1925`
- dissolution: `1944`
- disposition: promote as a DUCHY `County` title.

## Boundary

This screen records structured claim availability only. It does not resolve
parentage, holder, control, or territorial geometry.
