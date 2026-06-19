# Source-Custody Review Gate

## Acceptance Checklist

Before source-backed data enters DUCHY:

- [ ] Source record exists.
- [ ] License and rights posture are recorded.
- [ ] Allowed use is explicit.
- [ ] Attribution/share-alike obligations are recorded or marked not applicable.
- [ ] Fact extraction avoids restricted text, maps, and commercial-game data.
- [ ] Confidence label is assigned.
- [ ] Conflicting sources are preserved as contested alternatives.
- [ ] Query answer can cite source record IDs in its trace.
- [ ] Validation command is named.

## Decision Labels

| Label | Meaning |
|---|---|
| `accepted_metadata_only` | Source may be cited as a pointer, but no facts imported. |
| `accepted_structured_claims` | Structured facts may be imported under the recorded license posture. |
| `accepted_package_boundary` | Data may be packaged separately with non-MIT obligations. |
| `blocked_rights` | Rights posture blocks use. |
| `blocked_quality` | Source is too uncertain or weak for accepted facts. |
| `blocked_scope` | Source belongs to maps, mechanics, or another future package. |

## Current Review Decision

The seed-query foundation is custody-safe because it uses fictional seed
fixtures only. Real source import remains blocked until a concrete source record
passes this gate.
