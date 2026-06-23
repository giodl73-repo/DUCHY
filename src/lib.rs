use std::collections::{HashMap, HashSet};

pub type Year = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleRank {
    County,
    Duchy,
    Province,
    FreeCity,
    Kingdom,
    Crown,
    TheocraticState,
    Empire,
}

impl TitleRank {
    pub fn parent_rank(self) -> Option<Self> {
        match self {
            Self::County => Some(Self::Duchy),
            Self::Duchy => Some(Self::Kingdom),
            Self::Province => Some(Self::Kingdom),
            Self::FreeCity => Some(Self::Empire),
            Self::Kingdom => Some(Self::Crown),
            Self::Crown => Some(Self::Empire),
            Self::TheocraticState => Some(Self::Empire),
            Self::Empire => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub rank_policy: ParentageRankPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationSpan {
    pub subject_title_id: String,
    pub related_title_id: String,
    pub relation_kind: RelationKind,
    pub span: YearSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelationKind {
    ImperialState,
    ConfederationMember,
    FederalStateMember,
    CompositeCrownComponent,
    SplitFiefOrControl,
    VassalageOrSuzerainty,
    SubdivisionOrAppanage,
    RankTransition,
}

impl RelationKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ImperialState => "imperial_state",
            Self::ConfederationMember => "confederation_member",
            Self::FederalStateMember => "federal_state_member",
            Self::CompositeCrownComponent => "composite_crown_component",
            Self::SplitFiefOrControl => "split_fief_or_control",
            Self::VassalageOrSuzerainty => "vassalage_or_suzerainty",
            Self::SubdivisionOrAppanage => "subdivision_or_appanage",
            Self::RankTransition => "rank_transition",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentageRankPolicy {
    StrictImmediate,
    AllowRankSkip,
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

impl SourceClass {
    fn title_path_trace_code(self) -> &'static str {
        match self {
            Self::Seed => "seed_title_path",
            Self::Fictional => "fictional_title_path",
            Self::SourceBacked => "source_backed_title_path",
            Self::Contested => "contested_title_path",
            Self::Unsupported => "unsupported_title_path",
        }
    }
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
    pub supersedes_fact_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestedFactGroup {
    pub conflict_group: String,
    pub facts: Vec<FactRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateRecord {
    pub candidate_id: String,
    pub source_id: String,
    pub source_url: String,
    pub status: CandidateStatus,
    pub review_batch_id: Option<String>,
    pub import_scope: Option<ImportScope>,
    pub rank_basis: Option<RankBasis>,
    pub entity_class: Option<EntityClass>,
    pub source_claims_used: Vec<String>,
    pub confidence_detail: Option<ConfidenceDetail>,
    pub parentage_status: Option<ParentageStatus>,
    pub query_readiness: Option<QueryReadiness>,
    pub exclusion_reason: Option<ExclusionReason>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidateStatus {
    Pending,
    Reviewed,
    Promoted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportScope {
    TitleIdentityOnly,
    ParentageReady,
    TerritoryReady,
    HolderReady,
    ContestedReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RankBasis {
    Literal,
    Normalized,
    Approximate,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityClass {
    County,
    Duchy,
    Kingdom,
    Principality,
    FreeCity,
    TheocraticState,
    Confederation,
    Empire,
    AdministrativeRegion,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfidenceDetail {
    WikidataStructuredSingle,
    WikidataPlusTextCrosscheck,
    MultiSourceAgreement,
    DateConflict,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParentageStatus {
    NoneReviewed,
    CandidateAvailable,
    AcceptedPartial,
    AcceptedFull,
    Contested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryReadiness {
    ExistenceOnly,
    TitlePath,
    Transfer,
    LineageEvent,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExclusionReason {
    UnsupportedRank,
    NonTitlePolity,
    AmbiguousEntity,
    DateConflict,
    SuccessorPredecessorIssue,
    RightsBlocked,
    QualityBlocked,
    ScopeDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimKind {
    TitleExists,
    AreaTitle,
    Parentage,
    Relation,
    Holder,
    Event,
    Name,
    Rank,
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
    relations: Vec<RelationSpan>,
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

    pub fn add_relation(&mut self, relation: RelationSpan) {
        self.relations.push(relation);
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

    pub fn relations_for_title_in_year(
        &self,
        subject_title_id: &str,
        year: Year,
    ) -> Vec<&RelationSpan> {
        let mut relations: Vec<&RelationSpan> = self
            .relations
            .iter()
            .filter(|relation| {
                relation.subject_title_id == subject_title_id && relation.span.contains(year)
            })
            .collect();
        relations.sort_by(|left, right| {
            left.relation_kind
                .cmp(&right.relation_kind)
                .then(left.related_title_id.cmp(&right.related_title_id))
                .then(left.span.start.cmp(&right.span.start))
        });
        relations
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

    pub fn title_path_query_for_title_in_year(
        &self,
        title_id: &str,
        year: Year,
        source_class: SourceClass,
    ) -> QueryEnvelope<TitlePathAnswer> {
        if !self.titles.contains_key(title_id) {
            return QueryEnvelope::without_answer(
                QueryStatus::Unknown,
                source_class,
                "title_missing",
                format!("title {title_id} is not present in the timeline"),
            );
        }

        match self.title_path_for_title_in_year(title_id, year) {
            Some(answer) => {
                let mut envelope = QueryEnvelope::with_answer(
                    answer,
                    source_class,
                    source_class.title_path_trace_code(),
                    format!("resolved {source_class:?} title path for title {title_id} in {year}"),
                );
                self.append_relation_trace_notes(&mut envelope, year);
                envelope
            }
            None => QueryEnvelope::without_answer(
                QueryStatus::Empty,
                source_class,
                "no_title_span",
                format!("title {title_id} has no modeled title path in {year}"),
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

    fn append_relation_trace_notes(
        &self,
        envelope: &mut QueryEnvelope<TitlePathAnswer>,
        year: Year,
    ) {
        let Some(answer) = &envelope.answer else {
            return;
        };
        for step in &answer.titles {
            for relation in self.relations_for_title_in_year(&step.title_id, year) {
                let related_name = self
                    .titles
                    .get(&relation.related_title_id)
                    .map(|title| title.name.as_str())
                    .unwrap_or("unknown title");
                envelope.trace.push(TraceNote::new(
                    "relation_context",
                    format!(
                        "{} has {} relation to {} ({}) over {}",
                        step.title_id,
                        relation.relation_kind.as_str(),
                        relation.related_title_id,
                        related_name,
                        format_year_span(&relation.span)
                    ),
                ));
            }
        }
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
                    match parentage.rank_policy {
                        ParentageRankPolicy::StrictImmediate => match child.rank.parent_rank() {
                            Some(expected_rank) if parent.rank == expected_rank => {}
                            Some(expected_rank) => errors.push(format!(
                                "{} expects temporal parent rank {:?}, found {:?}",
                                child.id, expected_rank, parent.rank
                            )),
                            None => errors.push(format!(
                                "{} cannot have temporal parent {}",
                                child.id, parent.id
                            )),
                        },
                        ParentageRankPolicy::AllowRankSkip => {
                            if parent.rank <= child.rank {
                                errors.push(format!(
                                    "{} expects temporal parent rank above {:?}, found {:?}",
                                    child.id, child.rank, parent.rank
                                ));
                            }
                        }
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

        for relation in &self.relations {
            match (
                self.titles.get(&relation.subject_title_id),
                self.titles.get(&relation.related_title_id),
            ) {
                (Some(subject), Some(related)) => {
                    if !relation.span.is_valid() {
                        errors.push(format!(
                            "{} -> {} has an invalid relation span",
                            relation.subject_title_id, relation.related_title_id
                        ));
                    }
                    if relation.span.start < subject.exists.start {
                        errors.push(format!(
                            "{} relation starts before subject title exists",
                            subject.id
                        ));
                    }
                    if relation.span.start < related.exists.start {
                        errors.push(format!(
                            "{} relation starts before related title exists",
                            related.id
                        ));
                    }
                    if let Some(subject_end) = subject.exists.end {
                        if relation.span.end.map_or(true, |end| end > subject_end) {
                            errors.push(format!("{} relation outlives subject title", subject.id));
                        }
                    }
                    if let Some(related_end) = related.exists.end {
                        if relation.span.end.map_or(true, |end| end > related_end) {
                            errors.push(format!("{} relation outlives related title", related.id));
                        }
                    }
                }
                (None, Some(_)) => errors.push(format!(
                    "relation references missing subject title {}",
                    relation.subject_title_id
                )),
                (Some(_), None) => errors.push(format!(
                    "relation references missing related title {}",
                    relation.related_title_id
                )),
                (None, None) => errors.push(format!(
                    "relation references missing subject title {} and related title {}",
                    relation.subject_title_id, relation.related_title_id
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

    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    pub fn review_count(&self) -> usize {
        self.reviews.len()
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
                    if catalog.records.contains_key(&record.source_id) {
                        errors.push(format!(
                            "record {}: duplicate source_id {}",
                            index + 1,
                            record.source_id
                        ));
                    }
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

fn parse_required_claim_kind(
    values: &HashMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> ClaimKind {
    match values.get(key) {
        Some(value) => match parse_claim_kind(value) {
            Some(claim_kind) => claim_kind,
            None => {
                errors.push(format!("invalid claim_kind {value}"));
                ClaimKind::Event
            }
        },
        None => {
            errors.push(format!("missing required field {key}"));
            ClaimKind::Event
        }
    }
}

fn parse_required_confidence(
    values: &HashMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> ConfidenceLabel {
    match values.get(key) {
        Some(value) => match parse_confidence_label(value) {
            Some(confidence) => confidence,
            None => {
                errors.push(format!("invalid confidence {value}"));
                ConfidenceLabel::Unsupported
            }
        },
        None => {
            errors.push(format!("missing required field {key}"));
            ConfidenceLabel::Unsupported
        }
    }
}

fn parse_required_candidate_status(
    values: &HashMap<String, String>,
    key: &str,
    errors: &mut Vec<String>,
) -> CandidateStatus {
    match values.get(key) {
        Some(value) => match parse_candidate_status(value) {
            Some(status) => status,
            None => {
                errors.push(format!("invalid status {value}"));
                CandidateStatus::Rejected
            }
        },
        None => {
            errors.push(format!("missing required field {key}"));
            CandidateStatus::Rejected
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

fn parse_claim_kind(value: &str) -> Option<ClaimKind> {
    match value {
        "title_exists" => Some(ClaimKind::TitleExists),
        "area_title" => Some(ClaimKind::AreaTitle),
        "parentage" => Some(ClaimKind::Parentage),
        "relation" => Some(ClaimKind::Relation),
        "holder" => Some(ClaimKind::Holder),
        "event" => Some(ClaimKind::Event),
        "name" => Some(ClaimKind::Name),
        "rank" => Some(ClaimKind::Rank),
        _ => None,
    }
}

fn parse_relation_kind(value: &str) -> Option<RelationKind> {
    match value {
        "imperial_state" => Some(RelationKind::ImperialState),
        "confederation_member" => Some(RelationKind::ConfederationMember),
        "federal_state_member" => Some(RelationKind::FederalStateMember),
        "composite_crown_component" => Some(RelationKind::CompositeCrownComponent),
        "split_fief_or_control" => Some(RelationKind::SplitFiefOrControl),
        "vassalage_or_suzerainty" => Some(RelationKind::VassalageOrSuzerainty),
        "subdivision_or_appanage" => Some(RelationKind::SubdivisionOrAppanage),
        "rank_transition" => Some(RelationKind::RankTransition),
        _ => None,
    }
}

fn parse_relation_value(value: &str) -> Option<(RelationKind, String)> {
    let (kind, related_title_id) = value.split_once(':')?;
    let relation_kind = parse_relation_kind(kind.trim())?;
    let related_title_id = related_title_id.trim();
    if related_title_id.is_empty() {
        return None;
    }
    Some((relation_kind, related_title_id.to_string()))
}

fn parse_confidence_label(value: &str) -> Option<ConfidenceLabel> {
    match value {
        "seed" => Some(ConfidenceLabel::Seed),
        "metadata_pointer" => Some(ConfidenceLabel::MetadataPointer),
        "single_source" => Some(ConfidenceLabel::SingleSource),
        "multi_source" => Some(ConfidenceLabel::MultiSource),
        "contested" => Some(ConfidenceLabel::Contested),
        "unsupported" => Some(ConfidenceLabel::Unsupported),
        _ => None,
    }
}

fn parse_year_span(value: &str, errors: &mut Vec<String>) -> Option<YearSpan> {
    let Some((start, end)) = value.split_once("..") else {
        errors.push(format!("invalid span {value}; expected start..end"));
        return None;
    };

    let start = match start.trim().parse::<Year>() {
        Ok(start) => start,
        Err(_) => {
            errors.push(format!("invalid span start {}", start.trim()));
            return None;
        }
    };
    let end = match end.trim() {
        "" | "present" | "none" => None,
        value => match value.parse::<Year>() {
            Ok(end) => Some(end),
            Err(_) => {
                errors.push(format!("invalid span end {value}"));
                return None;
            }
        },
    };

    Some(YearSpan::new(start, end))
}

fn format_year_span(span: &YearSpan) -> String {
    let end = span
        .end
        .map(|end| end.to_string())
        .unwrap_or_else(|| "present".to_string());
    format!("{}..{end}", span.start)
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

fn parse_candidate_status(value: &str) -> Option<CandidateStatus> {
    match value {
        "pending" => Some(CandidateStatus::Pending),
        "reviewed" => Some(CandidateStatus::Reviewed),
        "promoted" => Some(CandidateStatus::Promoted),
        "rejected" => Some(CandidateStatus::Rejected),
        _ => None,
    }
}

fn parse_import_scope(value: &str) -> Option<ImportScope> {
    match value {
        "title_identity_only" => Some(ImportScope::TitleIdentityOnly),
        "parentage_ready" => Some(ImportScope::ParentageReady),
        "territory_ready" => Some(ImportScope::TerritoryReady),
        "holder_ready" => Some(ImportScope::HolderReady),
        "contested_review" => Some(ImportScope::ContestedReview),
        _ => None,
    }
}

fn parse_rank_basis(value: &str) -> Option<RankBasis> {
    match value {
        "literal" => Some(RankBasis::Literal),
        "normalized" => Some(RankBasis::Normalized),
        "approximate" => Some(RankBasis::Approximate),
        "unsupported" => Some(RankBasis::Unsupported),
        _ => None,
    }
}

fn parse_entity_class(value: &str) -> Option<EntityClass> {
    match value {
        "county" => Some(EntityClass::County),
        "duchy" => Some(EntityClass::Duchy),
        "kingdom" => Some(EntityClass::Kingdom),
        "principality" => Some(EntityClass::Principality),
        "free_city" => Some(EntityClass::FreeCity),
        "theocratic_state" => Some(EntityClass::TheocraticState),
        "confederation" => Some(EntityClass::Confederation),
        "empire" => Some(EntityClass::Empire),
        "administrative_region" => Some(EntityClass::AdministrativeRegion),
        "other" => Some(EntityClass::Other),
        _ => None,
    }
}

fn parse_confidence_detail(value: &str) -> Option<ConfidenceDetail> {
    match value {
        "wikidata_structured_single" => Some(ConfidenceDetail::WikidataStructuredSingle),
        "wikidata_plus_text_crosscheck" => Some(ConfidenceDetail::WikidataPlusTextCrosscheck),
        "multi_source_agreement" => Some(ConfidenceDetail::MultiSourceAgreement),
        "date_conflict" => Some(ConfidenceDetail::DateConflict),
        "unsupported" => Some(ConfidenceDetail::Unsupported),
        _ => None,
    }
}

fn parse_parentage_status(value: &str) -> Option<ParentageStatus> {
    match value {
        "none_reviewed" => Some(ParentageStatus::NoneReviewed),
        "candidate_available" => Some(ParentageStatus::CandidateAvailable),
        "accepted_partial" => Some(ParentageStatus::AcceptedPartial),
        "accepted_full" => Some(ParentageStatus::AcceptedFull),
        "contested" => Some(ParentageStatus::Contested),
        _ => None,
    }
}

fn parse_query_readiness(value: &str) -> Option<QueryReadiness> {
    match value {
        "existence_only" => Some(QueryReadiness::ExistenceOnly),
        "title_path" => Some(QueryReadiness::TitlePath),
        "transfer" => Some(QueryReadiness::Transfer),
        "lineage_event" => Some(QueryReadiness::LineageEvent),
        "unsupported" => Some(QueryReadiness::Unsupported),
        _ => None,
    }
}

fn parse_exclusion_reason(value: &str) -> Option<ExclusionReason> {
    match value {
        "unsupported_rank" => Some(ExclusionReason::UnsupportedRank),
        "non_title_polity" => Some(ExclusionReason::NonTitlePolity),
        "ambiguous_entity" => Some(ExclusionReason::AmbiguousEntity),
        "date_conflict" => Some(ExclusionReason::DateConflict),
        "successor_predecessor_issue" => Some(ExclusionReason::SuccessorPredecessorIssue),
        "rights_blocked" => Some(ExclusionReason::RightsBlocked),
        "quality_blocked" => Some(ExclusionReason::QualityBlocked),
        "scope_deferred" => Some(ExclusionReason::ScopeDeferred),
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

pub fn first_real_source_catalog() -> SourceCatalog {
    first_real_source_catalog_from_fixture().expect("first-real source fixture should parse")
}

pub fn first_real_source_catalog_from_fixture() -> Result<SourceCatalog, Vec<String>> {
    SourceCatalog::from_metadata_text(include_str!("../fixtures/first-real.sources"))
}

pub fn first_real_fact_records() -> Vec<FactRecord> {
    first_real_fact_records_from_fixture().expect("first-real fact fixture should parse")
}

pub fn fact_records_from_text(input: &str) -> Result<Vec<FactRecord>, Vec<String>> {
    let mut facts = Vec::new();
    let mut errors = Vec::new();

    for (index, block) in input.split("\n---").enumerate() {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }
        match parse_fact_record_block(block) {
            Ok(fact) => facts.push(fact),
            Err(mut block_errors) => {
                for error in block_errors.drain(..) {
                    errors.push(format!("fact {}: {error}", index + 1));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(facts)
    } else {
        Err(errors)
    }
}

pub fn first_real_fact_records_from_fixture() -> Result<Vec<FactRecord>, Vec<String>> {
    fact_records_from_text(include_str!("../fixtures/first-real.facts"))
}

pub fn candidate_records_from_text(input: &str) -> Result<Vec<CandidateRecord>, Vec<String>> {
    let mut candidates = Vec::new();
    let mut errors = Vec::new();

    for (index, block) in input.split("\n---").enumerate() {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }
        match parse_candidate_record_block(block) {
            Ok(candidate) => candidates.push(candidate),
            Err(mut block_errors) => {
                for error in block_errors.drain(..) {
                    errors.push(format!("candidate {}: {error}", index + 1));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(candidates)
    } else {
        Err(errors)
    }
}

pub fn validate_candidate_records(candidates: &[CandidateRecord]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut candidate_ids: HashMap<&str, usize> = HashMap::new();
    let mut source_ids: HashMap<&str, usize> = HashMap::new();

    for (index, candidate) in candidates.iter().enumerate() {
        let position = index + 1;
        if candidate.candidate_id.trim().is_empty() {
            errors.push(format!(
                "candidate {position} candidate_id must not be empty"
            ));
        }
        if candidate.source_id.trim().is_empty() {
            errors.push(format!(
                "{} source_id must not be empty",
                candidate.candidate_id
            ));
        }
        if candidate.source_url.trim().is_empty() {
            errors.push(format!(
                "{} source_url must not be empty",
                candidate.candidate_id
            ));
        }
        if let Some(first_seen) = candidate_ids.insert(candidate.candidate_id.as_str(), position) {
            errors.push(format!(
                "{} duplicates candidate_id first seen at position {}",
                candidate.candidate_id, first_seen
            ));
        }
        if let Some(first_seen) = source_ids.insert(candidate.source_id.as_str(), position) {
            errors.push(format!(
                "{} duplicates source_id {} first seen at position {}",
                candidate.candidate_id, candidate.source_id, first_seen
            ));
        }
        if matches!(
            candidate.status,
            CandidateStatus::Reviewed | CandidateStatus::Promoted
        ) {
            if candidate.review_batch_id.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires review_batch_id",
                    candidate.candidate_id
                ));
            }
            if candidate.import_scope.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires import_scope",
                    candidate.candidate_id
                ));
            }
            if candidate.rank_basis.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires rank_basis",
                    candidate.candidate_id
                ));
            }
            if candidate.entity_class.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires entity_class",
                    candidate.candidate_id
                ));
            }
            if candidate.source_claims_used.is_empty() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires source_claims_used",
                    candidate.candidate_id
                ));
            }
            if candidate.confidence_detail.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires confidence_detail",
                    candidate.candidate_id
                ));
            }
            if candidate.parentage_status.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires parentage_status",
                    candidate.candidate_id
                ));
            }
            if candidate.query_readiness.is_none() {
                errors.push(format!(
                    "{} reviewed/promoted candidate requires query_readiness",
                    candidate.candidate_id
                ));
            }
        }
        if candidate.status == CandidateStatus::Rejected && candidate.exclusion_reason.is_none() {
            errors.push(format!(
                "{} rejected candidate requires exclusion_reason",
                candidate.candidate_id
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_fact_records(
    catalog: &SourceCatalog,
    facts: &[FactRecord],
) -> Result<(), Vec<String>> {
    let mut errors = catalog.validate().err().unwrap_or_default();
    let mut fact_ids: HashMap<&str, usize> = HashMap::new();
    let mut fact_by_id: HashMap<&str, &FactRecord> = HashMap::new();
    let mut superseded_fact_ids: HashSet<&str> = HashSet::new();
    let mut accepted_claims: HashMap<(&str, ClaimKind, Option<YearSpan>), &FactRecord> =
        HashMap::new();

    for fact in facts {
        if let Some(first_seen) = fact_ids.insert(fact.fact_id.as_str(), fact_ids.len() + 1) {
            errors.push(format!(
                "{} duplicates fact_id first seen at position {}",
                fact.fact_id, first_seen
            ));
        }
        fact_by_id.insert(fact.fact_id.as_str(), fact);
        if let Some(superseded_fact_id) = &fact.supersedes_fact_id {
            superseded_fact_ids.insert(superseded_fact_id.as_str());
        }
    }

    for fact in facts {
        if let Err(mut fact_errors) = catalog.validate_fact(fact) {
            errors.append(&mut fact_errors);
        }

        if let Some(superseded_fact_id) = &fact.supersedes_fact_id {
            if superseded_fact_id == &fact.fact_id {
                errors.push(format!("{} cannot supersede itself", fact.fact_id));
            }
            if fact.confidence == ConfidenceLabel::Contested {
                errors.push(format!(
                    "{} contested fact cannot supersede an accepted fact",
                    fact.fact_id
                ));
            }
            match fact_by_id.get(superseded_fact_id.as_str()) {
                Some(superseded) => {
                    if superseded.confidence == ConfidenceLabel::Contested {
                        errors.push(format!(
                            "{} cannot supersede contested fact {}",
                            fact.fact_id, superseded_fact_id
                        ));
                    }
                    let compatible_span = if fact.claim_kind == ClaimKind::Parentage {
                        match (&superseded.span, &fact.span) {
                            (Some(superseded_span), Some(replacement_span)) => {
                                span_contains_span(superseded_span, replacement_span)
                            }
                            _ => superseded.span == fact.span,
                        }
                    } else {
                        superseded.span == fact.span
                    };
                    if superseded.subject_id != fact.subject_id
                        || superseded.claim_kind != fact.claim_kind
                        || !compatible_span
                    {
                        errors.push(format!(
                            "{} supersedes {} but subject, claim_kind, or span are incompatible",
                            fact.fact_id, superseded_fact_id
                        ));
                    }
                }
                None => errors.push(format!(
                    "{} supersedes missing fact {}",
                    fact.fact_id, superseded_fact_id
                )),
            }
        }

        if fact.confidence == ConfidenceLabel::Contested {
            continue;
        }
        if superseded_fact_ids.contains(fact.fact_id.as_str()) {
            continue;
        }

        let claim_key = (fact.subject_id.as_str(), fact.claim_kind, fact.span.clone());
        match accepted_claims.get(&claim_key) {
            Some(existing) if existing.value != fact.value => errors.push(format!(
                "{} conflicts with {} for {:?} on {} over {:?}: {} vs {}",
                fact.fact_id,
                existing.fact_id,
                fact.claim_kind,
                fact.subject_id,
                fact.span,
                fact.value,
                existing.value
            )),
            Some(_) => {}
            None => {
                accepted_claims.insert(claim_key, fact);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse_candidate_record_block(block: &str) -> Result<CandidateRecord, Vec<String>> {
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

    let candidate_id = take_required(&values, "candidate_id", &mut errors);
    let source_id = take_required(&values, "source_id", &mut errors);
    let source_url = take_required(&values, "source_url", &mut errors);
    let status = parse_required_candidate_status(&values, "status", &mut errors);
    let review_batch_id = optional_field(&values, "review_batch_id");
    let import_scope =
        parse_optional_candidate_field(&values, "import_scope", parse_import_scope, &mut errors);
    let rank_basis =
        parse_optional_candidate_field(&values, "rank_basis", parse_rank_basis, &mut errors);
    let entity_class =
        parse_optional_candidate_field(&values, "entity_class", parse_entity_class, &mut errors);
    let source_claims_used = values
        .get("source_claims_used")
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|claim| !claim.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let confidence_detail = parse_optional_candidate_field(
        &values,
        "confidence_detail",
        parse_confidence_detail,
        &mut errors,
    );
    let parentage_status = parse_optional_candidate_field(
        &values,
        "parentage_status",
        parse_parentage_status,
        &mut errors,
    );
    let query_readiness = parse_optional_candidate_field(
        &values,
        "query_readiness",
        parse_query_readiness,
        &mut errors,
    );
    let exclusion_reason = parse_optional_candidate_field(
        &values,
        "exclusion_reason",
        parse_exclusion_reason,
        &mut errors,
    );
    let notes = values
        .get("notes")
        .filter(|value| !value.trim().is_empty())
        .cloned();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(CandidateRecord {
        candidate_id,
        source_id,
        source_url,
        status,
        review_batch_id,
        import_scope,
        rank_basis,
        entity_class,
        source_claims_used,
        confidence_detail,
        parentage_status,
        query_readiness,
        exclusion_reason,
        notes,
    })
}

fn optional_field(values: &HashMap<String, String>, key: &str) -> Option<String> {
    values
        .get(key)
        .filter(|value| !value.trim().is_empty())
        .cloned()
}

fn parse_optional_candidate_field<T>(
    values: &HashMap<String, String>,
    key: &str,
    parser: fn(&str) -> Option<T>,
    errors: &mut Vec<String>,
) -> Option<T> {
    let Some(value) = values.get(key).filter(|value| !value.trim().is_empty()) else {
        return None;
    };
    match parser(value) {
        Some(parsed) => Some(parsed),
        None => {
            errors.push(format!("invalid {key} {value}"));
            None
        }
    }
}

fn parse_fact_record_block(block: &str) -> Result<FactRecord, Vec<String>> {
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

    let fact_id = take_required(&values, "fact_id", &mut errors);
    let subject_id = take_required(&values, "subject_id", &mut errors);
    let claim_kind = parse_required_claim_kind(&values, "claim_kind", &mut errors);
    let span = values
        .get("span")
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| parse_year_span(value, &mut errors));
    let value = take_required(&values, "value", &mut errors);
    let source_ids = values
        .get("source_ids")
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|source_id| !source_id.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if source_ids.is_empty() {
        errors.push("missing required field source_ids".to_string());
    }
    let confidence = parse_required_confidence(&values, "confidence", &mut errors);
    let conflict_group = values
        .get("conflict_group")
        .filter(|value| !value.trim().is_empty())
        .cloned();
    let supersedes_fact_id = values
        .get("supersedes_fact_id")
        .filter(|value| !value.trim().is_empty())
        .cloned();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(FactRecord {
        fact_id,
        subject_id,
        claim_kind,
        span,
        value,
        source_ids,
        confidence,
        conflict_group,
        supersedes_fact_id,
    })
}

pub fn validate_first_real_facts() -> Result<(), Vec<String>> {
    let catalog = first_real_source_catalog();
    let facts = first_real_fact_records();
    validate_fact_records(&catalog, &facts)
}

pub fn contested_fact_groups(facts: &[FactRecord]) -> Vec<ContestedFactGroup> {
    let mut grouped: HashMap<String, Vec<FactRecord>> = HashMap::new();

    for fact in facts
        .iter()
        .filter(|fact| fact.confidence == ConfidenceLabel::Contested)
    {
        if let Some(conflict_group) = &fact.conflict_group {
            grouped
                .entry(conflict_group.clone())
                .or_default()
                .push(fact.clone());
        }
    }

    let mut groups = grouped
        .into_iter()
        .map(|(conflict_group, mut facts)| {
            facts.sort_by(|left, right| left.fact_id.cmp(&right.fact_id));
            ContestedFactGroup {
                conflict_group,
                facts,
            }
        })
        .collect::<Vec<_>>();
    groups.sort_by(|left, right| left.conflict_group.cmp(&right.conflict_group));
    groups
}

fn superseded_fact_ids(facts: &[FactRecord]) -> HashSet<&str> {
    facts
        .iter()
        .filter_map(|fact| fact.supersedes_fact_id.as_deref())
        .collect()
}

pub fn source_backed_titles_from_facts(
    catalog: &SourceCatalog,
    facts: &[FactRecord],
) -> Result<Vec<Title>, Vec<String>> {
    let mut errors = validate_fact_records(catalog, facts)
        .err()
        .unwrap_or_default();
    let superseded_fact_ids = superseded_fact_ids(facts);

    let mut grouped: HashMap<&str, Vec<&FactRecord>> = HashMap::new();
    for fact in facts.iter().filter(|fact| {
        !superseded_fact_ids.contains(fact.fact_id.as_str())
            && matches!(
                fact.claim_kind,
                ClaimKind::Name | ClaimKind::Rank | ClaimKind::TitleExists
            )
    }) {
        grouped.entry(&fact.subject_id).or_default().push(fact);
    }

    let mut titles = Vec::new();
    for (subject_id, subject_facts) in grouped {
        if subject_facts
            .iter()
            .any(|fact| fact.confidence == ConfidenceLabel::Contested)
        {
            errors.push(format!(
                "{subject_id} has contested facts; resolve or route as contested before materialization"
            ));
            continue;
        }

        let name = subject_facts
            .iter()
            .find(|fact| fact.claim_kind == ClaimKind::Name)
            .map(|fact| fact.value.clone());
        let rank = subject_facts
            .iter()
            .find(|fact| fact.claim_kind == ClaimKind::Rank)
            .and_then(|fact| parse_title_rank(&fact.value));
        let exists = subject_facts
            .iter()
            .find(|fact| fact.claim_kind == ClaimKind::TitleExists)
            .and_then(|fact| fact.span.clone());

        match (name, rank, exists) {
            (Some(name), Some(rank), Some(exists)) => titles.push(Title {
                id: subject_id.to_string(),
                name,
                rank,
                exists,
                de_jure_parent: None,
            }),
            (name, rank, exists) => {
                if name.is_none() {
                    errors.push(format!("{subject_id} missing source-backed name fact"));
                }
                if rank.is_none() {
                    errors.push(format!("{subject_id} missing source-backed rank fact"));
                }
                if exists.is_none() {
                    errors.push(format!("{subject_id} missing source-backed existence span"));
                }
            }
        }
    }

    if errors.is_empty() {
        titles.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(titles)
    } else {
        Err(errors)
    }
}

pub fn first_real_titles() -> Result<Vec<Title>, Vec<String>> {
    let catalog = first_real_source_catalog();
    let facts = first_real_fact_records();
    source_backed_titles_from_facts(&catalog, &facts)
}

pub fn first_real_titles_from_fixture() -> Result<Vec<Title>, Vec<String>> {
    let catalog = first_real_source_catalog_from_fixture()?;
    let facts = first_real_fact_records_from_fixture()?;
    source_backed_titles_from_facts(&catalog, &facts)
}

pub fn source_backed_timeline_from_facts(
    catalog: &SourceCatalog,
    facts: &[FactRecord],
) -> Result<TitleTimeline, Vec<String>> {
    let mut timeline = TitleTimeline::new();

    for title in source_backed_titles_from_facts(catalog, facts)? {
        timeline.add_title(title);
    }

    let parentage = source_backed_parentage_from_facts(catalog, facts, &timeline)?;
    for parentage in parentage {
        timeline.add_parentage(parentage);
    }

    let relations = source_backed_relations_from_facts(catalog, facts, &timeline)?;
    for relation in relations {
        timeline.add_relation(relation);
    }

    timeline.validate()?;
    Ok(timeline)
}

pub fn source_backed_parentage_from_facts(
    catalog: &SourceCatalog,
    facts: &[FactRecord],
    timeline: &TitleTimeline,
) -> Result<Vec<ParentageSpan>, Vec<String>> {
    let mut errors = validate_fact_records(catalog, facts)
        .err()
        .unwrap_or_default();
    let mut parentage = Vec::new();
    let superseded_fact_ids = superseded_fact_ids(facts);
    let mut replacement_spans_by_fact_id: HashMap<&str, Vec<YearSpan>> = HashMap::new();
    for fact in facts
        .iter()
        .filter(|fact| fact.claim_kind == ClaimKind::Parentage)
    {
        if let (Some(superseded_fact_id), Some(span)) =
            (fact.supersedes_fact_id.as_deref(), fact.span.clone())
        {
            replacement_spans_by_fact_id
                .entry(superseded_fact_id)
                .or_default()
                .push(span);
        }
    }

    for fact in facts
        .iter()
        .filter(|fact| fact.claim_kind == ClaimKind::Parentage)
    {
        if let Err(mut fact_errors) = catalog.validate_fact(fact) {
            errors.append(&mut fact_errors);
        }
        if fact.confidence == ConfidenceLabel::Contested {
            errors.push(format!(
                "{} is contested; resolve before parentage materialization",
                fact.fact_id
            ));
            continue;
        }

        let Some(span) = fact.span.clone() else {
            errors.push(format!("{} parentage fact requires a span", fact.fact_id));
            continue;
        };
        let child_title_id = fact.subject_id.clone();
        let parent_title_id = fact.value.clone();
        let active_spans = if superseded_fact_ids.contains(fact.fact_id.as_str()) {
            subtract_year_spans(
                &span,
                replacement_spans_by_fact_id
                    .get(fact.fact_id.as_str())
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
            )
        } else {
            vec![span]
        };
        if active_spans.is_empty() {
            continue;
        }

        if !timeline.titles.contains_key(&child_title_id) {
            errors.push(format!(
                "{} parentage child {} is not materialized",
                fact.fact_id, child_title_id
            ));
        }
        if !timeline.titles.contains_key(&parent_title_id) {
            errors.push(format!(
                "{} parentage parent {} is not materialized",
                fact.fact_id, parent_title_id
            ));
        }

        for active_span in active_spans {
            parentage.push(ParentageSpan {
                child_title_id: child_title_id.clone(),
                parent_title_id: parent_title_id.clone(),
                span: active_span,
                rank_policy: ParentageRankPolicy::AllowRankSkip,
            });
        }
    }

    if errors.is_empty() {
        parentage.sort_by(|left, right| {
            left.child_title_id
                .cmp(&right.child_title_id)
                .then(left.span.start.cmp(&right.span.start))
                .then(left.parent_title_id.cmp(&right.parent_title_id))
        });
        Ok(parentage)
    } else {
        Err(errors)
    }
}

pub fn source_backed_relations_from_facts(
    catalog: &SourceCatalog,
    facts: &[FactRecord],
    timeline: &TitleTimeline,
) -> Result<Vec<RelationSpan>, Vec<String>> {
    let mut errors = validate_fact_records(catalog, facts)
        .err()
        .unwrap_or_default();
    let mut relations = Vec::new();

    for fact in facts
        .iter()
        .filter(|fact| fact.claim_kind == ClaimKind::Relation)
    {
        if let Err(mut fact_errors) = catalog.validate_fact(fact) {
            errors.append(&mut fact_errors);
        }
        if fact.confidence == ConfidenceLabel::Contested {
            errors.push(format!(
                "{} is contested; resolve before relation materialization",
                fact.fact_id
            ));
            continue;
        }

        let Some(span) = fact.span.clone() else {
            errors.push(format!("{} relation fact requires a span", fact.fact_id));
            continue;
        };
        let Some((relation_kind, related_title_id)) = parse_relation_value(&fact.value) else {
            errors.push(format!(
                "{} relation fact requires value relation_kind:related_title_id",
                fact.fact_id
            ));
            continue;
        };
        let subject_title_id = fact.subject_id.clone();

        if !timeline.titles.contains_key(&subject_title_id) {
            errors.push(format!(
                "{} relation subject {} is not materialized",
                fact.fact_id, subject_title_id
            ));
        }
        if !timeline.titles.contains_key(&related_title_id) {
            errors.push(format!(
                "{} relation target {} is not materialized",
                fact.fact_id, related_title_id
            ));
        }

        relations.push(RelationSpan {
            subject_title_id,
            related_title_id,
            relation_kind,
            span,
        });
    }

    if errors.is_empty() {
        relations.sort_by(|left, right| {
            left.subject_title_id
                .cmp(&right.subject_title_id)
                .then(left.span.start.cmp(&right.span.start))
                .then(left.relation_kind.cmp(&right.relation_kind))
                .then(left.related_title_id.cmp(&right.related_title_id))
        });
        Ok(relations)
    } else {
        Err(errors)
    }
}

pub fn first_real_timeline() -> Result<TitleTimeline, Vec<String>> {
    let catalog = first_real_source_catalog();
    let facts = first_real_fact_records();
    source_backed_timeline_from_facts(&catalog, &facts)
}

pub fn first_real_timeline_from_fixture() -> Result<TitleTimeline, Vec<String>> {
    let catalog = first_real_source_catalog_from_fixture()?;
    let facts = first_real_fact_records_from_fixture()?;
    source_backed_timeline_from_facts(&catalog, &facts)
}

fn parse_title_rank(value: &str) -> Option<TitleRank> {
    match value.trim().to_ascii_lowercase().as_str() {
        "county" => Some(TitleRank::County),
        "duchy" | "grand duchy" => Some(TitleRank::Duchy),
        "province" => Some(TitleRank::Province),
        "freecity" | "free city" | "free imperial city" | "imperial city" => {
            Some(TitleRank::FreeCity)
        }
        "kingdom" => Some(TitleRank::Kingdom),
        "crown" | "composite crown" | "composite realm" => Some(TitleRank::Crown),
        "theocratic state" | "papal state" | "ecclesiastical state" => {
            Some(TitleRank::TheocraticState)
        }
        "empire" => Some(TitleRank::Empire),
        _ => None,
    }
}

fn span_overlaps(span: &YearSpan, start: Year, end: Year) -> bool {
    span.start <= end && span.end.map_or(true, |span_end| span_end >= start)
}

fn span_contains_span(outer: &YearSpan, inner: &YearSpan) -> bool {
    if outer.start > inner.start {
        return false;
    }
    match (outer.end, inner.end) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(outer_end), Some(inner_end)) => inner_end <= outer_end,
    }
}

fn subtract_year_spans(base: &YearSpan, cuts: &[YearSpan]) -> Vec<YearSpan> {
    let mut contained_cuts = cuts
        .iter()
        .filter(|cut| span_contains_span(base, cut))
        .cloned()
        .collect::<Vec<_>>();
    contained_cuts
        .sort_by(|left, right| left.start.cmp(&right.start).then(left.end.cmp(&right.end)));

    let mut spans = Vec::new();
    let mut next_start = base.start;
    for cut in contained_cuts {
        if cut.start > next_start {
            spans.push(YearSpan::new(next_start, Some(cut.start - 1)));
        }
        match cut.end {
            Some(cut_end) => {
                next_start = next_start.max(cut_end + 1);
            }
            None => return spans,
        }
    }

    match base.end {
        Some(base_end) if next_start <= base_end => {
            spans.push(YearSpan::new(next_start, Some(base_end)));
        }
        None => spans.push(YearSpan::new(next_start, None)),
        _ => {}
    }
    spans
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
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "d_river_seed".to_string(),
        parent_title_id: "k_burgundy_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "d_highland_seed".to_string(),
        parent_title_id: "k_burgundy_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_lake_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1100)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_bridge_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1049)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_bridge_seed".to_string(),
        parent_title_id: "d_river_seed".to_string(),
        span: YearSpan::new(1050, Some(1100)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_alpine_seed".to_string(),
        span: YearSpan::new(1000, Some(1024)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_river_seed".to_string(),
        span: YearSpan::new(1025, Some(1074)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
    });
    timeline.add_parentage(ParentageSpan {
        child_title_id: "c_ford_seed".to_string(),
        parent_title_id: "d_highland_seed".to_string(),
        span: YearSpan::new(1075, Some(1100)),
        rank_policy: ParentageRankPolicy::StrictImmediate,
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
    fn source_catalog_rejects_duplicate_source_ids() {
        let text = r#"
source_id: src-duplicate
source_kind: other
source_url: urn:duchy:test-one
license: test license
retrieved_on: 2026-06-19
allowed_use: structured_claims
attribution: Test fixture
review_decision: accepted_structured_claims
reviewer: Source Custody Reviewer
review_note: First record.
---
source_id: src-duplicate
source_kind: other
source_url: urn:duchy:test-two
license: test license
retrieved_on: 2026-06-19
allowed_use: structured_claims
attribution: Test fixture
review_decision: accepted_structured_claims
reviewer: Source Custody Reviewer
review_note: Duplicate record.
"#;

        let errors =
            SourceCatalog::from_metadata_text(text).expect_err("duplicate source should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("duplicate source_id src-duplicate")));
    }

    #[test]
    fn source_catalog_accepts_structured_claim_fact_with_review() {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord {
            source_id: "src-structured".to_string(),
            source_kind: SourceKind::Wikidata,
            source_url: "urn:duchy:test-structured".to_string(),
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
            supersedes_fact_id: None,
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
            supersedes_fact_id: None,
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
            source_url: "urn:duchy:test-structured".to_string(),
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
            supersedes_fact_id: None,
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
            source_url: "urn:duchy:test-structured".to_string(),
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
            supersedes_fact_id: None,
        };

        let errors = catalog
            .validate_fact(&fact)
            .expect_err("contested fact needs conflict group");
        assert!(errors
            .iter()
            .any(|error| error.contains("requires a conflict_group")));
    }

    #[test]
    fn fact_set_validation_rejects_duplicate_fact_ids() {
        let catalog = test_structured_claim_catalog();
        let facts = vec![
            FactRecord {
                fact_id: "fact-duplicate".to_string(),
                subject_id: "title-example".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Example One".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-duplicate".to_string(),
                subject_id: "title-example-two".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Example Two".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
        ];

        let errors =
            validate_fact_records(&catalog, &facts).expect_err("duplicate fact ids should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("duplicates fact_id")));
    }

    #[test]
    fn fact_set_validation_rejects_conflicting_accepted_claims() {
        let catalog = test_structured_claim_catalog();
        let facts = vec![
            FactRecord {
                fact_id: "fact-name-one".to_string(),
                subject_id: "title-example".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Example One".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-name-two".to_string(),
                subject_id: "title-example".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Example Two".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
        ];

        let errors = validate_fact_records(&catalog, &facts)
            .expect_err("conflicting accepted facts should fail");

        assert!(errors.iter().any(|error| error.contains("conflicts with")));
    }

    #[test]
    fn first_real_wikidata_facts_pass_source_gate() {
        assert_eq!(validate_first_real_facts(), Ok(()));
    }

    #[test]
    fn first_real_fact_records_are_fixture_backed_claims() {
        let facts = first_real_fact_records();

        assert!(!facts.is_empty());
        assert!(facts.iter().all(|fact| !fact.fact_id.trim().is_empty()));
        assert!(facts.iter().all(|fact| !fact.subject_id.trim().is_empty()));
        assert!(facts.iter().all(|fact| !fact.value.trim().is_empty()));
        assert!(facts.iter().all(|fact| !fact.source_ids.is_empty()));
        assert!(facts.iter().any(|fact| fact.claim_kind == ClaimKind::Name));
        assert!(facts.iter().any(|fact| fact.claim_kind == ClaimKind::Rank));
        assert!(facts
            .iter()
            .any(|fact| fact.claim_kind == ClaimKind::TitleExists && fact.span.is_some()));
        assert!(facts
            .iter()
            .any(|fact| fact.claim_kind == ClaimKind::Parentage && fact.span.is_some()));
    }

    #[test]
    fn fact_records_parse_first_real_fixture() {
        let facts =
            first_real_fact_records_from_fixture().expect("first real fixture should parse");

        assert_eq!(facts, first_real_fact_records());
    }

    #[test]
    fn candidate_records_parse_manifest_text() {
        let text = r#"
candidate_id: cand-one
source_id: src-one
source_url: urn:duchy:candidate-one
status: pending
notes: Pending candidate.
---
candidate_id: cand-two
source_id: src-two
source_url: urn:duchy:candidate-two
status: reviewed
review_batch_id: batch-test
import_scope: title_identity_only
rank_basis: normalized
entity_class: duchy
source_claims_used: label, inception, dissolution
confidence_detail: wikidata_structured_single
parentage_status: none_reviewed
query_readiness: existence_only
"#;

        let candidates =
            candidate_records_from_text(text).expect("candidate manifest should parse");

        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].candidate_id, "cand-one");
        assert_eq!(candidates[0].status, CandidateStatus::Pending);
        assert_eq!(candidates[1].status, CandidateStatus::Reviewed);
        assert_eq!(
            candidates[1].import_scope,
            Some(ImportScope::TitleIdentityOnly)
        );
        assert_eq!(candidates[1].rank_basis, Some(RankBasis::Normalized));
        assert_eq!(candidates[1].entity_class, Some(EntityClass::Duchy));
        assert_eq!(
            candidates[1].source_claims_used,
            vec![
                "label".to_string(),
                "inception".to_string(),
                "dissolution".to_string()
            ]
        );
        assert_eq!(
            candidates[1].query_readiness,
            Some(QueryReadiness::ExistenceOnly)
        );
        assert_eq!(validate_candidate_records(&candidates), Ok(()));
    }

    #[test]
    fn candidate_records_reject_invalid_status_and_duplicates() {
        let text = r#"
candidate_id: cand-one
source_id: src-one
source_url: urn:duchy:candidate-one
status: maybe
---
candidate_id: cand-one
source_id: src-one
source_url: urn:duchy:candidate-two
status: pending
"#;

        let parse_errors =
            candidate_records_from_text(text).expect_err("invalid candidate status should fail");
        assert!(parse_errors
            .iter()
            .any(|error| error.contains("invalid status")));

        let candidates = vec![
            CandidateRecord {
                candidate_id: "cand-one".to_string(),
                source_id: "src-one".to_string(),
                source_url: "urn:duchy:candidate-one".to_string(),
                status: CandidateStatus::Pending,
                review_batch_id: None,
                import_scope: None,
                rank_basis: None,
                entity_class: None,
                source_claims_used: Vec::new(),
                confidence_detail: None,
                parentage_status: None,
                query_readiness: None,
                exclusion_reason: None,
                notes: None,
            },
            CandidateRecord {
                candidate_id: "cand-one".to_string(),
                source_id: "src-one".to_string(),
                source_url: "urn:duchy:candidate-two".to_string(),
                status: CandidateStatus::Reviewed,
                review_batch_id: None,
                import_scope: None,
                rank_basis: None,
                entity_class: None,
                source_claims_used: Vec::new(),
                confidence_detail: None,
                parentage_status: None,
                query_readiness: None,
                exclusion_reason: None,
                notes: None,
            },
        ];

        let validation_errors = validate_candidate_records(&candidates)
            .expect_err("duplicate manifest entries should fail");
        assert!(validation_errors
            .iter()
            .any(|error| error.contains("duplicates candidate_id")));
        assert!(validation_errors
            .iter()
            .any(|error| error.contains("duplicates source_id")));
        assert!(validation_errors
            .iter()
            .any(|error| error.contains("requires import_scope")));
    }

    #[test]
    fn source_catalog_parses_first_real_fixture() {
        let catalog = first_real_source_catalog_from_fixture()
            .expect("first real source fixture should parse");
        let facts =
            first_real_fact_records_from_fixture().expect("first real fact fixture should parse");
        let mut source_ids = facts
            .iter()
            .flat_map(|fact| fact.source_ids.iter().map(String::as_str))
            .collect::<Vec<_>>();
        source_ids.sort_unstable();
        source_ids.dedup();

        assert!(!source_ids.is_empty());
        for source_id in source_ids {
            assert!(catalog.record(source_id).is_some());
            assert_eq!(
                catalog
                    .latest_review(source_id)
                    .map(|review| review.decision),
                Some(SourceReviewDecision::AcceptedStructuredClaims)
            );
        }
    }

    #[test]
    fn first_real_fixture_sources_validate_fixture_facts() {
        let catalog = first_real_source_catalog_from_fixture()
            .expect("first real source fixture should parse");
        let facts =
            first_real_fact_records_from_fixture().expect("first real fact fixture should parse");

        for fact in facts {
            assert_eq!(catalog.validate_fact(&fact), Ok(()));
        }
    }

    #[test]
    fn fact_records_reject_invalid_text() {
        let text = r#"
fact_id: fact-bad
subject_id: title-bad
claim_kind: no_such_claim
span: eighteen..nineteen
value: Bad
source_ids: src-test-structured
confidence: maybe
"#;

        let errors = fact_records_from_text(text).expect_err("invalid fact text should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("invalid claim_kind")));
        assert!(errors
            .iter()
            .any(|error| error.contains("invalid span start")));
        assert!(errors
            .iter()
            .any(|error| error.contains("invalid confidence")));
    }

    #[test]
    fn contested_fact_groups_collect_alternative_claims() {
        let facts = vec![
            FactRecord {
                fact_id: "fact-contested-rank-duchy".to_string(),
                subject_id: "title-contested".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Duchy".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::Contested,
                conflict_group: Some("conflict-title-contested-rank".to_string()),
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-contested-rank-kingdom".to_string(),
                subject_id: "title-contested".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::Contested,
                conflict_group: Some("conflict-title-contested-rank".to_string()),
                supersedes_fact_id: None,
            },
        ];

        let groups = contested_fact_groups(&facts);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].conflict_group, "conflict-title-contested-rank");
        assert_eq!(
            groups[0]
                .facts
                .iter()
                .map(|fact| fact.value.as_str())
                .collect::<Vec<_>>(),
            vec!["Duchy", "Kingdom"]
        );
    }

    #[test]
    fn first_real_facts_materialize_a_source_backed_title() {
        let titles = first_real_titles_from_fixture().expect("first real title should materialize");
        let facts =
            first_real_fact_records_from_fixture().expect("first real fact fixture should parse");
        let mut expected_title_ids = facts
            .iter()
            .filter(|fact| fact.claim_kind == ClaimKind::Name)
            .map(|fact| fact.subject_id.as_str())
            .collect::<Vec<_>>();
        expected_title_ids.sort_unstable();
        expected_title_ids.dedup();

        assert_eq!(titles.len(), expected_title_ids.len());
        assert_eq!(
            titles
                .iter()
                .map(|title| title.id.as_str())
                .collect::<Vec<_>>(),
            expected_title_ids
        );
        for title in titles {
            assert!(!title.name.trim().is_empty());
            assert!(title.exists.is_valid());
            assert_eq!(title.de_jure_parent, None);
        }
    }

    #[test]
    fn first_real_timeline_answers_source_backed_title_path() {
        let timeline =
            first_real_timeline_from_fixture().expect("first real timeline should materialize");
        let titles = first_real_titles_from_fixture().expect("first real title should materialize");
        let title = titles.first().expect("fixture should materialize a title");
        let query = timeline.title_path_query_for_title_in_year(
            &title.id,
            title.exists.start,
            SourceClass::SourceBacked,
        );

        assert_eq!(query.status, QueryStatus::Answered);
        assert_eq!(query.source_class, SourceClass::SourceBacked);
        assert_eq!(query.trace[0].code, "source_backed_title_path");
        assert_eq!(
            query
                .answer
                .as_ref()
                .and_then(|answer| answer.titles.first())
                .map(|step| step.title_id.as_str()),
            Some(title.id.as_str())
        );
    }

    #[test]
    fn first_real_timeline_answers_source_backed_parentage_path() {
        let timeline =
            first_real_timeline_from_fixture().expect("first real timeline should materialize");
        let facts =
            first_real_fact_records_from_fixture().expect("first real fact fixture should parse");
        let superseded_fact_ids = superseded_fact_ids(&facts);
        let parentage_facts = facts
            .iter()
            .filter(|fact| {
                fact.claim_kind == ClaimKind::Parentage
                    && !superseded_fact_ids.contains(fact.fact_id.as_str())
            })
            .collect::<Vec<_>>();

        assert!(!parentage_facts.is_empty());
        for parentage_fact in parentage_facts {
            let span = parentage_fact
                .span
                .as_ref()
                .expect("reviewed parentage fact should carry a span");
            let query = timeline.title_path_query_for_title_in_year(
                &parentage_fact.subject_id,
                span.start,
                SourceClass::SourceBacked,
            );
            assert_eq!(query.status, QueryStatus::Answered);
            let path = query
                .answer
                .as_ref()
                .map(|answer| {
                    answer
                        .titles
                        .iter()
                        .map(|step| step.title_id.as_str())
                        .collect::<Vec<_>>()
                })
                .expect("answered query should carry a title path");
            assert!(path.len() >= 2);
            assert_eq!(path[0], parentage_fact.subject_id.as_str());
            assert_eq!(path[1], parentage_fact.value.as_str());
        }
    }

    #[test]
    fn first_real_timeline_keeps_parentage_span_bounded() {
        let timeline =
            first_real_timeline_from_fixture().expect("first real timeline should materialize");
        let facts =
            first_real_fact_records_from_fixture().expect("first real fact fixture should parse");
        let parentage_fact = facts
            .iter()
            .filter(|fact| fact.claim_kind == ClaimKind::Parentage)
            .find(|fact| {
                let Some(span) = &fact.span else {
                    return false;
                };
                timeline
                    .title_path_for_title_in_year(&fact.subject_id, span.start - 1)
                    .is_some()
            })
            .expect("fixture should include a parentage fact with pre-parentage child existence");
        let span = parentage_fact
            .span
            .as_ref()
            .expect("reviewed parentage fact should carry a span");
        let query = timeline.title_path_query_for_title_in_year(
            &parentage_fact.subject_id,
            span.start - 1,
            SourceClass::SourceBacked,
        );

        assert_eq!(query.status, QueryStatus::Answered);
        assert_eq!(
            query.answer.as_ref().map(|answer| {
                answer
                    .titles
                    .iter()
                    .map(|step| step.title_id.as_str())
                    .collect::<Vec<_>>()
            }),
            Some(vec![parentage_fact.subject_id.as_str()])
        );
    }

    #[test]
    fn first_real_timeline_returns_empty_outside_existence_span() {
        let timeline =
            first_real_timeline_from_fixture().expect("first real timeline should materialize");
        let title = first_real_titles_from_fixture()
            .expect("first real title should materialize")
            .into_iter()
            .find(|title| title.exists.start > Year::MIN)
            .expect("fixture should include a bounded-start title");
        let query = timeline.title_path_query_for_title_in_year(
            &title.id,
            title.exists.start - 1,
            SourceClass::SourceBacked,
        );

        assert_eq!(query.status, QueryStatus::Empty);
        assert_eq!(query.source_class, SourceClass::SourceBacked);
        assert_eq!(query.answer, None);
    }

    #[test]
    fn materialization_rejects_incomplete_title_fact_set() {
        let catalog = test_structured_claim_catalog();
        let facts = vec![FactRecord {
            fact_id: "fact-incomplete-name".to_string(),
            subject_id: "title-incomplete".to_string(),
            claim_kind: ClaimKind::Name,
            span: None,
            value: "Incomplete Title".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: None,
        }];

        let errors = source_backed_titles_from_facts(&catalog, &facts).unwrap_err();

        assert!(errors
            .iter()
            .any(|error| error.contains("missing source-backed rank fact")));
        assert!(errors
            .iter()
            .any(|error| error.contains("missing source-backed existence span")));
    }

    #[test]
    fn materialization_rejects_contested_fact_set() {
        let catalog = test_structured_claim_catalog();
        let facts = vec![
            FactRecord {
                fact_id: "fact-contested-name".to_string(),
                subject_id: "title-contested".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Contested Title".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-contested-rank".to_string(),
                subject_id: "title-contested".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Duchy".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::Contested,
                conflict_group: Some("conflict-title-contested-rank".to_string()),
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-contested-exists".to_string(),
                subject_id: "title-contested".to_string(),
                claim_kind: ClaimKind::TitleExists,
                span: Some(YearSpan::new(1815, Some(1918))),
                value: "exists".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
        ];

        let errors = source_backed_titles_from_facts(&catalog, &facts).unwrap_err();

        assert!(errors
            .iter()
            .any(|error| error.contains("has contested facts")));
    }

    #[test]
    fn source_backed_timeline_materializes_reviewed_parentage() {
        let catalog = test_structured_claim_catalog();
        let facts = test_parentage_fact_set();

        let timeline =
            source_backed_timeline_from_facts(&catalog, &facts).expect("timeline should build");
        let answer = timeline
            .title_path_for_title_in_year("title-child-duchy", 1050)
            .expect("child should have parent path");

        assert_eq!(
            answer
                .titles
                .iter()
                .map(|step| step.title_id.as_str())
                .collect::<Vec<_>>(),
            vec!["title-child-duchy", "title-parent-kingdom"]
        );
    }

    #[test]
    fn source_backed_timeline_uses_parentage_replacement_fact() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.extend([
            FactRecord {
                fact_id: "fact-replacement-parent-name".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Test Replacement Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-replacement-parent-rank".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-replacement-parent-exists".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::TitleExists,
                span: Some(YearSpan::new(1000, Some(1100))),
                value: "exists".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-parentage-replacement".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::Parentage,
                span: Some(YearSpan::new(1000, Some(1100))),
                value: "title-replacement-kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: Some("fact-child-parentage".to_string()),
            },
        ]);

        validate_fact_records(&catalog, &facts).expect("replacement fact should validate");
        let timeline =
            source_backed_timeline_from_facts(&catalog, &facts).expect("timeline should build");
        let answer = timeline
            .title_path_for_title_in_year("title-child-duchy", 1050)
            .expect("child should have replacement parent path");

        assert_eq!(
            answer
                .titles
                .iter()
                .map(|step| step.title_id.as_str())
                .collect::<Vec<_>>(),
            vec!["title-child-duchy", "title-replacement-kingdom"]
        );
    }

    #[test]
    fn source_backed_timeline_splits_partial_parentage_replacement_fact() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.extend([
            FactRecord {
                fact_id: "fact-replacement-parent-name".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Test Replacement Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-replacement-parent-rank".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-replacement-parent-exists".to_string(),
                subject_id: "title-replacement-kingdom".to_string(),
                claim_kind: ClaimKind::TitleExists,
                span: Some(YearSpan::new(1025, Some(1075))),
                value: "exists".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-parentage-partial-replacement".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::Parentage,
                span: Some(YearSpan::new(1025, Some(1075))),
                value: "title-replacement-kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: Some("fact-child-parentage".to_string()),
            },
        ]);

        validate_fact_records(&catalog, &facts).expect("partial replacement fact should validate");
        let timeline =
            source_backed_timeline_from_facts(&catalog, &facts).expect("timeline should build");

        let before = timeline
            .parent_title_in_year("title-child-duchy", 1024)
            .expect("original parent remains before replacement");
        let during = timeline
            .parent_title_in_year("title-child-duchy", 1050)
            .expect("replacement parent applies during replacement span");
        let after = timeline
            .parent_title_in_year("title-child-duchy", 1076)
            .expect("original parent remains after replacement");

        assert_eq!(before.id, "title-parent-kingdom");
        assert_eq!(during.id, "title-replacement-kingdom");
        assert_eq!(after.id, "title-parent-kingdom");
    }

    #[test]
    fn fact_set_validation_rejects_unrelated_replacement_fact() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.push(FactRecord {
            fact_id: "fact-child-parentage-unrelated-replacement".to_string(),
            subject_id: "title-parent-kingdom".to_string(),
            claim_kind: ClaimKind::Parentage,
            span: Some(YearSpan::new(1000, Some(1100))),
            value: "title-child-duchy".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: Some("fact-child-parentage".to_string()),
        });

        let errors = validate_fact_records(&catalog, &facts)
            .expect_err("unrelated replacement fact should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("subject, claim_kind, or span are incompatible")));
    }

    #[test]
    fn fact_set_validation_rejects_parentage_replacement_outside_superseded_span() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.push(FactRecord {
            fact_id: "fact-child-parentage-outside-replacement".to_string(),
            subject_id: "title-child-duchy".to_string(),
            claim_kind: ClaimKind::Parentage,
            span: Some(YearSpan::new(900, Some(950))),
            value: "title-parent-kingdom".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: Some("fact-child-parentage".to_string()),
        });

        let errors = validate_fact_records(&catalog, &facts)
            .expect_err("replacement outside superseded span should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("subject, claim_kind, or span are incompatible")));
    }

    #[test]
    fn source_backed_parentage_requires_span() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts
            .iter_mut()
            .find(|fact| fact.claim_kind == ClaimKind::Parentage)
            .expect("fixture should have parentage")
            .span = None;

        let errors = source_backed_timeline_from_facts(&catalog, &facts)
            .expect_err("parentage without span should fail");

        assert!(errors
            .iter()
            .any(|error| error.contains("parentage fact requires a span")));
    }

    #[test]
    fn source_backed_parentage_requires_materialized_parent() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.retain(|fact| fact.subject_id != "title-parent-kingdom");

        let errors = source_backed_timeline_from_facts(&catalog, &facts)
            .expect_err("missing parent should fail");

        assert!(errors.iter().any(
            |error| error.contains("parentage parent title-parent-kingdom is not materialized")
        ));
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
            rank_policy: ParentageRankPolicy::StrictImmediate,
        });

        let errors = timeline
            .validate()
            .expect_err("wrong temporal parent rank should fail");
        assert!(errors
            .iter()
            .any(|error| error.contains("expects temporal parent rank Duchy")));
    }

    #[test]
    fn crown_rank_sits_between_kingdom_and_empire() {
        assert_eq!(TitleRank::Kingdom.parent_rank(), Some(TitleRank::Crown));
        assert_eq!(TitleRank::Crown.parent_rank(), Some(TitleRank::Empire));
        assert!(TitleRank::Kingdom < TitleRank::Crown);
        assert!(TitleRank::Crown < TitleRank::Empire);
    }

    #[test]
    fn province_rank_sits_below_kingdom() {
        assert_eq!(TitleRank::Province.parent_rank(), Some(TitleRank::Kingdom));
        assert_eq!(TitleRank::Duchy.parent_rank(), Some(TitleRank::Kingdom));
        assert!(TitleRank::Duchy < TitleRank::Province);
        assert!(TitleRank::Province < TitleRank::Kingdom);
    }

    #[test]
    fn free_city_rank_sits_below_kingdom_with_empire_parent() {
        assert_eq!(TitleRank::FreeCity.parent_rank(), Some(TitleRank::Empire));
        assert!(TitleRank::Province < TitleRank::FreeCity);
        assert!(TitleRank::FreeCity < TitleRank::Kingdom);
    }

    #[test]
    fn theocratic_state_rank_sits_below_empire() {
        assert_eq!(
            TitleRank::TheocraticState.parent_rank(),
            Some(TitleRank::Empire)
        );
        assert!(TitleRank::Crown < TitleRank::TheocraticState);
        assert!(TitleRank::TheocraticState < TitleRank::Empire);
    }

    fn test_structured_claim_catalog() -> SourceCatalog {
        let mut catalog = SourceCatalog::new();
        catalog.add_record(SourceRecord {
            source_id: "src-test-structured".to_string(),
            source_kind: SourceKind::Other,
            source_url: "urn:duchy:test-structured".to_string(),
            license: "test structured claims".to_string(),
            retrieved_on: "2026-06-19".to_string(),
            allowed_use: AllowedUse::StructuredClaims,
            attribution: Some("DUCHY test fixture".to_string()),
            notes: None,
        });
        catalog.add_review(SourceReview {
            source_id: "src-test-structured".to_string(),
            decision: SourceReviewDecision::AcceptedStructuredClaims,
            reviewer: "Source Custody Reviewer".to_string(),
            note: "Accepted for source-backed parentage materialization tests.".to_string(),
        });
        catalog
    }

    fn test_parentage_fact_set() -> Vec<FactRecord> {
        vec![
            FactRecord {
                fact_id: "fact-parent-name".to_string(),
                subject_id: "title-parent-kingdom".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Test Parent Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-parent-rank".to_string(),
                subject_id: "title-parent-kingdom".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-parent-exists".to_string(),
                subject_id: "title-parent-kingdom".to_string(),
                claim_kind: ClaimKind::TitleExists,
                span: Some(YearSpan::new(1000, Some(1100))),
                value: "exists".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-name".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::Name,
                span: None,
                value: "Test Child Duchy".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-rank".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::Rank,
                span: None,
                value: "Duchy".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-exists".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::TitleExists,
                span: Some(YearSpan::new(1000, Some(1100))),
                value: "exists".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
            FactRecord {
                fact_id: "fact-child-parentage".to_string(),
                subject_id: "title-child-duchy".to_string(),
                claim_kind: ClaimKind::Parentage,
                span: Some(YearSpan::new(1000, Some(1100))),
                value: "title-parent-kingdom".to_string(),
                source_ids: vec!["src-test-structured".to_string()],
                confidence: ConfidenceLabel::SingleSource,
                conflict_group: None,
                supersedes_fact_id: None,
            },
        ]
    }

    #[test]
    fn source_backed_relation_facts_materialize_separately_from_parentage() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.push(FactRecord {
            fact_id: "fact-child-imperial-state".to_string(),
            subject_id: "title-child-duchy".to_string(),
            claim_kind: ClaimKind::Relation,
            span: Some(YearSpan::new(1000, Some(1100))),
            value: "imperial_state:title-parent-kingdom".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: None,
        });

        let timeline = source_backed_timeline_from_facts(&catalog, &facts).unwrap();

        let title_path = timeline
            .title_path_for_title_in_year("title-child-duchy", 1050)
            .unwrap();
        assert_eq!(
            title_path
                .titles
                .iter()
                .map(|step| step.title_id.as_str())
                .collect::<Vec<_>>(),
            vec!["title-child-duchy", "title-parent-kingdom"]
        );

        let relations = timeline.relations_for_title_in_year("title-child-duchy", 1050);
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].relation_kind, RelationKind::ImperialState);
        assert_eq!(relations[0].related_title_id, "title-parent-kingdom");
    }

    #[test]
    fn title_path_query_traces_relation_context_without_mutating_path() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.push(FactRecord {
            fact_id: "fact-child-confederation-member".to_string(),
            subject_id: "title-child-duchy".to_string(),
            claim_kind: ClaimKind::Relation,
            span: Some(YearSpan::new(1000, Some(1100))),
            value: "confederation_member:title-parent-kingdom".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: None,
        });

        let timeline = source_backed_timeline_from_facts(&catalog, &facts).unwrap();
        let query = timeline.title_path_query_for_title_in_year(
            "title-child-duchy",
            1050,
            SourceClass::SourceBacked,
        );

        assert_eq!(query.status, QueryStatus::Answered);
        assert_eq!(
            query
                .answer
                .as_ref()
                .unwrap()
                .titles
                .iter()
                .map(|step| step.title_id.as_str())
                .collect::<Vec<_>>(),
            vec!["title-child-duchy", "title-parent-kingdom"]
        );
        assert!(query.trace.iter().any(|trace| {
            trace.code == "relation_context"
                && trace.detail.contains("confederation_member")
                && trace.detail.contains("title-parent-kingdom")
        }));
    }

    #[test]
    fn source_backed_relation_facts_require_typed_value_and_span() {
        let catalog = test_structured_claim_catalog();
        let mut facts = test_parentage_fact_set();
        facts.push(FactRecord {
            fact_id: "fact-child-bad-relation".to_string(),
            subject_id: "title-child-duchy".to_string(),
            claim_kind: ClaimKind::Relation,
            span: None,
            value: "imperial_state:title-parent-kingdom".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: None,
        });
        facts.push(FactRecord {
            fact_id: "fact-child-unknown-relation".to_string(),
            subject_id: "title-child-duchy".to_string(),
            claim_kind: ClaimKind::Relation,
            span: Some(YearSpan::new(1000, Some(1100))),
            value: "unknown_relation:title-parent-kingdom".to_string(),
            source_ids: vec!["src-test-structured".to_string()],
            confidence: ConfidenceLabel::SingleSource,
            conflict_group: None,
            supersedes_fact_id: None,
        });

        let timeline = source_backed_timeline_from_facts(&catalog, &facts).unwrap_err();

        assert!(timeline
            .iter()
            .any(|error| error.contains("fact-child-bad-relation relation fact requires a span")));
        assert!(timeline.iter().any(|error| error.contains(
            "fact-child-unknown-relation relation fact requires value relation_kind:related_title_id"
        )));
    }
}
