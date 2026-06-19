use std::collections::HashMap;

pub type Year = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleRank {
    County,
    Duchy,
    Kingdom,
    Empire,
}

impl TitleRank {
    pub fn parent_rank(self) -> Option<Self> {
        match self {
            Self::County => Some(Self::Duchy),
            Self::Duchy => Some(Self::Kingdom),
            Self::Kingdom => Some(Self::Empire),
            Self::Empire => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearSpan {
    pub start: Year,
    pub end: Option<Year>,
}

impl YearSpan {
    pub fn new(start: Year, end: Option<Year>) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, year: Year) -> bool {
        self.start <= year && self.end.map_or(true, |end| year <= end)
    }

    fn is_valid(&self) -> bool {
        self.end.map_or(true, |end| self.start <= end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title {
    pub id: String,
    pub name: String,
    pub rank: TitleRank,
    pub exists: YearSpan,
    pub de_jure_parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub exists: YearSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaTitleSpan {
    pub area_id: String,
    pub title_id: String,
    pub span: YearSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentageSpan {
    pub child_title_id: String,
    pub parent_title_id: String,
    pub span: YearSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitlePathAnswer {
    pub subject_id: String,
    pub year: Year,
    pub titles: Vec<TitlePathStep>,
    pub events: Vec<ContinuityEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitlePathStep {
    pub title_id: String,
    pub name: String,
    pub rank: TitleRank,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferAnswer {
    pub subject_id: String,
    pub rank: TitleRank,
    pub start: Year,
    pub end: Year,
    pub transfers: Vec<TransferStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferStep {
    pub year: Year,
    pub from_title_id: String,
    pub from_name: String,
    pub to_title_id: String,
    pub to_name: String,
    pub event_kind: Option<ContinuityEventKind>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelope<T> {
    pub status: QueryStatus,
    pub source_class: SourceClass,
    pub trace: Vec<TraceNote>,
    pub answer: Option<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryStatus {
    Answered,
    Empty,
    Unknown,
    Unsupported,
    Contested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceClass {
    Seed,
    Fictional,
    SourceBacked,
    Contested,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceNote {
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRecord {
    pub source_id: String,
    pub source_kind: SourceKind,
    pub source_url: String,
    pub license: String,
    pub retrieved_on: String,
    pub allowed_use: AllowedUse,
    pub attribution: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Wikidata,
    OpenHistoricalMap,
    WikimediaText,
    PublicDomainWork,
    ScholarlyDatabase,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedUse {
    MetadataOnly,
    StructuredClaims,
    Geometry,
    TextExcerpt,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceReview {
    pub source_id: String,
    pub decision: SourceReviewDecision,
    pub reviewer: String,
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceReviewDecision {
    AcceptedMetadataOnly,
    AcceptedStructuredClaims,
    AcceptedPackageBoundary,
    BlockedRights,
    BlockedQuality,
    BlockedScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactRecord {
    pub fact_id: String,
    pub subject_id: String,
    pub claim_kind: ClaimKind,
    pub span: Option<YearSpan>,
    pub value: String,
    pub source_ids: Vec<String>,
    pub confidence: ConfidenceLabel,
    pub conflict_group: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimKind {
    TitleExists,
    AreaTitle,
    Parentage,
    Holder,
    Event,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLabel {
    Seed,
    MetadataPointer,
    SingleSource,
    MultiSource,
    Contested,
    Unsupported,
}

#[derive(Debug, Clone, Default)]
pub struct SourceCatalog {
    records: HashMap<String, SourceRecord>,
    reviews: Vec<SourceReview>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContinuityEventKind {
    Created,
    Inherited,
    Conquered,
    Partitioned,
    Extinct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityEvent {
    pub year: Year,
    pub title_id: String,
    pub kind: ContinuityEventKind,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSpan {
    pub title_id: String,
    pub holder: String,
    pub span: YearSpan,
}

#[derive(Debug, Clone, Default)]
pub struct TitleTimeline {
    areas: HashMap<String, Area>,
    titles: HashMap<String, Title>,
    area_titles: Vec<AreaTitleSpan>,
    parentage: Vec<ParentageSpan>,
    control: Vec<ControlSpan>,
    events: Vec<ContinuityEvent>,
}

impl TitleTimeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_title(&mut self, title: Title) {
        self.titles.insert(title.id.clone(), title);
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.insert(area.id.clone(), area);
    }

    pub fn add_area_title(&mut self, area_title: AreaTitleSpan) {
        self.area_titles.push(area_title);
    }

    pub fn add_parentage(&mut self, parentage: ParentageSpan) {
        self.parentage.push(parentage);
    }

    pub fn add_control(&mut self, control: ControlSpan) {
        self.control.push(control);
    }

    pub fn add_event(&mut self, event: ContinuityEvent) {
        self.events.push(event);
    }

    pub fn titles_in_year(&self, year: Year) -> Vec<&Title> {
        let mut titles: Vec<&Title> = self
            .titles
            .values()
            .filter(|title| title.exists.contains(year))
            .collect();
        titles.sort_by(|left, right| left.rank.cmp(&right.rank).then(left.id.cmp(&right.id)));
        titles
    }

    pub fn holder_in_year(&self, title_id: &str, year: Year) -> Option<&str> {
        self.control
            .iter()
            .find(|control| control.title_id == title_id && control.span.contains(year))
            .map(|control| control.holder.as_str())
    }

    pub fn title_for_area_in_year(&self, area_id: &str, year: Year) -> Option<&Title> {
        self.area_titles
            .iter()
            .find(|area_title| area_title.area_id == area_id && area_title.span.contains(year))
            .and_then(|area_title| self.titles.get(&area_title.title_id))
    }

    pub fn parentage_for_title_in_year(
        &self,
        child_title_id: &str,
        year: Year,
    ) -> Option<&ParentageSpan> {
        self.parentage.iter().find(|parentage| {
            parentage.child_title_id == child_title_id && parentage.span.contains(year)
        })
    }

    pub fn parent_title_in_year(&self, child_title_id: &str, year: Year) -> Option<&Title> {
        self.parentage_for_title_in_year(child_title_id, year)
            .and_then(|parentage| self.titles.get(&parentage.parent_title_id))
    }

    pub fn title_path_for_area_in_year(
        &self,
        area_id: &str,
        year: Year,
    ) -> Option<TitlePathAnswer> {
        let title = self.title_for_area_in_year(area_id, year)?;
        self.title_path_for_title_in_year(&title.id, year)
            .map(|answer| TitlePathAnswer {
                subject_id: area_id.to_string(),
                ..answer
            })
    }

    pub fn title_path_query_for_area_in_year(
        &self,
        area_id: &str,
        year: Year,
    ) -> QueryEnvelope<TitlePathAnswer> {
        if !self.areas.contains_key(area_id) {
            return QueryEnvelope::without_answer(
                QueryStatus::Unknown,
                SourceClass::Seed,
                "area_missing",
                format!("area {area_id} is not present in the seed timeline"),
            );
        }

        match self.title_path_for_area_in_year(area_id, year) {
            Some(answer) => QueryEnvelope::with_answer(
                answer,
                SourceClass::Seed,
                "seed_title_path",
                format!("resolved seed title path for area {area_id} in {year}"),
            ),
            None => QueryEnvelope::without_answer(
                QueryStatus::Empty,
                SourceClass::Seed,
                "no_area_title_span",
                format!("area {area_id} has no modeled title path in {year}"),
            ),
        }
    }

    pub fn title_path_for_title_in_year(
        &self,
        title_id: &str,
        year: Year,
    ) -> Option<TitlePathAnswer> {
        let title = self.titles.get(title_id)?;
        if !title.exists.contains(year) {
            return None;
        }

        let mut titles = vec![TitlePathStep::from_title(title)];
        let mut current_id = title.id.as_str();

        while let Some(parentage) = self.parentage_for_title_in_year(current_id, year) {
            let parent = self.titles.get(&parentage.parent_title_id)?;
            titles.push(TitlePathStep::from_title(parent));
            current_id = parent.id.as_str();
        }

        let events = self
            .events_for_title(title_id)
            .into_iter()
            .cloned()
            .collect();

        Some(TitlePathAnswer {
            subject_id: title_id.to_string(),
            year,
            titles,
            events,
        })
    }

    pub fn transfers_for_area_between(
        &self,
        area_id: &str,
        rank: TitleRank,
        start: Year,
        end: Year,
    ) -> Option<TransferAnswer> {
        let title = self.title_for_area_in_year(area_id, start)?;
        self.transfers_for_title_between(&title.id, rank, start, end)
            .map(|answer| TransferAnswer {
                subject_id: area_id.to_string(),
                ..answer
            })
    }

    pub fn transfers_query_for_area_between(
        &self,
        area_id: &str,
        rank: TitleRank,
        start: Year,
        end: Year,
    ) -> QueryEnvelope<TransferAnswer> {
        if start > end {
            return QueryEnvelope::without_answer(
                QueryStatus::Unsupported,
                SourceClass::Seed,
                "invalid_range",
                format!("start year {start} is after end year {end}"),
            );
        }
        if rank != TitleRank::Duchy {
            return QueryEnvelope::without_answer(
                QueryStatus::Unsupported,
                SourceClass::Seed,
                "unsupported_transfer_rank",
                format!("transfer queries currently support Duchy rank, not {rank:?}"),
            );
        }
        if !self.areas.contains_key(area_id) {
            return QueryEnvelope::without_answer(
                QueryStatus::Unknown,
                SourceClass::Seed,
                "area_missing",
                format!("area {area_id} is not present in the seed timeline"),
            );
        }

        match self.transfers_for_area_between(area_id, rank, start, end) {
            Some(answer) if answer.transfers.is_empty() => QueryEnvelope::with_status(
                QueryStatus::Empty,
                answer,
                SourceClass::Seed,
                "no_transfers",
                format!("area {area_id} has no modeled duchy transfers from {start} to {end}"),
            ),
            Some(answer) => QueryEnvelope::with_answer(
                answer,
                SourceClass::Seed,
                "seed_transfer_query",
                format!("resolved seed duchy transfers for area {area_id} from {start} to {end}"),
            ),
            None => QueryEnvelope::without_answer(
                QueryStatus::Empty,
                SourceClass::Seed,
                "no_transfer_subject",
                format!("area {area_id} has no modeled transfer subject from {start} to {end}"),
            ),
        }
    }

    pub fn transfers_for_title_between(
        &self,
        title_id: &str,
        rank: TitleRank,
        start: Year,
        end: Year,
    ) -> Option<TransferAnswer> {
        if start > end {
            return None;
        }

        let title = self.titles.get(title_id)?;
        if !title.exists.contains(start) {
            return None;
        }

        let mut matching_parentage: Vec<&ParentageSpan> = self
            .parentage
            .iter()
            .filter(|parentage| parentage.child_title_id == title_id)
            .filter(|parentage| span_overlaps(&parentage.span, start, end))
            .filter(|parentage| {
                self.titles
                    .get(&parentage.parent_title_id)
                    .map_or(false, |parent| parent.rank == rank)
            })
            .collect();
        matching_parentage.sort_by_key(|parentage| parentage.span.start);

        let mut transfers = Vec::new();
        for window in matching_parentage.windows(2) {
            let from = window[0];
            let to = window[1];
            if from.parent_title_id == to.parent_title_id {
                continue;
            }
            let from_title = self.titles.get(&from.parent_title_id)?;
            let to_title = self.titles.get(&to.parent_title_id)?;
            let event = self.event_for_title_in_year(title_id, to.span.start);
            transfers.push(TransferStep {
                year: to.span.start,
                from_title_id: from_title.id.clone(),
                from_name: from_title.name.clone(),
                to_title_id: to_title.id.clone(),
                to_name: to_title.name.clone(),
                event_kind: event.map(|event| event.kind.clone()),
                note: event.map(|event| event.note.clone()),
            });
        }

        Some(TransferAnswer {
            subject_id: title_id.to_string(),
            rank,
            start,
            end,
            transfers,
        })
    }

    fn event_for_title_in_year(&self, title_id: &str, year: Year) -> Option<&ContinuityEvent> {
        self.events
            .iter()
            .find(|event| event.title_id == title_id && event.year == year)
    }

    pub fn events_for_title(&self, title_id: &str) -> Vec<&ContinuityEvent> {
        let mut events: Vec<&ContinuityEvent> = self
            .events
            .iter()
            .filter(|event| event.title_id == title_id)
            .collect();
        events.sort_by_key(|event| event.year);
        events
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for area in self.areas.values() {
            if area.id.trim().is_empty() {
                errors.push("area id must not be empty".to_string());
            }
            if !area.exists.is_valid() {
                errors.push(format!("{} has an invalid area span", area.id));
            }
        }

        for title in self.titles.values() {
            if title.id.trim().is_empty() {
                errors.push("title id must not be empty".to_string());
            }
            if !title.exists.is_valid() {
                errors.push(format!("{} has an invalid existence span", title.id));
            }
            match (&title.de_jure_parent, title.rank.parent_rank()) {
                (Some(parent_id), Some(expected_rank)) => match self.titles.get(parent_id) {
                    Some(parent) if parent.rank == expected_rank => {}
                    Some(parent) => errors.push(format!(
                        "{} expects parent rank {:?}, found {:?}",
                        title.id, expected_rank, parent.rank
                    )),
                    None => errors.push(format!(
                        "{} references missing parent {}",
                        title.id, parent_id
                    )),
                },
                (Some(parent_id), None) => {
                    errors.push(format!("{} cannot have parent {}", title.id, parent_id));
                }
                (None, Some(_)) => {}
                (None, None) => {}
            }
        }

        for area_title in &self.area_titles {
            match (
                self.areas.get(&area_title.area_id),
                self.titles.get(&area_title.title_id),
            ) {
                (Some(area), Some(title)) => {
                    if !area_title.span.is_valid() {
                        errors.push(format!(
                            "{} -> {} has an invalid area-title span",
                            area_title.area_id, area_title.title_id
                        ));
                    }
                    if area_title.span.start < area.exists.start {
                        errors.push(format!(
                            "{} area-title span starts before area exists",
                            area_title.area_id
                        ));
                    }
                    if area_title.span.start < title.exists.start {
                        errors.push(format!(
                            "{} area-title span starts before title exists",
                            area_title.title_id
                        ));
                    }
                    if let Some(area_end) = area.exists.end {
                        if area_title.span.end.map_or(true, |end| end > area_end) {
                            errors.push(format!(
                                "{} area-title span outlives area",
                                area_title.area_id
                            ));
                        }
                    }
                    if let Some(title_end) = title.exists.end {
                        if area_title.span.end.map_or(true, |end| end > title_end) {
                            errors.push(format!(
                                "{} area-title span outlives title",
                                area_title.title_id
                            ));
                        }
                    }
                }
                (None, Some(_)) => errors.push(format!(
                    "area-title span references missing area {}",
                    area_title.area_id
                )),
                (Some(_), None) => errors.push(format!(
                    "area-title span references missing title {}",
                    area_title.title_id
                )),
                (None, None) => errors.push(format!(
                    "area-title span references missing area {} and title {}",
                    area_title.area_id, area_title.title_id
                )),
            }
        }

        for parentage in &self.parentage {
            match (
                self.titles.get(&parentage.child_title_id),
                self.titles.get(&parentage.parent_title_id),
            ) {
                (Some(child), Some(parent)) => {
                    if !parentage.span.is_valid() {
                        errors.push(format!(
                            "{} -> {} has an invalid parentage span",
                            parentage.child_title_id, parentage.parent_title_id
                        ));
                    }
                    match child.rank.parent_rank() {
                        Some(expected_rank) if parent.rank == expected_rank => {}
                        Some(expected_rank) => errors.push(format!(
                            "{} expects temporal parent rank {:?}, found {:?}",
                            child.id, expected_rank, parent.rank
                        )),
                        None => errors.push(format!(
                            "{} cannot have temporal parent {}",
                            child.id, parent.id
                        )),
                    }
                    if parentage.span.start < child.exists.start {
                        errors.push(format!(
                            "{} parentage starts before child title exists",
                            child.id
                        ));
                    }
                    if parentage.span.start < parent.exists.start {
                        errors.push(format!(
                            "{} parentage starts before parent title exists",
                            parent.id
                        ));
                    }
                    if let Some(child_end) = child.exists.end {
                        if parentage.span.end.map_or(true, |end| end > child_end) {
                            errors.push(format!("{} parentage outlives child title", child.id));
                        }
                    }
                    if let Some(parent_end) = parent.exists.end {
                        if parentage.span.end.map_or(true, |end| end > parent_end) {
                            errors.push(format!("{} parentage outlives parent title", parent.id));
                        }
                    }
                }
                (None, Some(_)) => errors.push(format!(
                    "parentage references missing child title {}",
                    parentage.child_title_id
                )),
                (Some(_), None) => errors.push(format!(
                    "parentage references missing parent title {}",
                    parentage.parent_title_id
                )),
                (None, None) => errors.push(format!(
                    "parentage references missing child title {} and parent title {}",
                    parentage.child_title_id, parentage.parent_title_id
                )),
            }
        }

        for control in &self.control {
            match self.titles.get(&control.title_id) {
                Some(title) => {
                    if !control.span.is_valid() {
                        errors.push(format!("{} has an invalid control span", control.title_id));
                    }
                    if control.span.start < title.exists.start {
                        errors.push(format!(
                            "{} control starts before title exists",
                            control.title_id
                        ));
                    }
                    if let Some(title_end) = title.exists.end {
                        if control.span.end.map_or(true, |end| end > title_end) {
                            errors.push(format!("{} control outlives title", control.title_id));
                        }
                    }
                }
                None => errors.push(format!(
                    "control references missing title {}",
                    control.title_id
                )),
            }
        }

        for event in &self.events {
            match self.titles.get(&event.title_id) {
                Some(title) if title.exists.contains(event.year) => {}
                Some(_) => errors.push(format!(
                    "event for {} falls outside title span",
                    event.title_id
                )),
                None => errors.push(format!("event references missing title {}", event.title_id)),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl SourceRecord {
    fn allows_fact_claims(&self) -> bool {
        matches!(
            self.allowed_use,
            AllowedUse::StructuredClaims | AllowedUse::Geometry | AllowedUse::TextExcerpt
        )
    }
}

impl SourceReviewDecision {
    fn allows_fact_claims(self) -> bool {
        matches!(
            self,
            Self::AcceptedStructuredClaims | Self::AcceptedPackageBoundary
        )
    }
}

impl<T> QueryEnvelope<T> {
    fn with_answer(answer: T, source_class: SourceClass, code: &str, detail: String) -> Self {
        Self::with_status(QueryStatus::Answered, answer, source_class, code, detail)
    }

    fn with_status(
        status: QueryStatus,
        answer: T,
        source_class: SourceClass,
        code: &str,
        detail: String,
    ) -> Self {
        Self {
            status,
            source_class,
            trace: vec![TraceNote::new(code, detail)],
            answer: Some(answer),
        }
    }

    fn without_answer(
        status: QueryStatus,
        source_class: SourceClass,
        code: &str,
        detail: String,
    ) -> Self {
        Self {
            status,
            source_class,
            trace: vec![TraceNote::new(code, detail)],
            answer: None,
        }
    }
}

impl TraceNote {
    fn new(code: &str, detail: String) -> Self {
        Self {
            code: code.to_string(),
            detail,
        }
    }
}

impl SourceRecord {
    pub fn metadata_only(
        source_id: &str,
        source_kind: SourceKind,
        source_url: &str,
        license: &str,
        retrieved_on: &str,
    ) -> Self {
        Self {
            source_id: source_id.to_string(),
            source_kind,
            source_url: source_url.to_string(),
            license: license.to_string(),
            retrieved_on: retrieved_on.to_string(),
            allowed_use: AllowedUse::MetadataOnly,
            attribution: None,
            notes: None,
        }
    }
}

impl SourceCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_record(&mut self, record: SourceRecord) {
        self.records.insert(record.source_id.clone(), record);
    }

    pub fn add_review(&mut self, review: SourceReview) {
        self.reviews.push(review);
    }

    pub fn record(&self, source_id: &str) -> Option<&SourceRecord> {
        self.records.get(source_id)
    }

    pub fn latest_review(&self, source_id: &str) -> Option<&SourceReview> {
        self.reviews
            .iter()
            .rev()
            .find(|review| review.source_id == source_id)
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for record in self.records.values() {
            if record.source_id.trim().is_empty() {
                errors.push("source id must not be empty".to_string());
            }
            if record.source_url.trim().is_empty() {
                errors.push(format!("{} source_url must not be empty", record.source_id));
            }
            if record.license.trim().is_empty() {
                errors.push(format!("{} license must not be empty", record.source_id));
            }
            if record.retrieved_on.trim().is_empty() {
                errors.push(format!(
                    "{} retrieved_on must not be empty",
                    record.source_id
                ));
            }
            if requires_attribution(record.allowed_use) && record.attribution.is_none() {
                errors.push(format!(
                    "{} requires attribution for {:?}",
                    record.source_id, record.allowed_use
                ));
            }
        }

        for review in &self.reviews {
            if !self.records.contains_key(&review.source_id) {
                errors.push(format!(
                    "review references missing source {}",
                    review.source_id
                ));
            }
            if review.reviewer.trim().is_empty() {
                errors.push(format!("{} review has no reviewer", review.source_id));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_fact(&self, fact: &FactRecord) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if fact.fact_id.trim().is_empty() {
            errors.push("fact id must not be empty".to_string());
        }
        if fact.subject_id.trim().is_empty() {
            errors.push(format!("{} subject_id must not be empty", fact.fact_id));
        }
        if fact.value.trim().is_empty() {
            errors.push(format!("{} value must not be empty", fact.fact_id));
        }
        if fact.source_ids.is_empty() {
            errors.push(format!("{} must cite at least one source", fact.fact_id));
        }
        if let Some(span) = &fact.span {
            if !span.is_valid() {
                errors.push(format!("{} has invalid fact span", fact.fact_id));
            }
        }

        match fact.confidence {
            ConfidenceLabel::SingleSource if fact.source_ids.len() != 1 => errors.push(format!(
                "{} single_source confidence requires exactly one source",
                fact.fact_id
            )),
            ConfidenceLabel::MultiSource if fact.source_ids.len() < 2 => errors.push(format!(
                "{} multi_source confidence requires at least two sources",
                fact.fact_id
            )),
            ConfidenceLabel::Contested if fact.conflict_group.is_none() => errors.push(format!(
                "{} contested confidence requires a conflict_group",
                fact.fact_id
            )),
            ConfidenceLabel::Seed
            | ConfidenceLabel::MetadataPointer
            | ConfidenceLabel::Unsupported => {
                errors.push(format!(
                    "{} confidence {:?} is not accepted for source-backed facts",
                    fact.fact_id, fact.confidence
                ));
            }
            _ => {}
        }

        for source_id in &fact.source_ids {
            match self.records.get(source_id) {
                Some(record) => {
                    if !record.allows_fact_claims() {
                        errors.push(format!(
                            "{} source {} is not allowed for fact claims",
                            fact.fact_id, source_id
                        ));
                    }
                    match self.latest_review(source_id) {
                        Some(review) if review.decision.allows_fact_claims() => {}
                        Some(review) => errors.push(format!(
                            "{} source {} review {:?} does not allow fact claims",
                            fact.fact_id, source_id, review.decision
                        )),
                        None => errors.push(format!(
                            "{} source {} has no review decision",
                            fact.fact_id, source_id
                        )),
                    }
                }
                None => errors.push(format!(
                    "{} references missing source {}",
                    fact.fact_id, source_id
                )),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn from_metadata_text(input: &str) -> Result<Self, Vec<String>> {
        let mut catalog = SourceCatalog::new();
        let mut errors = Vec::new();

        for (index, block) in input.split("\n---").enumerate() {
            let block = block.trim();
            if block.is_empty() {
                continue;
            }
            match parse_source_record_block(block) {
                Ok((record, review)) => {
                    catalog.add_record(record);
                    if let Some(review) = review {
                        catalog.add_review(review);
                    }
                }
                Err(mut block_errors) => {
                    for error in block_errors.drain(..) {
                        errors.push(format!("record {}: {error}", index + 1));
                    }
                }
            }
        }

        if errors.is_empty() {
            catalog.validate()?;
            Ok(catalog)
        } else {
            Err(errors)
        }
    }
}

fn parse_source_record_block(
    block: &str,
) -> Result<(SourceRecord, Option<SourceReview>), Vec<String>> {
    let mut values = HashMap::new();
    let mut errors = Vec::new();

    for (line_index, line) in block.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            errors.push(format!("line {} is not key: value", line_index + 1));
            continue;
        };
        values.insert(key.trim().to_string(), value.trim().to_string());
    }

    let source_id = take_required(&values, "source_id", &mut errors);
    let source_kind = parse_required_source_kind(&values, "source_kind", &mut errors);
    let source_url = take_required(&values, "source_url", &mut errors);
    let license = take_required(&values, "license", &mut errors);
    let retrieved_on = take_required(&values, "retrieved_on", &mut errors);
    let allowed_use = parse_required_allowed_use(&values, "allowed_use", &mut errors);
    let attribution = values
        .get("attribution")
        .filter(|value| !value.is_empty())
        .cloned();
    let notes = values
        .get("notes")
        .filter(|value| !value.is_empty())
        .cloned();

    let review_decision = values
        .get("review_decision")
        .map(|value| parse_source_review_decision(value))
        .transpose()
        .unwrap_or_else(|error| {
            errors.push(error);
            None
        });
    let reviewer = values.get("reviewer").cloned();
    let review_note = values.get("review_note").cloned();

    if !errors.is_empty() {
        return Err(errors);
    }

    let record = SourceRecord {
        source_id: source_id.clone(),
        source_kind,
        source_url,
        license,
        retrieved_on,
        allowed_use,
        attribution,
        notes,
    };

    let review = review_decision.map(|decision| SourceReview {
        source_id,
        decision,
        reviewer: reviewer.unwrap_or_else(|| "Source Custody Reviewer".to_string()),
        note: review_note.unwrap_or_else(|| "Parsed metadata source review.".to_string()),
    });

    Ok((record, review))
}

fn take_required(values: &HashMap<String, String>, key: &str, errors: &mut Vec<String>) -> String {
    match values.get(key) {
        Some(value) if !value.trim().is_empty() => value.clone(),
        _ => {
            errors.push(format!("missing required field {key}"));
            String::new()
        }
    }
}

fn parse_required_source_kind(
    values: &HashMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> SourceKind {
    match values.get(key) {
        Some(value) => match parse_source_kind(value) {
            Some(kind) => kind,
            None => {
                errors.push(format!("invalid source_kind {value}"));
                SourceKind::Other
            }
        },
        None => {
            errors.push(format!("missing required field {key}"));
            SourceKind::Other
        }
    }
}

fn parse_required_allowed_use(
    values: &HashMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> AllowedUse {
    match values.get(key) {
        Some(value) => match parse_allowed_use(value) {
            Some(allowed_use) => allowed_use,
            None => {
                errors.push(format!("invalid allowed_use {value}"));
                AllowedUse::Blocked
            }
        },
        None => {
            errors.push(format!("missing required field {key}"));
            AllowedUse::Blocked
        }
    }
}

fn parse_source_kind(value: &str) -> Option<SourceKind> {
    match value {
        "wikidata" => Some(SourceKind::Wikidata),
        "openhistoricalmap" => Some(SourceKind::OpenHistoricalMap),
        "wikimedia_text" => Some(SourceKind::WikimediaText),
        "public_domain_work" => Some(SourceKind::PublicDomainWork),
        "scholarly_database" => Some(SourceKind::ScholarlyDatabase),
        "other" => Some(SourceKind::Other),
        _ => None,
    }
}

fn parse_allowed_use(value: &str) -> Option<AllowedUse> {
    match value {
        "metadata_only" => Some(AllowedUse::MetadataOnly),
        "structured_claims" => Some(AllowedUse::StructuredClaims),
        "geometry" => Some(AllowedUse::Geometry),
        "text_excerpt" => Some(AllowedUse::TextExcerpt),
        "blocked" => Some(AllowedUse::Blocked),
        _ => None,
    }
}

fn parse_source_review_decision(value: &str) -> Result<SourceReviewDecision, String> {
    match value {
        "accepted_metadata_only" => Ok(SourceReviewDecision::AcceptedMetadataOnly),
        "accepted_structured_claims" => Ok(SourceReviewDecision::AcceptedStructuredClaims),
        "accepted_package_boundary" => Ok(SourceReviewDecision::AcceptedPackageBoundary),
        "blocked_rights" => Ok(SourceReviewDecision::BlockedRights),
        "blocked_quality" => Ok(SourceReviewDecision::BlockedQuality),
        "blocked_scope" => Ok(SourceReviewDecision::BlockedScope),
        _ => Err(format!("invalid review_decision {value}")),
    }
}

fn requires_attribution(allowed_use: AllowedUse) -> bool {
    matches!(
        allowed_use,
        AllowedUse::StructuredClaims | AllowedUse::Geometry | AllowedUse::TextExcerpt
    )
}

pub fn source_policy_catalog() -> SourceCatalog {
    let mut catalog = SourceCatalog::new();

    catalog.add_record(SourceRecord::metadata_only(
        "src-wikidata-licensing",
        SourceKind::Wikidata,
        "https://www.wikidata.org/wiki/Wikidata:Licensing",
        "CC0 for structured data; CC BY-SA 4.0 for text outside data namespaces",
        "2026-06-19",
    ));
    catalog.add_record(SourceRecord::metadata_only(
        "src-openhistoricalmap-copyright",
        SourceKind::OpenHistoricalMap,
        "https://www.openhistoricalmap.org/copyright",
        "CC0 unless noted; some features may carry CC BY or CC BY-SA tags",
        "2026-06-19",
    ));
    catalog.add_record(SourceRecord::metadata_only(
        "src-wikimedia-terms",
        SourceKind::WikimediaText,
        "https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use",
        "Free/open licenses; project text commonly requires attribution/share-alike",
        "2026-06-19",
    ));

    for source_id in [
        "src-wikidata-licensing",
        "src-openhistoricalmap-copyright",
        "src-wikimedia-terms",
    ] {
        catalog.add_review(SourceReview {
            source_id: source_id.to_string(),
            decision: SourceReviewDecision::AcceptedMetadataOnly,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "Accepted as a source-policy pointer only; no historical facts imported."
                .to_string(),
        });
    }

    catalog
}

fn span_overlaps(span: &YearSpan, start: Year, end: Year) -> bool {
    span.start <= end && span.end.map_or(true, |span_end| span_end >= start)
}

impl TitlePathStep {
    fn from_title(title: &Title) -> Self {
        Self {
            title_id: title.id.clone(),
            name: title.name.clone(),
            rank: title.rank,
        }
    }
}

pub fn seed_timeline() -> TitleTimeline {
    let mut timeline = TitleTimeline::new();

    timeline.add_title(Title {
        id: "k_burgundy_seed".to_string(),
        name: "Seed Kingdom of Burgundy".to_string(),
        rank: TitleRank::Kingdom,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: None,
    });
    timeline.add_title(Title {
        id: "d_alpine_seed".to_string(),
        name: "Seed Duchy of the Alpine March".to_string(),
        rank: TitleRank::Duchy,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("k_burgundy_seed".to_string()),
    });
    timeline.add_title(Title {
        id: "d_river_seed".to_string(),
        name: "Seed Duchy of the River Gate".to_string(),
        rank: TitleRank::Duchy,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("k_burgundy_seed".to_string()),
    });
    timeline.add_title(Title {
        id: "d_highland_seed".to_string(),
        name: "Seed Duchy of the Highland Road".to_string(),
        rank: TitleRank::Duchy,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("k_burgundy_seed".to_string()),
    });
    timeline.add_title(Title {
        id: "c_lake_seed".to_string(),
        name: "Seed County of the Lake Road".to_string(),
        rank: TitleRank::County,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("d_alpine_seed".to_string()),
    });
    timeline.add_title(Title {
        id: "c_bridge_seed".to_string(),
        name: "Seed County of the Bridge Ford".to_string(),
        rank: TitleRank::County,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("d_alpine_seed".to_string()),
    });
    timeline.add_title(Title {
        id: "c_ford_seed".to_string(),
        name: "Seed County of the Old Ford".to_string(),
        rank: TitleRank::County,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("d_alpine_seed".to_string()),
    });

    timeline.add_area(Area {
        id: "area_lake_road".to_string(),
        name: "Lake Road Area".to_string(),
        exists: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_area(Area {
        id: "area_bridge_ford".to_string(),
        name: "Bridge Ford Area".to_string(),
        exists: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_area(Area {
        id: "area_old_ford".to_string(),
        name: "Old Ford Area".to_string(),
        exists: YearSpan::new(1000, Some(1100)),
    });

    timeline.add_area_title(AreaTitleSpan {
        area_id: "area_lake_road".to_string(),
        title_id: "c_lake_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_area_title(AreaTitleSpan {
        area_id: "area_bridge_ford".to_string(),
        title_id: "c_bridge_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_area_title(AreaTitleSpan {
        area_id: "area_old_ford".to_string(),
        title_id: "c_ford_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });

    timeline.add_parentage(ParentageSpan {
        child_title_id: "d_alpine_seed".to_string(),
        parent_title_id: "k_burgundy_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "d_river_seed".to_string(),
        parent_title_id: "k_burgundy_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "d_highland_seed".to_string(),
        parent_title_id: "k_burgundy_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_lake_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_bridge_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1049)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_bridge_seed".to_string(),
        parent_title_id: "d_river_seed".to_string(),
        span: YearSpan::new(1050, Some(1100)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1024)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_river_seed".to_string(),
        span: YearSpan::new(1025, Some(1074)),
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_highland_seed".to_string(),
        span: YearSpan::new(1075, Some(1100)),
    });

    timeline.add_control(ControlSpan {
        title_id: "c_lake_seed".to_string(),
        holder: "House Alder".to_string(),
        span: YearSpan::new(1000, Some(1049)),
    });
    timeline.add_control(ControlSpan {
        title_id: "c_lake_seed".to_string(),
        holder: "House Rowan".to_string(),
        span: YearSpan::new(1050, Some(1100)),
    });
    timeline.add_event(ContinuityEvent {
        year: 1000,
        title_id: "c_lake_seed".to_string(),
        kind: ContinuityEventKind::Created,
        note: "Seed fixture title creation.".to_string(),
    });
    timeline.add_event(ContinuityEvent {
        year: 1050,
        title_id: "c_lake_seed".to_string(),
        kind: ContinuityEventKind::Inherited,
        note: "Seed fixture holder transition.".to_string(),
    });
    timeline.add_event(ContinuityEvent {
        year: 1050,
        title_id: "c_bridge_seed".to_string(),
        kind: ContinuityEventKind::Conquered,
        note: "Seed fixture single duchy transfer.".to_string(),
    });
    timeline.add_event(ContinuityEvent {
        year: 1025,
        title_id: "c_ford_seed".to_string(),
        kind: ContinuityEventKind::Conquered,
        note: "Seed fixture first duchy transfer.".to_string(),
    });
    timeline.add_event(ContinuityEvent {
        year: 1075,
        title_id: "c_ford_seed".to_string(),
        kind: ContinuityEventKind::Partitioned,
        note: "Seed fixture second duchy transfer.".to_string(),
    });

    timeline
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_timeline_validates() {
        let timeline = seed_timeline();
        assert_eq!(timeline.validate(), Ok(()));
    }

    #[test]
    fn yearly_snapshot_filters_titles_and_control() {
        let timeline = seed_timeline();
        let titles = timeline.titles_in_year(1050);
        assert_eq!(titles.len(), 7);
        assert_eq!(
            timeline.holder_in_year("c_lake_seed", 1040),
            Some("House Alder")
        );
        assert_eq!(
            timeline.holder_in_year("c_lake_seed", 1050),
            Some("House Rowan")
        );
        assert_eq!(timeline.holder_in_year("c_lake_seed", 1200), None);
    }

    #[test]
    fn area_identity_links_to_title_over_time() {
        let timeline = seed_timeline();
        let title = timeline
            .title_for_area_in_year("area_bridge_ford", 1050)
            .expect("area should resolve to a county title");

        assert_eq!(title.id, "c_bridge_seed");
    }

    #[test]
    fn temporal_parentage_covers_no_single_and_multi_transfer_fixtures() {
        let timeline = seed_timeline();

        assert_eq!(
            timeline
                .parent_title_in_year("c_lake_seed", 1040)
                .map(|title| title.id.as_str()),
            Some("d_alpine_seed")
        );
        assert_eq!(
            timeline
                .parent_title_in_year("c_lake_seed", 1080)
                .map(|title| title.id.as_str()),
            Some("d_alpine_seed")
        );

        assert_eq!(
            timeline
                .parent_title_in_year("c_bridge_seed", 1049)
                .map(|title| title.id.as_str()),
            Some("d_alpine_seed")
        );
        assert_eq!(
            timeline
                .parent_title_in_year("c_bridge_seed", 1050)
                .map(|title| title.id.as_str()),
            Some("d_river_seed")
        );

        assert_eq!(
            timeline
                .parent_title_in_year("c_ford_seed", 1024)
                .map(|title| title.id.as_str()),
            Some("d_alpine_seed")
        );
        assert_eq!(
            timeline
                .parent_title_in_year("c_ford_seed", 1025)
                .map(|title| title.id.as_str()),
            Some("d_river_seed")
        );
        assert_eq!(
            timeline
                .parent_title_in_year("c_ford_seed", 1075)
                .map(|title| title.id.as_str()),
            Some("d_highland_seed")
        );
    }

    #[test]
    fn title_path_answer_resolves_area_hierarchy_for_year() {
        let timeline = seed_timeline();
        let answer = timeline
            .title_path_for_area_in_year("area_bridge_ford", 1050)
            .expect("area should resolve to title path");

        let ids: Vec<&str> = answer
            .titles
            .iter()
            .map(|step| step.title_id.as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["c_bridge_seed", "d_river_seed", "k_burgundy_seed"]
        );
        assert_eq!(answer.subject_id, "area_bridge_ford");
        assert_eq!(answer.year, 1050);
        assert!(answer
            .events
            .iter()
            .any(|event| event.kind == ContinuityEventKind::Conquered));
    }

    #[test]
    fn title_path_answer_respects_temporal_parentage() {
        let timeline = seed_timeline();

        let before = timeline
            .title_path_for_title_in_year("c_ford_seed", 1024)
            .expect("title should have early path");
        let during = timeline
            .title_path_for_title_in_year("c_ford_seed", 1025)
            .expect("title should have middle path");
        let after = timeline
            .title_path_for_title_in_year("c_ford_seed", 1075)
            .expect("title should have late path");

        assert_eq!(before.titles[1].title_id, "d_alpine_seed");
        assert_eq!(during.titles[1].title_id, "d_river_seed");
        assert_eq!(after.titles[1].title_id, "d_highland_seed");
    }

    #[test]
    fn title_path_answer_is_empty_for_missing_year_or_area() {
        let timeline = seed_timeline();

        assert!(timeline
            .title_path_for_area_in_year("area_bridge_ford", 1200)
            .is_none());
        assert!(timeline
            .title_path_for_area_in_year("missing_area", 1050)
            .is_none());
    }

    #[test]
    fn transfer_query_returns_no_transfer_case() {
        let timeline = seed_timeline();
        let answer = timeline
            .transfers_for_area_between("area_lake_road", TitleRank::Duchy, 1000, 1100)
            .expect("area should resolve to transfer answer");

        assert_eq!(answer.subject_id, "area_lake_road");
        assert_eq!(answer.transfers, Vec::new());
    }

    #[test]
    fn transfer_query_returns_single_duchy_transfer() {
        let timeline = seed_timeline();
        let answer = timeline
            .transfers_for_area_between("area_bridge_ford", TitleRank::Duchy, 1000, 1100)
            .expect("area should resolve to transfer answer");

        assert_eq!(answer.transfers.len(), 1);
        assert_eq!(answer.transfers[0].year, 1050);
        assert_eq!(answer.transfers[0].from_title_id, "d_alpine_seed");
        assert_eq!(answer.transfers[0].to_title_id, "d_river_seed");
        assert_eq!(
            answer.transfers[0].event_kind,
            Some(ContinuityEventKind::Conquered)
        );
    }

    #[test]
    fn transfer_query_returns_multiple_duchy_transfers_in_order() {
        let timeline = seed_timeline();
        let answer = timeline
            .transfers_for_area_between("area_old_ford", TitleRank::Duchy, 1000, 1100)
            .expect("area should resolve to transfer answer");

        let years: Vec<Year> = answer
            .transfers
            .iter()
            .map(|transfer| transfer.year)
            .collect();
        let to_titles: Vec<&str> = answer
            .transfers
            .iter()
            .map(|transfer| transfer.to_title_id.as_str())
            .collect();
        assert_eq!(years, vec![1025, 1075]);
        assert_eq!(to_titles, vec!["d_river_seed", "d_highland_seed"]);
    }

    #[test]
    fn transfer_query_respects_requested_range() {
        let timeline = seed_timeline();
        let answer = timeline
            .transfers_for_title_between("c_ford_seed", TitleRank::Duchy, 1030, 1100)
            .expect("title should resolve to transfer answer");

        assert_eq!(answer.transfers.len(), 1);
        assert_eq!(answer.transfers[0].year, 1075);
    }

    #[test]
    fn title_path_query_reports_answered_seed_trace() {
        let timeline = seed_timeline();
        let envelope = timeline.title_path_query_for_area_in_year("area_bridge_ford", 1050);

        assert_eq!(envelope.status, QueryStatus::Answered);
        assert_eq!(envelope.source_class, SourceClass::Seed);
        assert_eq!(envelope.trace[0].code, "seed_title_path");
        assert!(envelope.answer.is_some());
    }

    #[test]
    fn title_path_query_distinguishes_unknown_and_empty() {
        let timeline = seed_timeline();
        let missing = timeline.title_path_query_for_area_in_year("missing_area", 1050);
        let out_of_range = timeline.title_path_query_for_area_in_year("area_bridge_ford", 1200);

        assert_eq!(missing.status, QueryStatus::Unknown);
        assert_eq!(missing.trace[0].code, "area_missing");
        assert_eq!(out_of_range.status, QueryStatus::Empty);
        assert_eq!(out_of_range.trace[0].code, "no_area_title_span");
    }

    #[test]
    fn transfer_query_reports_empty_and_answered_statuses() {
        let timeline = seed_timeline();
        let no_transfer = timeline.transfers_query_for_area_between(
            "area_lake_road",
            TitleRank::Duchy,
            1000,
            1100,
        );
        let transfer = timeline.transfers_query_for_area_between(
            "area_old_ford",
            TitleRank::Duchy,
            1000,
            1100,
        );

        assert_eq!(no_transfer.status, QueryStatus::Empty);
        assert_eq!(no_transfer.trace[0].code, "no_transfers");
        assert!(no_transfer.answer.is_some());
        assert_eq!(transfer.status, QueryStatus::Answered);
        assert_eq!(transfer.trace[0].code, "seed_transfer_query");
        assert_eq!(transfer.answer.expect("answer").transfers.len(), 2);
    }

    #[test]
    fn transfer_query_reports_unsupported_cases() {
        let timeline = seed_timeline();
        let invalid_range = timeline.transfers_query_for_area_between(
            "area_old_ford",
            TitleRank::Duchy,
            1100,
            1000,
        );
        let unsupported_rank = timeline.transfers_query_for_area_between(
            "area_old_ford",
            TitleRank::Kingdom,
            1000,
            1100,
        );

        assert_eq!(invalid_range.status, QueryStatus::Unsupported);
        assert_eq!(invalid_range.trace[0].code, "invalid_range");
        assert_eq!(unsupported_rank.status, QueryStatus::Unsupported);
        assert_eq!(unsupported_rank.trace[0].code, "unsupported_transfer_rank");
    }

    #[test]
    fn source_policy_catalog_validates_metadata_only_records() {
        let catalog = source_policy_catalog();

        assert_eq!(catalog.validate(), Ok(()));
        assert_eq!(
            catalog
                .record("src-wikidata-licensing")
                .map(|record| record.allowed_use),
            Some(AllowedUse::MetadataOnly)
        );
        assert_eq!(
            catalog
                .latest_review("src-wikidata-licensing")
                .map(|review| review.decision),
            Some(SourceReviewDecision::AcceptedMetadataOnly)
        );
    }

    #[test]
    fn source_catalog_rejects_missing_required_metadata() {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord::metadata_only(
            "bad-source",
            SourceKind::Other,
            "",
            "",
            "",
        ));

        let errors = catalog
            .validate()
            .expect_err("missing source metadata should fail");
        assert!(errors
            .iter()
            .any(|error| error.contains("source_url must not be empty")));
        assert!(errors
            .iter()
            .any(|error| error.contains("license must not be empty")));
        assert!(errors
            .iter()
            .any(|error| error.contains("retrieved_on must not be empty")));
    }

    #[test]
    fn source_catalog_rejects_orphan_reviews() {
        let mut catalog = SourceCatalog::new();
        catalog.add_review(SourceReview {
            source_id: "missing-source".to_string(),
            decision: SourceReviewDecision::AcceptedMetadataOnly,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "No matching source record.".to_string(),
        });

        let errors = catalog.validate().expect_err("orphan review should fail");
        assert!(errors
            .iter()
            .any(|error| error.contains("review references missing source")));
    }

    #[test]
    fn source_catalog_parses_metadata_text_records() {
        let text = r#"
source_id: src-example
source_kind: wikidata
source_url: https://www.wikidata.org/wiki/Wikidata:Licensing
license: CC0 structured data
retrieved_on: 2026-06-19
allowed_use: metadata_only
review_decision: accepted_metadata_only
reviewer: Source Custody Reviewer
review_note: Policy pointer only.
"#;

        let catalog = SourceCatalog::from_metadata_text(text).expect("metadata text should parse");

        assert_eq!(
            catalog
                .record("src-example")
                .map(|record| record.source_kind),
            Some(SourceKind::Wikidata)
        );
        assert_eq!(
            catalog
                .latest_review("src-example")
                .map(|review| review.decision),
            Some(SourceReviewDecision::AcceptedMetadataOnly)
        );
    }

    #[test]
    fn source_catalog_parses_multiple_metadata_text_records() {
        let text = r#"
source_id: src-one
source_kind: wikidata
source_url: https://www.wikidata.org/wiki/Wikidata:Licensing
license: CC0 structured data
retrieved_on: 2026-06-19
allowed_use: metadata_only
---
source_id: src-two
source_kind: other
source_url: urn:example:blocked
license: blocked
retrieved_on: 2026-06-19
allowed_use: blocked
"#;

        let catalog = SourceCatalog::from_metadata_text(text).expect("metadata text should parse");

        assert!(catalog.record("src-one").is_some());
        assert!(catalog.record("src-two").is_some());
    }

    #[test]
    fn source_catalog_rejects_invalid_metadata_text() {
        let text = r#"
source_id: src-bad
source_kind: unknown_kind
source_url: https://example.invalid
license: example
retrieved_on: 2026-06-19
allowed_use: no_such_use
"#;

        let errors = SourceCatalog::from_metadata_text(text)
            .expect_err("invalid source metadata should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("invalid source_kind")));
        assert!(errors
            .iter()
            .any(|error| error.contains("invalid allowed_use")));
    }

    #[test]
    fn source_catalog_accepts_structured_claim_fact_with_review() {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord {
            source_id: "src-structured".to_string(),
            source_kind: SourceKind::Wikidata,
            source_url: "https://www.wikidata.org/wiki/Q31".to_string(),
            license: "CC0 structured data".to_string(),
            retrieved_on: "2026-06-19".to_string(),
            allowed_use: AllowedUse::StructuredClaims,
            attribution: Some("Wikidata contributors".to_string()),
            notes: None,
        });
        catalog.add_review(SourceReview {
            source_id: "src-structured".to_string(),
            decision: SourceReviewDecision::AcceptedStructuredClaims,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "Structured claims accepted for fact-gate test.".to_string(),
        });
        let fact = FactRecord {
            fact_id: "fact-title-exists".to_string(),
            subject_id: "c_example".to_string(),
            claim_kind: ClaimKind::TitleExists,
            span: Some(YearSpan::new(1000, Some(1100))),
            value: "exists".to_string(),
            source_ids: vec!["src-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
        };

        assert_eq!(catalog.validate(), Ok(()));
        assert_eq!(catalog.validate_fact(&fact), Ok(()));
    }

    #[test]
    fn source_catalog_rejects_metadata_only_source_for_fact() {
        let catalog = source_policy_catalog();
        let fact = FactRecord {
            fact_id: "fact-bad".to_string(),
            subject_id: "c_example".to_string(),
            claim_kind: ClaimKind::TitleExists,
            span: None,
            value: "exists".to_string(),
            source_ids: vec!["src-wikidata-licensing".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
        };

        let errors = catalog
            .validate_fact(&fact)
            .expect_err("metadata-only source cannot support facts");
        assert!(errors
            .iter()
            .any(|error| error.contains("not allowed for fact claims")));
        assert!(errors
            .iter()
            .any(|error| error.contains("does not allow fact claims")));
    }

    #[test]
    fn source_catalog_rejects_bad_fact_confidence_counts() {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord {
            source_id: "src-structured".to_string(),
            source_kind: SourceKind::Wikidata,
            source_url: "https://www.wikidata.org/wiki/Q31".to_string(),
            license: "CC0 structured data".to_string(),
            retrieved_on: "2026-06-19".to_string(),
            allowed_use: AllowedUse::StructuredClaims,
            attribution: Some("Wikidata contributors".to_string()),
            notes: None,
        });
        catalog.add_review(SourceReview {
            source_id: "src-structured".to_string(),
            decision: SourceReviewDecision::AcceptedStructuredClaims,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "Structured claims accepted for fact-gate test.".to_string(),
        });
        let fact = FactRecord {
            fact_id: "fact-bad-count".to_string(),
            subject_id: "c_example".to_string(),
            claim_kind: ClaimKind::TitleExists,
            span: None,
            value: "exists".to_string(),
            source_ids: vec!["src-structured".to_string()],
            confidence: ConfidenceLabel::MultiSource,
            conflict_group: None,
        };

        let errors = catalog
            .validate_fact(&fact)
            .expect_err("multi_source needs multiple sources");
        assert!(errors
            .iter()
            .any(|error| error.contains("requires at least two sources")));
    }

    #[test]
    fn source_catalog_requires_conflict_group_for_contested_fact() {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord {
            source_id: "src-structured".to_string(),
            source_kind: SourceKind::Wikidata,
            source_url: "https://www.wikidata.org/wiki/Q31".to_string(),
            license: "CC0 structured data".to_string(),
            retrieved_on: "2026-06-19".to_string(),
            allowed_use: AllowedUse::StructuredClaims,
            attribution: Some("Wikidata contributors".to_string()),
            notes: None,
        });
        catalog.add_review(SourceReview {
            source_id: "src-structured".to_string(),
            decision: SourceReviewDecision::AcceptedStructuredClaims,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "Structured claims accepted for fact-gate test.".to_string(),
        });
        let fact = FactRecord {
            fact_id: "fact-contested".to_string(),
            subject_id: "c_example".to_string(),
            claim_kind: ClaimKind::Parentage,
            span: Some(YearSpan::new(1000, Some(1050))),
            value: "d_example".to_string(),
            source_ids: vec!["src-structured".to_string()],
            confidence: ConfidenceLabel::Contested,
            conflict_group: None,
        };

        let errors = catalog
            .validate_fact(&fact)
            .expect_err("contested fact needs conflict group");
        assert!(errors
            .iter()
            .any(|error| error.contains("requires a conflict_group")));
    }

    #[test]
    fn validation_rejects_wrong_parent_rank() {
        let mut timeline = TitleTimeline::new();
        timeline.add_title(Title {
            id: "c_bad".to_string(),
            name: "Bad County".to_string(),
            rank: TitleRank::County,
            exists: YearSpan::new(1000, None),
            de_jure_parent: Some("k_bad".to_string()),
        });
        timeline.add_title(Title {
            id: "k_bad".to_string(),
            name: "Bad Kingdom".to_string(),
            rank: TitleRank::Kingdom,
            exists: YearSpan::new(1000, None),
            de_jure_parent: None,
        });

        let errors = timeline
            .validate()
            .expect_err("wrong parent rank should fail");
        assert!(errors
            .iter()
            .any(|error| error.contains("expects parent rank Duchy")));
    }

    #[test]
    fn validation_rejects_wrong_temporal_parent_rank() {
        let mut timeline = TitleTimeline::new();
        timeline.add_title(Title {
            id: "c_bad".to_string(),
            name: "Bad County".to_string(),
            rank: TitleRank::County,
            exists: YearSpan::new(1000, None),
            de_jure_parent: None,
        });
        timeline.add_title(Title {
            id: "k_bad".to_string(),
            name: "Bad Kingdom".to_string(),
            rank: TitleRank::Kingdom,
            exists: YearSpan::new(1000, None),
            de_jure_parent: None,
        });
        timeline.add_parentage(ParentageSpan {
            child_title_id: "c_bad".to_string(),
            parent_title_id: "k_bad".to_string(),
            span: YearSpan::new(1000, None),
        });

        let errors = timeline
            .validate()
            .expect_err("wrong temporal parent rank should fail");
        assert!(errors
            .iter()
            .any(|error| error.contains("expects temporal parent rank Duchy")));
    }
}
