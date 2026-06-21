# DUCHY Parentage Gap Review Report

source_tsv: data\staging\parentage-gap-shards\batch-008.tsv
gap_rows: 5

## Priority Counts

| Priority | Rows |
|---|---:|
| high_parentage_review | 1 |
| medium_parentage_review | 3 |
| root_or_successor_review | 1 |

## Rank Counts

| Rank | Rows |
|---|---:|
| Duchy | 1 |
| Empire | 1 |
| Kingdom | 3 |

## Review Rows

### title-q905131 | Kingdom of Breifne

- rank: Kingdom
- exists: 700..1256
- review_priority: medium_parentage_review
- notes: Find reviewed empire, union, confederation, or successor-context source.

### title-q926295 | Italian Empire

- rank: Empire
- exists: 1882..1946
- review_priority: root_or_successor_review
- notes: May be a root title; review only if successor, union, or super-entity claim exists.

### title-q954585 | Kingdom of Brycheiniog

- rank: Kingdom
- exists: 450..1045
- review_priority: medium_parentage_review
- notes: Find reviewed empire, union, confederation, or successor-context source.

### title-q956451 | Kingdom of Dyfed

- rank: Kingdom
- exists: 410..920
- review_priority: medium_parentage_review
- notes: Find reviewed empire, union, confederation, or successor-context source.

### title-q958291 | United Principalities of Moldavia and Wallachia

- rank: Duchy
- exists: 1859..1881
- review_priority: high_parentage_review
- notes: Find reviewed kingdom or empire parentage source.

