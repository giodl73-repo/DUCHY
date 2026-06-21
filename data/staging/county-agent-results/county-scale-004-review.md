# County Scale 004 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-004.tsv`.
No fixtures, source code, or queue files were edited. CK3 rows were treated only
as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 1 |
| `rank_policy_blocked` | 3 |
| `source_resolution_blocked` | 31 |
| `reject_bad_lead` | 15 |

## Priority Rows

### 157 Brycheiniog

Decision: `rank_policy_blocked`.

The accepted local title is `title-q954585` / Kingdom of Brycheiniog, sourced in
fixtures by `src-wikidata-q954585` for label, rank, and `450..1045` existence.
Independent review evidence supports a Deheubarth relationship but not a
currently importable parentage fact: Encyclopedia.com / Oxford Companion says
Brycheiniog fell under Deheubarth influence c.940, and History Files describes
it as effectively a sub-kingdom under Deheubarth pressure c.920.

Recommended blocker: `title-q954585` -> `title-q837136` Kingdom of Deheubarth,
candidate span `c.940..1045`, blocked because current validation expects parent
rank above child rank and this is kingdom -> kingdom.

Sources:

- https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/brycheiniog
- https://www.historyfiles.co.uk/KingListsBritain/CymruBrycheiniog.htm

### 167 Byzantion

Decision: `rank_policy_blocked`.

The local reviewed source-resolution record `src-wikidata-q23725` already treats
Byzantium as source-resolution only. Independent evidence supports that reading:
History.com identifies Byzantium/Byzantion as an ancient Greek settlement
founded in 657 BC and refounded as Constantinople in AD 330. Wikidata also
classifies Q23725 as an ancient city/polis. That is not a materialized DUCHY
county rank.

Recommended action: keep blocked until ancient-city rank semantics are modeled
or explicitly deferred.

Sources:

- https://www.history.com/articles/constantinople
- https://www.wikidata.org/wiki/Q23725

### 196 Cetatea Alba

Decision: `already_accepted`.

This row is a metrics seed only. The accepted local title is `title-q8273263` /
Cetatea-Alba County, rank `County`, exists `1925..1944`, with accepted parentage
to `title-q203493` / Kingdom of Romania for `1925..1944`. Wikidata Q8273263
supports the structured label, county-of-Kingdom-of-Romania description,
inception, country, and dissolved-date claims already used by fixtures.

Source:

- https://www.wikidata.org/wiki/Q8273263

### 161 Buckinghamshire

Decision: `source_resolution_blocked`.

VCH Buckinghamshire is a good source lead and confirms county-history coverage,
including Anglo-Saxon archaeology and Domesday material, but this pass did not
establish a bounded `title_exists` span or parentage safe enough for promotion.

Source:

- https://www.history.ac.uk/research/victoria-county-history/counties-z/buckinghamshire

### 172 Caithness

Decision: `source_resolution_blocked`.

Britannica supports Caithness as a Scottish county area with earlier Pictish,
Norse, and medieval earldom context. It does not by itself provide a safe bounded
county title span or parentage packet for this workflow.

Source:

- https://www.britannica.com/place/Caithness

### 179 Cambridgeshire

Decision: `source_resolution_blocked`.

Britannica supports historic county context and medieval administration, while
VCH Cambridgeshire is a strong follow-up source family. This pass did not
establish bounded title dates or parentage.

Sources:

- https://www.britannica.com/place/Cambridgeshire
- https://www.history.ac.uk/research/victoria-county-history/counties-z/cambridgeshire

### 195 Ceredigion/Cardiganshire

Decision: `source_resolution_blocked`.

The modern principal-area lead should not be promoted. A better historical title
candidate exists: `title-q2578706` / Kingdom of Ceredigion. Ceredigion County
Council's history page supports the Ceredigion, Seisyllwg, and Deheubarth
context, including the AD 920 formation of Deheubarth from Dyfed and Seisyllwg
including Ceredigion. Date custody remains conflicted across leads, so no
bounded title packet or parentage recommendation is ready.

Sources:

- https://www.discoverceredigion.wales/heritage-and-culture-to-explore/the-story-of-ceredigion/
- https://www.wikidata.org/wiki/Q2578706

## General Blockers

Many rows remain explicit blockers because the current top lead is a modern
place, family name, genus, natural region, island, valley, or encyclopedia
article item rather than a sourced historical title. These were recorded as
`reject_bad_lead` where the lead is plainly wrong-type or wrong-place, and
`source_resolution_blocked` where a historical title may exist but still needs
independent bounded title/date sourcing.
