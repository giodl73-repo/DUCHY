# Source Schema Sketch

## Source Record

Future source-backed fixtures should include a source record with at least:

| Field | Required | Meaning |
|---|---:|---|
| `source_id` | yes | Stable DUCHY-local ID. |
| `source_class` | yes | `wikidata`, `openhistoricalmap`, `wikimedia_text`, `public_domain_work`, `scholarly_database`, or `other`. |
| `source_url` | yes | URL or durable citation pointer. |
| `license` | yes | License or rights posture as observed. |
| `retrieved_on` | yes | Date the source posture was checked. |
| `allowed_use` | yes | `metadata_only`, `structured_claims`, `geometry`, `text_excerpt`, or `blocked`. |
| `attribution` | conditional | Required attribution text when applicable. |
| `notes` | no | Source-specific caveats. |

## Rust Model

The current crate implements the metadata-only layer:

| Rust item | Purpose |
|---|---|
| `SourceRecord` | Source metadata record. |
| `SourceKind` | Source class such as Wikidata, OpenHistoricalMap, Wikimedia text, public-domain work, scholarly database, or other. |
| `AllowedUse` | Metadata-only, structured claims, geometry, text excerpt, or blocked. |
| `SourceReview` | Review decision attached to a source record. |
| `SourceReviewDecision` | Accepted/blocked source-custody decision labels. |
| `SourceCatalog` | Validates source records and review references. |

The implemented seed catalog is policy-only and contains no historical title
facts.

## Fact Record

Future title facts should include:

| Field | Required | Meaning |
|---|---:|---|
| `fact_id` | yes | Stable DUCHY-local ID. |
| `subject_id` | yes | Area or title identity. |
| `claim_kind` | yes | `title_exists`, `area_title`, `parentage`, `holder`, `event`, or `name`. |
| `span` | conditional | Date span for temporal claims. |
| `value` | yes | Referenced title, holder, event kind, name, or status. |
| `source_ids` | yes | Source records supporting the claim. |
| `confidence` | yes | Confidence label from `CONFIDENCE_MODEL.md`. |
| `conflict_group` | no | ID joining alternative contested claims. |

## Non-Goals

- This is not a final serialization format.
- This does not authorize any source import.
- This does not define geometry storage.
