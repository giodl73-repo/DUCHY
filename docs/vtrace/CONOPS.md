# DUCHY Concept Of Operations

## Scope

This CONOPS defines how DUCHY should answer lineage and territorial-transfer
questions over title timelines.

## Operating Flow

```text
fixture/source row
  -> title identity and rank
  -> yearly parentage/control span
  -> continuity event
  -> query
  -> answer with trace and uncertainty
```

## Primary Scenarios

| Scenario ID | Actor | Question | Expected Answer Shape |
|---|---|---|---|
| CONOPS-001 | Game designer | Which duchy contained county C in year Y? | Title path for year Y, including county, duchy, kingdom, and empire where known. |
| CONOPS-002 | Game designer | When did area A move between duchies during range R? | Ordered transfer list with from-title, to-title, year/span, and event cause. |
| CONOPS-003 | World builder | What is the lineage of title T? | Ordered continuity events and parentage/control changes for T. |
| CONOPS-004 | Campaign author | Who held title T in year Y, and who claimed it de jure? | Holder/control answer separated from de jure parentage answer. |
| CONOPS-005 | Source reviewer | Is this answer historical, fictional, contested, or unsupported? | Answer carries source class, confidence, and gap/uncertainty labels. |

## Operational Boundaries

- A title can exist without a known holder.
- An area can have de jure parentage that differs from de facto control.
- A transfer can be direct, contested, split, or uncertain.
- Empty results must distinguish "unknown", "not yet modeled", and "does not
  exist in this year" when the data model supports that distinction.
- Fixture data is valid for testing semantics, not for historical claims.

## Current Foundation State

The current Rust crate validates title ranks, existence spans, de jure parentage,
control spans, and continuity events for a seed fixture. It does not yet provide
the transfer query surface, answer trace object, source/confidence fields, or
range queries.
