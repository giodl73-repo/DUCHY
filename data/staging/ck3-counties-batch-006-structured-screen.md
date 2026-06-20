# DUCHY CK3 Batch 006 Structured Claim Screen

source_research: `data/staging/ck3-counties-batch-006-wikidata-research.tsv`
screen_tsv: `data/staging/ck3-counties-batch-006-structured-screen.tsv`
rows_screened: 45

## Summary

- bounded_candidate: 2
- defer: 43

## Bounded Candidates

### Donji Kraji

- CK3 candidate: `c_donjikraji`
- Wikidata item: `Q5296194`
- instance of: historical region / zemlja
- inception: `1230`
- dissolution: `1463`
- disposition: source-resolution only. Title facts remain deferred because
  region/zemlja rank semantics are not materialized in DUCHY.

### Duklja

- CK3 candidate: `c_duklja`
- Wikidata item: `Q1252942`
- instance of: historical country; state
- inception: `854`
- dissolution: `1252`
- disposition: already accepted from the earlier 500-source candidate queue as
  `src-wikidata-q1252942` with title facts, so batch 006 does not duplicate it.

## Boundary

This screen records structured claim availability only. It does not resolve
parentage, holder, control, or territorial geometry.
