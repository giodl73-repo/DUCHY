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

## Metadata File Format

The source metadata fixture format is dependency-free line-oriented text:

```text
source_id: src-example
source_kind: wikidata
source_url: https://www.wikidata.org/wiki/Wikidata:Licensing
license: CC0 structured data
retrieved_on: 2026-06-19
allowed_use: metadata_only
review_decision: accepted_metadata_only
reviewer: Source Custody Reviewer
review_note: Policy pointer only.
---
source_id: src-another
...
```

Rules:

- Records are separated by `---`.
- Blank lines and `#` comments are ignored.
- Required enum values use the lowercase labels in this file.
- This format carries source metadata and review decisions only; it does not
  carry historical facts.

The seed metadata fixture is `fixtures/source-policy.sources`. The reviewed
real source fixture is `fixtures/first-real.sources`.

## Fact Record

Future title facts should include:

| Field | Required | Meaning |
|---|---:|---|
| `fact_id` | yes | Stable DUCHY-local ID. |
| `subject_id` | yes | Area or title identity. |
| `claim_kind` | yes | `title_exists`, `area_title`, `parentage`, `holder`, `event`, `name`, or `rank`. |
| `span` | conditional | Date span for temporal claims. |
| `value` | yes | Referenced title, holder, event kind, name, or status. |
| `source_ids` | yes | Source records supporting the claim. |
| `confidence` | yes | Confidence label from `CONFIDENCE_MODEL.md`. |
| `conflict_group` | no | ID joining alternative contested claims. |

## Fact File Format

The fact fixture format is also dependency-free line-oriented text:

```text
fact_id: fact-example-name
subject_id: title-example
claim_kind: name
value: Example Duchy
source_ids: src-example
confidence: single_source
---
fact_id: fact-example-exists
subject_id: title-example
claim_kind: title_exists
span: 1815..1918
value: exists
source_ids: src-example
confidence: single_source
```

Rules:

- Records are separated by `---`.
- Blank lines and `#` comments are ignored.
- `source_ids` is a comma-separated list.
- `span` uses `start..end`; open spans use `start..present`.
- `parentage` facts use `subject_id` as the child title and `value` as the
  parent title ID.
- Fact files still require separate source metadata and source-custody review.

The first real fact fixture is `fixtures/first-real.facts`; it must be validated
against `fixtures/first-real.sources`.

## Fact Gate

The current crate implements the fact-gate layer:

| Rust item | Purpose |
|---|---|
| `FactRecord` | Candidate source-backed claim. |
| `ContestedFactGroup` | Review packet for alternative contested fact claims. |
| `ClaimKind` | `title_exists`, `area_title`, `parentage`, `holder`, `event`, `name`, or `rank`. |
| `ConfidenceLabel` | `single_source`, `multi_source`, `contested`, and rejected non-fact labels. |
| `SourceCatalog::validate_fact` | Ensures facts cite reviewed sources with allowed use and coherent confidence. |
| `source_backed_parentage_from_facts` | Converts reviewed parentage facts into `ParentageSpan` records after title materialization. |

Fact-gate rules:

- `metadata_only` sources cannot support fact claims.
- source reviews must allow fact claims through `accepted_structured_claims` or
  `accepted_package_boundary`.
- `single_source` requires exactly one source.
- `multi_source` requires at least two sources.
- `contested` requires a `conflict_group`.
- `seed`, `metadata_pointer`, and `unsupported` are not accepted as
  source-backed fact confidence labels.
- Materialization rejects fact sets containing contested facts.
- Parentage materialization requires a span and already-materialized child and
  parent titles.

## Non-Goals

- This is not a final serialization format.
- This does not authorize importing unreviewed concrete historical title facts.
- This does not define geometry storage.
