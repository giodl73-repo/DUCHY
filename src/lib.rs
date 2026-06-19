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
