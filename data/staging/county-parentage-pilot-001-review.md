# County Parentage Pilot 001 Review

## Intent

Work through 10 CK3 county search-driver rows manually before scaling county
parentage work with parallel agents.

## Selection

The pilot uses the first 10 bounded candidates from the CK3 Europe county
structured screens, excluding the false `Isle de France` Mauritius lead. CK3
rows remain search drivers only; accepted data must come from independent
reviewed historical sources.

## Findings

- 4 rows already have accepted parentage:
  - `c_abauj` -> Abaúj county -> Kingdom of Hungary.
  - `c_cetatea_alba` -> Cetatea-Albă County -> Kingdom of Romania.
  - `c_duklja` -> Duklja -> Byzantine Empire.
  - `c_hordalandi` -> Hordaland -> Kingdom of Norway.
- 1 row has accepted title facts but blocked parentage:
  - `c_brecknockshire` -> Kingdom of Brycheiniog has a plausible Deheubarth
    subordination/successor context, but current parentage validation requires
    a parent rank above the child rank, so kingdom -> kingdom subordination
    needs an explicit relation policy before import.
- 5 rows are source-resolution records only:
  - Ailech is a historical-country lead, not a county under current ranks.
  - Arbanon is a principality lead and needs principality rank semantics.
  - Béarn is a former-province lead and needs province/former-province
    semantics.
  - Byzantium is an ancient-city lead and should not be promoted as a county.
  - Donji Kraji is a region/zemlja lead and needs regional-rank semantics.

## Scaling Rule

Agents should not treat CK3 `county` as a DUCHY `County`. The safe workflow is:

1. Resolve the CK3 seed to an independent historical source.
2. Classify the resolved entity rank from the source, not from CK3.
3. Promote title facts only if rank and bounded existence are materialized.
4. Promote parentage only if the parent relation passes current rank policy, or
   record the row as a relation-policy blocker.
5. Use accepted parentage rows as metric seeds for change-count reports.

## Open Modeling Needs

- Equal-rank subordination or successor relation for cases like Brycheiniog and
  Deheubarth.
- Principality, province/former-province, region/zemlja, and city rank
  decisions.
- A mass county parentage metrics report that counts parent changes by child
  title and parent rank.
