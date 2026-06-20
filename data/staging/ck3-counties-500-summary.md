# DUCHY CK3 Europe County Discovery Queue

source_file: `C:\Users\giodl\Downloads\List of counties - CK3 Wiki.mhtml`
source_page: `https://ck3.paradoxwikis.com/List_of_counties`
parsed_county_rows: 3476
europe_bucket_rows: 1069
selected_candidates: 500
status: closed

## Boundary

This queue uses the CK3 county list as a search driver only. Rows are not
accepted historical facts, and the CK3 wiki is not an accepted historical
authority for DUCHY promotion.

Each row must resolve to an independent reviewed historical source before any
name, rank, parentage, holder, control, or territory claim can enter accepted
fixtures.

## Europe Filter

Rows were selected when at least one CK3 empire column matched one of these
Europe-facing buckets:

- Baltic Empire
- Britannia
- Byzantine Empire
- Carpathia
- Francia
- Germania
- Hispania
- Holy Roman Empire
- Italia
- Russia
- Scandinavia
- West-Slavia

## Generated Artifacts

- `data/staging/ck3-counties-500.tsv`
- `data/staging/ck3-counties-500.manifest`
- `data/staging/ck3-counties-500-report.md`
- `data/staging/ck3-counties-500-duplicate-urls.md`
- `data/staging/ck3-counties-500-rejected.md`
- `data/staging/ck3-counties-500-closed.manifest`
- `data/staging/ck3-counties-500-shards/INDEX.md`

## Validation Snapshot

- candidates: 500
- pending: 0
- reviewed: 0
- promoted: 0
- rejected: 500
- duplicate source URLs: 0

## Closure

All 10 shards have Wikidata top-lead research and structured-claim screens.
The CK3 rows themselves are archived as `scope_deferred` discovery records,
because CK3 is a search driver rather than an accepted historical source.

Reviewed independent sources and facts promoted from those screens live in the
accepted source/fact fixtures and their packet-specific staging reports.
