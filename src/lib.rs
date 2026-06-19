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
    titles: HashMap<String, Title>,
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
        id: "c_lake_seed".to_string(),
        name: "Seed County of the Lake Road".to_string(),
        rank: TitleRank::County,
        exists: YearSpan::new(1000, Some(1100)),
        de_jure_parent: Some("d_alpine_seed".to_string()),
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
        assert_eq!(titles.len(), 3);
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
}
