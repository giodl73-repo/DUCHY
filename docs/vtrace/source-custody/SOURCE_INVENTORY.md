# Source Inventory

## Scope

Candidate source classes for future real historical DUCHY fixtures.

This inventory is rights posture, not import approval. Each concrete dataset or
work still needs a source record before use.

## Candidate Classes

| Source class | Possible use | Rights posture | Initial decision |
|---|---|---|---|
| Wikidata structured data | Title/entity IDs, labels, inception/dissolution dates, parent/part-of claims, source pointers. | Wikidata states structured data in main/property/lexeme namespaces is CC0; text in other namespaces is CC BY-SA 4.0. | Candidate for metadata-first source records; do not copy article prose. |
| OpenHistoricalMap data | Historical boundary and place geometry where needed later. | OpenHistoricalMap says its data is generally CC0 unless noted, with some individual features under open licenses such as CC BY or CC BY-SA. | Candidate only after per-feature license tags and attribution/share-alike impact are reviewed. |
| Wikimedia/Wikipedia text | Human-readable context and citation discovery. | Wikimedia terms allow reuse under free/open licenses, but text commonly carries attribution/share-alike obligations. | Metadata/citation discovery only by default; do not ingest prose into DUCHY fixtures. |
| Public-domain primary or secondary works | Names, dates, title successions, jurisdiction notes. | Work-specific; age alone is not enough because editions, translations, scans, and databases may have rights. | Candidate after work-level rights review and citation record. |
| Modern scholarly databases | Cross-checking contested title histories. | Often copyrighted, licensed, or access-restricted. | Pointer-only unless explicit compatible license exists. |
| Commercial-game data or fan extracts | None. | Not a permissible source for DUCHY facts, mechanics, maps, or UI. | Forbidden. |

## Source Links Checked

- Wikidata licensing: https://www.wikidata.org/wiki/Wikidata:Licensing
- OpenHistoricalMap copyright: https://www.openhistoricalmap.org/copyright
- Wikimedia Foundation Terms of Use:
  https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use

## Inventory Rules

1. Prefer facts with stable source identifiers and source pointers.
2. Treat source text, map geometry, and structured claims separately.
3. Preserve the source license and retrieval date in the future source record.
4. Do not combine share-alike source material into MIT code or fixtures without
   an explicit packaging decision.
5. Do not treat commercial-game names, IDs, maps, or title history as source
   material.
