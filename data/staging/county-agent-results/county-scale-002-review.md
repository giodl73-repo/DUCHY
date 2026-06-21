# County Scale 002 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-002.tsv`.
No fixtures, source code, queue files, or other shard result files were edited.
CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `ready_title_packet` | 1 |
| `rank_policy_blocked` | 2 |
| `source_resolution_blocked` | 41 |
| `reject_bad_lead` | 6 |

## Priority Rows

### 52 Arbanon

Decision: `rank_policy_blocked`.

The local source-resolution record `src-wikidata-q2454379` already identifies
the lead as `title-q2454379` / Principality of Arbanon with structured
`1190..1255` claims, but the pilot review explicitly left principality semantics
unmodeled. Independent follow-up supports the same blocker: History Files
describes Arbanon/Arbër as a principality formed in 1190 within modern Albania.
The row should stay blocked until DUCHY has a principality rank policy, or an
explicit mapping from principality to an existing rank.

Sources:

- https://www.historyfiles.co.uk/KingListsEurope/EasternAlbaniaArbanon.htm
- https://www.wikidata.org/wiki/Q2454379

### 96 Béarn

Decision: `rank_policy_blocked`.

The local source-resolution record `src-wikidata-q213763` treats Béarn as a
former-province lead and defers title facts. Britannica gives a better
historical-title reading: Béarn became a viscounty feudally dependent on the
dukes of Aquitaine in 819, and its viscounts ceased acknowledging a suzerain in
the 11th century. That is useful evidence, but it moves the blocker from a
generic province to viscounty/former-province rank semantics. No current DUCHY
rank cleanly materializes the title.

Sources:

- https://www.britannica.com/place/Bearn
- https://www.wikidata.org/wiki/Q213763

### 89 Barcelona

Decision: `ready_title_packet`.

The city lead should not be imported, but a historical title lead is ready for
maintainer review: `title-q1233672` / County of Barcelona. Britannica has a
dedicated historical-county page, and Wikidata supplies a dissolved date of
1164. The recommended packet is title-only, rank `County`, existence
`801..1164`; parentage still needs separate review.

Sources:

- https://www.britannica.com/place/Barcelona-historical-county-Spain
- https://www.wikidata.org/wiki/Q1233672

## Source-Resolution Needed Rows

### 67 Austr Agðir

Decision: `source_resolution_blocked`.

The best available lead in this pass points toward Agder/Egdafylki and later
Nedenes/Aust-Agder administrative history. The evidence is not yet a safe
historical title packet because the CK3 seed uses older regional naming while
the surfaced bounded material is later county/amt terminology.

Source:

- https://en.wikipedia.org/wiki/Aust-Agder

### 68 Austrland

Decision: `source_resolution_blocked`.

The better lead is Icelandic regional-quarter material:
`Austfirðingafjórðungur` / Austurland. That is useful for identity resolution,
but it is regional/quarter geography, not a materialized DUCHY title rank.

Source:

- https://is.wikipedia.org/wiki/Landsfj%C3%B3r%C3%B0ungur

### 71 Avalois

Decision: `source_resolution_blocked`.

No reliable independent title source was found in this pass. The row remains a
source-resolution follow-up rather than a bad lead, because it may be a
Burgundian regional identity, but no title/rank/date packet is ready.

### 79 Azysia

Decision: `source_resolution_blocked`.

No reliable independent title source was found in this pass. The CK3 seed should
remain blocked until a historical name variant or source lead is identified.

### 84 Bagrewand

Decision: `source_resolution_blocked`.

The better lead is Bagrevand/Bagrewand, an Armenian historical region. That
supports identity resolution but not a county title under current ranks. It
should stay blocked until regional-rank handling is defined or a different
title source is found.

Source:

- https://en.wikipedia.org/wiki/Bagrevand

## Other Notable Rows

- `c_arad`: better historical county lead found as Arad County of the Kingdom of
  Hungary (`title-q253609`), but dates are too imprecise for a packet.
- `c_armagnac`: Britannica supports Armagnac as a historic region, but not the
  bounded County of Armagnac packet needed here.
- `c_bacs`: better lead is Bács-Bodrog County (`title-q852564`), but identity
  with CK3 Bacs and bounded dates require follow-up.
- `c_baranya`: better lead is historical Baranya County (`title-q15891578`),
  but bounded dates and parentage are not packet-ready.
- `c_baden`, `c_bars`, `c_beaumont`: current top leads are wrong-place matches.
- `c_aviovara`, `c_astorga`, `c_bekes`: current top leads are wrong-type
  matches.

## General Blockers

Most rows remain blocked because the current lead is a modern city, commune,
municipality, region, shire, historical division, or family/legal item rather
than a sourced historical title with a modeled rank and bounded existence. Rows
were marked `reject_bad_lead` only when the top lead was plainly wrong-type or
wrong-place. Rows were marked `source_resolution_blocked` when a plausible
historical title may exist but still needs independent bounded title/date
sourcing.
