# Confidence Model

## Labels

| Label | Meaning |
|---|---|
| `seed` | Fictional or non-authoritative test fixture. |
| `metadata_pointer` | Source is identified, but no fact has been accepted. |
| `single_source` | Fact is supported by one accepted source record. |
| `multi_source` | Fact is supported by two or more compatible source records. |
| `contested` | Accepted sources disagree or source interpretation is uncertain. |
| `unsupported` | The query asks for a fact DUCHY cannot support with current data. |

## Conflict Rules

- Do not silently overwrite one source with another.
- Represent disagreements as alternative claims or contested status.
- Record date-span uncertainty explicitly when a source gives approximate or
  conflicting dates.
- Keep de jure parentage, de facto control, title holder, and area identity as
  separate claims.

## Answer Rules

Future source-backed answers must include:

- source class,
- confidence label,
- source record IDs,
- retrieval or publication pointer,
- trace rows used to produce the answer,
- uncertainty or contested status when applicable.

Seed answers may use `SourceClass::Seed` and trace codes without source records.
