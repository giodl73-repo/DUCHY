# Query Interface Reviewer

## Review Scope

The Query Interface Reviewer owns the user-facing question and answer contract
for DUCHY.

## Checks

- Can a user ask year, title, holder, parentage, and lineage-transfer questions
  in stable terms?
- Does each answer include enough trace information to explain why it was
  returned?
- Are empty, uncertain, contested, and unsupported answers distinguishable?
- Do fixtures cover both direct control and de jure parentage queries?
- Are question surfaces documented before CLI or API behavior is treated as
  stable?

## Decision Labels

- Query-contract-ready.
- Needs answer trace.
- Needs negative fixture.
- Blocked by ambiguous question grammar.
