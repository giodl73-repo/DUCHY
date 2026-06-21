# County Scale 003 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-003.tsv`.
No fixtures, source code, existing queue files, or other shard result files were
edited. CK3 county/duchy/kingdom columns were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `ready_title_packet` | 4 |
| `rank_policy_blocked` | 4 |
| `source_resolution_blocked` | 32 |
| `reject_bad_lead` | 10 |

## Priority Rows

### 104 Benevento

Decision: `ready_title_packet`.

The city lead should not be imported, but a better title lead is available:
Duchy of Benevento. History Files places the independent Lombard duchy at
Benevento in 570-571, and the title maps cleanly to materialized `Duchy` rank.
Recommended title span is `571..774`; parentage remains follow-up.

Source:

- https://www.historyfiles.co.uk/KingListsEurope/ItalyBenevento.htm

### 138 Brabant

Decision: `ready_title_packet`.

The current top lead is a family-name item, but local fixtures already contain
accepted title facts for `title-q159856` / Duchy of Brabant with rank `Duchy`
and span `1183..1795`. This is title-ready only; no accepted parentage is tied
to this shard row.

Source:

- https://www.wikidata.org/wiki/Q159856

### 141 Brandenburg

Decision: `ready_title_packet`.

The current top lead is Brandenburg an der Havel, but local fixtures already
contain accepted title facts for `title-q148499` / Margraviate of Brandenburg
with rank `Duchy` and span `1157..1806`. Parentage is not recommended from this
county row.

Source:

- https://www.wikidata.org/wiki/Q148499

### 145 Breifne

Decision: `ready_title_packet`.

The top lead is a journal title, but local fixtures already contain accepted
title facts for `title-q905131` / Kingdom of Breifne with span `700..1256`.
This remains a title-only follow-up because parentage needs independent
relation evidence.

Source:

- https://www.wikidata.org/wiki/Q905131

## Rank Blockers

- `c_bithynia`: better lead is ancient Bithynia as a region/kingdom context,
  but not a materialized county-like medieval title for this workflow.
- `c_blekinge`: province rank is not materialized.
- `c_boeotia`: historical region rank is not materialized.
- `c_breisgau`: regional geography needs rank policy or a different county
  source.

## Other Notable Rows

- `c_blois`: Britannica supports Blois as a medieval feudal countship, but
  bounded existence and parentage were not packet-ready.
- `c_bouillon`: Duchy of Bouillon is a plausible better lead, but source custody
  and parentage need review.
- `c_brunswick` and `c_bremen`: related accepted duchy facts exist locally, but
  both CK3 rows resolve first to cities and need scope review before reuse.
- `c_bezichi`: no useful independent lead was resolved; it remains a low
  confidence source-resolution follow-up.

## Rejected Leads

Rows marked `reject_bad_lead` have plainly wrong top leads: wrong-place matches
(`c_belz`, `c_bihar`, `c_brene`), fictional or taxonomic items
(`c_bereg`, `c_beroe`, `c_biton`), or family-name items with no packet-ready
replacement in this pass (`c_bergh`, `c_bethen`, `c_bothin`, `c_boleslav`).

## General Blockers

Most remaining rows are blocked because the current lead is a modern city,
commune, municipality, island, region, or administrative district. Plausible
historical titles were recorded where found, but no parentage claim was
promoted without independent relation evidence.
