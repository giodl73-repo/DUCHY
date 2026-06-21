# County Parentage Agent Brief

## Purpose

Scale CK3 county search-driver review without weakening source custody.

Agents work from `data/staging/county-parentage-scale-queue.tsv` or one shard
under `data/staging/county-parentage-scale-shards/`. The CK3 row is never an
accepted source-backed fact. It is only a search prompt for independent
historical sources.

## Inputs

- Full queue: `data/staging/county-parentage-scale-queue.tsv`
- Shards: `data/staging/county-parentage-scale-shards/county-scale-NNN.tsv`
- Pilot example: `data/staging/county-parentage-pilot-001.tsv`
- Pilot review: `data/staging/county-parentage-pilot-001-review.md`
- Metrics baseline: `data/staging/parentage-change-report.md`

## Status Meaning

- `accepted_parentage`: already accepted in fixtures. Use as a metrics seed;
  do not re-import.
- `title_needs_parentage_review`: title facts are accepted, but parentage is
  missing or blocked. Search for independent parentage evidence.
- `rank_semantics_review`: a source lead has dates but entity/rank semantics
  block promotion. Decide whether the entity maps to an existing DUCHY rank or
  should stay blocked.
- `source_resolution_deferred`: the current top lead is weak, modern,
  administrative, municipal, non-title, or otherwise not safe to promote.
  Search for a better independent historical title source.
- `source_resolution_needed`: no useful lead exists yet. Search from the CK3
  seed and its historical names.

## Output Contract

Each agent writes one TSV and one Markdown review file under
`data/staging/county-agent-results/`:

- `county-scale-NNN-results.tsv`
- `county-scale-NNN-review.md`

The TSV header must be:

```text
ck_id	county	decision	recommended_source_id	recommended_source_url	recommended_title_id	recommended_title_label	recommended_rank	recommended_exists	recommended_parent_id	recommended_parent_label	recommended_parent_span	confidence	next_action	notes
```

Allowed `decision` values:

- `ready_parentage_packet`
- `ready_title_packet`
- `rank_policy_blocked`
- `source_resolution_blocked`
- `already_accepted`
- `reject_bad_lead`

## Review Rules

1. Do not edit `fixtures/first-real.sources` or `fixtures/first-real.facts`.
2. Do not create promotion packets unless a maintainer asks for that shard to
   advance after review.
3. Prefer independent historical sources over broad modern geography pages.
4. Wikidata structured claims can support label, rank, and bounded existence
   when the entity type is clear; parentage normally needs direct relation
   evidence or a second corroborating source.
5. Record blocked rows explicitly. A good blocker is useful data.
6. Do not treat CK3 duchy/kingdom/empire columns as historical facts.

## First Farm Priority

Start with `county-scale-004.tsv` because it contains the only current
`title_needs_parentage_review` row, Brycheiniog, plus one accepted county row
and one rank-semantics row. Then farm shards with `rank_semantics_review` rows:
`county-scale-001.tsv`, `county-scale-002.tsv`, `county-scale-006.tsv`, and
`county-scale-009.tsv`.
