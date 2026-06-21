# County Scale 009 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-009.tsv`.
No fixtures, source code, existing queue files, or other shard result files were
edited. CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 1 |
| `rank_policy_blocked` | 5 |
| `source_resolution_blocked` | 39 |
| `reject_bad_lead` | 5 |

## Priority Rows

### 409 Isle de France

Decision: `reject_bad_lead`.

The bounded top lead `Q12060804` is the former French colonial name for
Mauritius and surrounding colonies, dated 1715..1810. That is a wrong-place
entity for the CK3 France seed and should not be promoted as a title packet.
The row needs a fresh source-resolution pass for the French province/region if
that is desired.

Source:

- https://www.wikidata.org/wiki/Q12060804

### 407 Ipuskoa

Decision: `source_resolution_blocked`.

The CK3 seed resolves plausibly to Gipuzkoa/Guipuzcoa. Britannica supports the
name as a geographical entity from the late 10th century and says Guipuzcoa was
unified with Castile as a county in 1200. This is useful source-resolution
evidence, but it is not yet a bounded DUCHY title packet and no parentage span
is ready.

Source:

- https://www.britannica.com/place/Guipuzcoa

### 413 Jåhkåmåhkke

Decision: `source_resolution_blocked`.

The CK3 seed likely resolves to Jokkmokk. The available independent lead found
in this pass supports a Sami meeting place and 17th-century town/market history,
not a materialized title rank or bounded parentage packet.

Source:

- https://thehiddennorth.com/destination/jokkmokk/

### 418 Jovvkuj

Decision: `source_resolution_blocked`.

No independent historical title candidate was resolved in this pass. Treat this
as an explicit source follow-up row; search should continue from Sami/local
historical names rather than from CK3 parent columns.

### 422 Kaisereia

Decision: `rank_policy_blocked`.

The better source lead is Caesarea Mazaca/Kayseri. Encyclopedia.com / Columbia
Encyclopedia identifies it as an ancient city, formerly Mazaca, renamed
Caesarea around 10 BC by Archelaus of Cappadocia. This supports identity and
city semantics, but city rank is not currently materialized for county
parentage.

Source:

- https://www.encyclopedia.com/reference/encyclopedias-almanacs-transcripts-and-maps/caesarea-mazaca

## Other Notable Rows

### 412 Jaen

Decision: `already_accepted`.

The CK3 seed resolves to the locally accepted `title-q1617495` / Kingdom of
Jaen. Fixtures already contain title facts for `1246..1833` and parentage to
`title-q217196` / Crown of Castile for `1246..1715`, so this is a metrics seed
only and should not be re-imported.

Source:

- https://www.wikidata.org/wiki/Q1617495

### 406 Ionia and 408 Isauria

Decision: `rank_policy_blocked`.

Independent sources resolve these as ancient regions/districts, not materialized
county titles. Ionia is supported by Britannica as an ancient region of western
Anatolia. The current Isauria Wikidata top lead is a bad genus entity, but the
1911 Britannica article resolves the historical name to an ancient district of
Asia Minor. Both remain blocked on region-rank policy.

Sources:

- https://www.britannica.com/place/Ionia
- https://en.wikisource.org/wiki/1911_Encyclop%C3%A6dia_Britannica/Isauria

### 416 Jersika and 440 Khachen

Decision: `rank_policy_blocked`.

Jersika resolves to a medieval Latgalian principality, and Khachen resolves to a
melikdom/feudal entity in Karabakh. These may be historically meaningful, but
principality/melikdom semantics are outside the current county parentage import
policy.

Sources:

- https://www.historyfiles.co.uk/KingListsEurope/EasternLatvia.htm
- https://karabakh.org/karabakh-history/karabakh-during-the-xviii-cc/karabakh-meliks/

### 424 Kakheti, 433 Kartli, and 436 Kent

Decision: `source_resolution_blocked`.

Each has a better locally accepted title candidate, but no parentage packet is
ready from this shard review:

- `title-q1209822` / Kingdom of Kakheti, `1465..1762`.
- `title-q195972` / Kingdom of Kartli, `1466..1762`.
- `title-q328818` / Kingdom of Kent, `450..871`.

These should proceed through parentage source review, not through CK3-derived
parent columns.

Sources:

- https://www.wikidata.org/wiki/Q1209822
- https://www.wikidata.org/wiki/Q195972
- https://www.wikidata.org/wiki/Q328818
- https://www.britannica.com/place/Kent-historical-kingdom-England

## General Blockers

Most rows remain source-resolution blockers because the current lead is a modern
municipality, city, settlement, natural feature, region, or administrative area
without bounded title and parentage custody. Rows with plainly wrong-type leads
were marked `reject_bad_lead`, including the Mauritius `Isle de France` lead,
Juterbog railway station, Keve given name, Kintus genus, and Kitka family name.
