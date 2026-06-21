# County Scale 010 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-010.tsv`.
No fixtures, source code, existing queue files, or other shard result files were
edited. CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 2 |
| `ready_title_packet` | 2 |
| `rank_policy_blocked` | 8 |
| `source_resolution_blocked` | 26 |
| `reject_bad_lead` | 12 |

## Ready Title Candidates

### 456 Komárom

Decision: `ready_title_packet`.

The better lead is `Komárom County`, not the modern town lead. The reviewed
source identifies it as an administrative county of the Kingdom of Hungary,
with an 11th-century origin and later discontinuities around 1786, 1790, 1923,
1938, and 1945. This is title-fact useful, but parentage still needs direct
relation/source review before any promotion.

Source:

- https://en.wikipedia.org/wiki/Kom%C3%A1rom_County

### 474 La Marche

Decision: `ready_title_packet`.

The better lead is `County of La Marche`. The reviewed source supports a
medieval French county beginning with the La Marche counts in the 10th century.
Britannica separately supports Marche as the pre-Revolution French province.
This is a title candidate, not a parentage packet.

Sources:

- https://en.wikipedia.org/wiki/County_of_La_Marche
- https://www.britannica.com/place/Marche-historical-province-France

## Accepted Metrics Seeds

### 490 Leon

Decision: `already_accepted`.

The CK3 row's top Wikidata lead is a male given name, but local fixtures already
contain `title-q175276` / Kingdom of Leon, rank `Kingdom`, exists `910..1833`,
with parentage to `title-q217196` / Crown of Castile for `1230..1715`. Treat as
a metrics seed only.

Source:

- https://www.wikidata.org/wiki/Q175276

### 494 Liege

Decision: `already_accepted`.

The better local title is `title-q158835` / Prince-Bishopric of Liege, rank
`Duchy`, exists `985..1795`, with parentage to `title-q12548` / Holy Roman
Empire for the same span. Treat as a metrics seed only.

Source:

- https://www.wikidata.org/wiki/Q158835

## Rank Policy Blockers

### 453 Kokenois

Decision: `rank_policy_blocked`.

Independent review resolves the CK3 seed to Koknese/Kukenois, a small medieval
principality on the Daugava. This is historically meaningful, but principality
rank is not currently materialized for county parentage.

Source:

- https://www.historyfiles.co.uk/KingListsEurope/EasternKoknese.htm

### 458 Korinthos and 476 Laconia

Decision: `rank_policy_blocked`.

Korinthos resolves to ancient Corinth, a city/polis lead. Laconia resolves to a
historic region/regional unit in the Peloponnese. City and region semantics are
not currently importable as county parentage facts.

Sources:

- https://www.ascsa.edu.gr/excavations/ancient-corinth/about-the-excavations-1/history-timeline
- https://www.britannica.com/place/Laconia-department-Greece

### 470 Kujawy, 475 Labourd, and 496 Lika

Decision: `rank_policy_blocked`.

These resolve to historical region/province leads rather than materialized
county titles. They should wait for a region/province policy decision or a
different bounded title source.

Sources:

- https://www.britannica.com/place/Kujawsko-Pomorskie
- https://www.wikidata.org/wiki/Q671023
- https://www.wikidata.org/wiki/Q142894

### 484 Legnica

Decision: `rank_policy_blocked`.

Britannica supports Legnica as a 12th-century Silesian stronghold that became
capital of an autonomous principality in 1248. Principality rank remains outside
the current county parentage import policy.

Source:

- https://www.britannica.com/place/Legnica

## Source-Resolution Follow-Up

Several rows have plausible better historical leads but no bounded title and
parentage packet ready:

- `c_cammin` / Kołobrzeg: ecclesiastical Kołobrzeg/Cammin material exists, but
  this pass did not produce a safe title span and parentage packet.
- `c_lancashire`, `c_leicestershire`, `c_lincolnshire`, `c_linlithgowshire`,
  and `c_lanarkshire`: historic county leads need bounded title-date custody
  and parentage evidence.
- `c_leczycka`: Łęczyca has duchy/voivodeship context, but the row needs a
  stronger bounded title source before promotion.
- `c_leinster`: Britannica supports province and early kingdom context, but the
  row needs kingdom-versus-province resolution and dates.

Sources:

- https://www.catholic-hierarchy.org/diocese/dk566.html
- https://www.britannica.com/summary/Lancashire-county-England
- https://www.britannica.com/place/Parts-of-Lindsey
- https://en.wikipedia.org/wiki/%C5%81%C4%99czyca
- https://www.britannica.com/place/Leinster

## Bad Leads

Rows marked `reject_bad_lead` had top leads that were plainly wrong entity
types or wrong places: `Kolon` heart organ, `Krajina` street, `Krajna`
Slovenian settlement, `Kursk` submarine, `Leibach` family name, `Leibnitz`
philosopher, `Lennox` family name, Breton `Léon` given name, `Leuchtenburg`
wrong-place Seini lead, and `Limousin` family name. Some may have real
historical candidates, but they require fresh source resolution rather than
promotion from the current lead.

## General Blockers

Most remaining rows are modern municipalities, cities, towns, settlements,
islands, natural features, or ambiguous administrative areas. Those were left as
explicit `source_resolution_blocked` rows with follow-up actions instead of
inventing title or parentage facts from CK3 hierarchy columns.
