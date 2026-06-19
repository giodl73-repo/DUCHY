# Wave: Source Custody Intake

## Goal

Move DUCHY from seed-only fixtures toward source-aware historical data without
importing real title facts before custody review.

## Thesis

DUCHY can only answer real European historical questions responsibly if every
source-backed claim carries source metadata, allowed-use posture, confidence,
and review evidence. The next safe implementation layer is metadata-only source
records and validation.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Metadata-only source records | complete | Add source records, allowed-use posture, review decisions, and validation. |
| 02 | Metadata source file format | complete | Define and parse a fixture file shape for source records without importing historical facts. |
| 03 | Source-backed fact gate | pending | Validate fact records against accepted source records and confidence labels. |
| 04 | Contested-history review packet | pending | Represent conflicting source-backed claims without overwriting them. |

## Success Criteria

- Source records validate required metadata.
- Metadata-only records cannot masquerade as accepted historical facts.
- Review decisions reference existing source records.
- Validation commands pass.
- Real title facts remain blocked until the fact gate exists.
