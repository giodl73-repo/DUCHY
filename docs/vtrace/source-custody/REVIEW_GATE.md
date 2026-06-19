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
fixtures only.

The metadata-only source catalog is custody-safe as a policy pointer catalog.
The accepted records identify licensing/copyright pages and do not import
historical facts.

`src-wikidata-q158445` passes this gate for structured claims only. DUCHY imports
only the label/name, normalized title rank, and existence span facts for Grand
Duchy of Mecklenburg-Schwerin from Wikidata Q158445. Prose text, geometry, map
boundary, holder genealogy, and full parentage/control timeline remain out of
scope.

`src-wikidata-q20135` passes this gate for structured claims only. DUCHY imports
only the label/name, normalized title rank, and existence span facts for Grand
Duchy of Hesse from Wikidata Q20135. Prose text, geometry, map boundary, holder
genealogy, and full parentage/control timeline remain out of scope.

`src-wikidata-q43287` passes this gate for structured claims only. DUCHY imports
only the label/name, normalized title rank, and existence span facts for German
Empire from Wikidata Q43287. Prose text, geometry, map boundary, holder
genealogy, and full parentage/control timeline remain out of scope.

`fact-q20135-parent-q43287` passes this gate as a single source-backed
parentage claim from Grand Duchy of Hesse to German Empire for 1871-1918. This
authorizes one direct title-path relation only; it does not import intermediate
constituent-state semantics, holders, borders, legal prose, or a complete
German Empire hierarchy.

`fact-q158445-parent-q43287` passes this gate as a single source-backed
parentage claim from Grand Duchy of Mecklenburg-Schwerin to German Empire for
1871-1918. This authorizes one direct title-path relation only; it does not
import intermediate confederation/state semantics, holders, borders, legal
prose, or a complete German Empire hierarchy.

`src-wikidata-q27306` passes this gate for structured claims only. DUCHY imports
only the label/name, normalized title rank, existence span, and one country
relation to German Empire for Kingdom of Prussia from Wikidata Q27306. Prose
text, geometry, map boundary, holder genealogy, provincial structure, and full
parentage/control timeline remain out of scope.
