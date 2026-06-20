# DUCHY CK3 Batch 001 Wikidata Research Leads

source_manifest: `data/staging/ck3-counties-500-shards/batch-001.manifest`
research_tsv: `data/staging/ck3-counties-batch-001-wikidata-research.tsv`
rows: 50

## Boundary

These rows are external search leads only. A matching Wikidata label does not prove that the CK3 county, historical title, modern place, and source-backed DUCHY title are the same entity.

Promotion still requires source-custody review, accepted structured claims, and explicit title facts.

## Search Summary

- exact_label: 32
- fuzzy_label: 16
- no_result: 2
- error: 0

## Review Use

- Prefer exact historical-title or historical-region entities over modern municipalities when available.
- Mark ambiguous CK3 names for manual review instead of promoting them from the top search hit.
- Use independent source records for facts; the CK3 row and search result are not enough.
