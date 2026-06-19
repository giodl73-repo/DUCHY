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
