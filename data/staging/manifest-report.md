# DUCHY Candidate Manifest Report

source_manifest: data\staging\example.manifest
candidates: 3
pending: 1
reviewed: 1
promoted: 0
rejected: 1

## Pending

- candidate_id: cand-staging-one
  source_id: src-staging-one
  source_url: urn:duchy:staging-one
  notes: First synthetic candidate.

## Reviewed

- candidate_id: cand-staging-two
  source_id: src-staging-two
  source_url: urn:duchy:staging-two
  review_batch_id: batch-example-scale
  import_scope: title_identity_only
  rank_basis: normalized
  entity_class: duchy
  source_claims_used: label, inception, dissolution
  confidence_detail: wikidata_structured_single
  parentage_status: none_reviewed
  query_readiness: existence_only
  notes: Second synthetic candidate ready for fact extraction.

## Promoted

none

## Rejected

- candidate_id: cand-staging-three
  source_id: src-staging-three
  source_url: urn:duchy:staging-three
  exclusion_reason: scope_deferred
  notes: Rejected synthetic candidate.

