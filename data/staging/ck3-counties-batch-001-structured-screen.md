# DUCHY CK3 Batch 001 Structured Claim Screen

source_research: `data/staging/ck3-counties-batch-001-wikidata-research.tsv`
screen_tsv: `data/staging/ck3-counties-batch-001-structured-screen.tsv`
rows_screened: 48

## Summary

- bounded_candidate: 2
- defer: 46

## Bounded Candidates

### Abaúj county

- CK3 candidate: `c_abauj`
- Wikidata item: `Q1049854`
- instance of: `Q852231`
- inception: `1201`
- dissolution: `1881`
- disposition: promoted in `ck3-batch-001-title-facts`.

### Ailech

- CK3 candidate: `c_ailech`
- Wikidata item: `Q15104874`
- instance of: `Q3024240` historical country
- inception: `600`
- dissolution: `780`
- disposition: source-resolution only. Title facts remain deferred because the
  structured entity is a historical country while the CK3 discovery row is a
  county lead, so DUCHY rank semantics need manual review.

## Boundary

This screen records structured claim availability only. It does not resolve
identity, rank, parentage, holder, control, or territorial geometry.
