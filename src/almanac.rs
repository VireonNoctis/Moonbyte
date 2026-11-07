//! Seasonal Almanac: published annual chapter windows and a permanent personal archive.
//!
//! This is a read-only projection over the owning Atlas, Inventory and Expedition records.
//! It creates no duplicate milestone rows, seasonal currency or expiring ownership.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeasonalChapter {
    pub id: &'static str,
    pub title: &'static str,
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub window_label: &'static str,
    pub archive_record: &'static str,
    pub evergreen_route: &'static str,
}

pub const SEASONAL_CHAPTERS: [SeasonalChapter; 4] = [
    SeasonalChapter {
        id: "lantern-bloom",
        title: "Lantern Bloom",
        start_month: 3,
        start_day: 1,
        end_month: 3,
        end_day: 21,
        window_label: "March 1–21, every year",
        archive_record: "A first discovery beyond Lantern Square is recorded in your Atlas.",
        evergreen_route: ".atlas",
    },
    SeasonalChapter {
        id: "tideglass-days",
        title: "Tideglass Days",
        start_month: 6,
        start_day: 1,
        end_month: 6,
        end_day: 21,
        window_label: "June 1–21, every year",
        archive_record: "The account-bound Wayfarer's Lantern is present in your Satchel.",
        evergreen_route: ".craft wayfarer-lantern",
    },
    SeasonalChapter {
        id: "starfall-observances",
        title: "Starfall Observances",
        start_month: 9,
        start_day: 1,
        end_month: 9,
        end_day: 21,
        window_label: "September 1–21, every year",
        archive_record: "The one-time Mosslight Seal is present in your Satchel.",
        evergreen_route: ".vault list",
    },
    SeasonalChapter {
        id: "hearthlight-gathering",
        title: "Hearthlight Gathering",
        start_month: 12,
        start_day: 1,
        end_month: 12,
        end_day: 21,
        window_label: "December 1–21, every year",
        archive_record: "The Lantern Road Expedition has a completed run in your history.",
        evergreen_route: ".expedition",
    },
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArchiveEvidence {
    pub atlas_discovery: bool,
    pub wayfarer_lantern: bool,
    pub mosslight_seal: bool,
    pub completed_lantern_road: bool,
}

impl ArchiveEvidence {
    fn contains(self, chapter_id: &str) -> bool {
        match chapter_id {
            "lantern-bloom" => self.atlas_discovery,
            "tideglass-days" => self.wayfarer_lantern,
            "starfall-observances" => self.mosslight_seal,
            "hearthlight-gathering" => self.completed_lantern_road,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UtcDate {
    year: i32,
    month: u8,
    day: u8,
}

impl UtcDate {
    fn parse(value: &str) -> Option<Self> {
        let mut parts = value.split('-');
        let year = parts.next()?.parse::<i32>().ok()?;
        let month = parts.next()?.parse::<u8>().ok()?;
        let day = parts.next()?.parse::<u8>().ok()?;
        if parts.next().is_some() || year < 1 || !(1..=12).contains(&month) {
            return None;
        }

        let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap_year => 29,
            2 => 28,
            _ => return None,
        };
        if day == 0 || day > days_in_month {
            return None;
        }
        Some(Self { year, month, day })
    }

    const fn month_day(self) -> (u8, u8) {
        (self.month, self.day)
    }

    fn format(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn in_published_window(date: UtcDate, chapter: SeasonalChapter) -> bool {
    let current = date.month_day();
    current >= (chapter.start_month, chapter.start_day)
        && current <= (chapter.end_month, chapter.end_day)
}

fn next_reprise(date: UtcDate, chapter: SeasonalChapter) -> String {
    let start = (chapter.start_month, chapter.start_day);
    let year = if date.month_day() <= start {
        date.year
    } else {
        date.year.saturating_add(1)
    };
    format!(
        "{year:04}-{:02}-{:02}",
        chapter.start_month, chapter.start_day
    )
}

/// Render the four annual published windows using a database-supplied UTC date.
pub fn render_calendar(utc_date: &str) -> String {
    let Some(date) = UtcDate::parse(utc_date) else {
        return String::from(
            "**Seasonal Almanac**\n\nThe published UTC calendar is temporarily unavailable. Please try again.",
        );
    };

    let chapters = SEASONAL_CHAPTERS
        .iter()
        .map(|chapter| {
            if in_published_window(date, *chapter) {
                format!(
                    "• **{}** — published chapter window is open\n  Annual window: {}",
                    chapter.title, chapter.window_label
                )
            } else {
                format!(
                    "• **{}** — next published reprise: {}\n  Annual window: {}",
                    chapter.title,
                    next_reprise(date, *chapter),
                    chapter.window_label
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**Seasonal Almanac**\n\nCalendar date (UTC): **{}**\n\n{}\n\nChapter windows repeat on the published annual dates. Missing a window never removes a record or closes its archive route. This first release publishes the calendar and personal archive; seasonal quests, art packs and event-specific story chapters are not yet part of the live build.",
        date.format(),
        chapters
    )
}

/// Render permanent archive proof from source-owned records, never from the current date.
pub fn render_archive(utc_date: &str, evidence: ArchiveEvidence) -> String {
    let date = UtcDate::parse(utc_date)
        .map(UtcDate::format)
        .unwrap_or_else(|| "unavailable".to_owned());

    let entries = SEASONAL_CHAPTERS
        .iter()
        .map(|chapter| {
            if evidence.contains(chapter.id) {
                format!(
                    "✓ **{}** — archived\n  {}\n  Evergreen reference: `{}`",
                    chapter.title, chapter.archive_record, chapter.evergreen_route
                )
            } else {
                format!(
                    "◇ **{}** — not yet recorded\n  {}\n  Evergreen route: `{}`",
                    chapter.title, chapter.archive_record, chapter.evergreen_route
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "**Your Seasonal Archive**\n\nRead-only record checked: **{date} UTC**\n\n{entries}\n\nArchive markers come from your existing Atlas, Satchel and completed Expedition records. They are not duplicate rewards and do not expire. Every route stays available year-round; seasonal window dates never gate the original source activity."
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ArchiveEvidence, SEASONAL_CHAPTERS, UtcDate, in_published_window, next_reprise,
        render_archive, render_calendar,
    };

    #[test]
    fn published_windows_include_both_boundary_days() {
        let lantern = SEASONAL_CHAPTERS[0];
        assert!(in_published_window(
            UtcDate::parse("2026-03-01").unwrap(),
            lantern
        ));
        assert!(in_published_window(
            UtcDate::parse("2026-03-21").unwrap(),
            lantern
        ));
        assert!(!in_published_window(
            UtcDate::parse("2026-03-22").unwrap(),
            lantern
        ));
    }

    #[test]
    fn reprise_dates_are_predictable_and_annual() {
        let lantern = SEASONAL_CHAPTERS[0];
        assert_eq!(
            next_reprise(UtcDate::parse("2026-02-28").unwrap(), lantern),
            "2026-03-01"
        );
        assert_eq!(
            next_reprise(UtcDate::parse("2026-03-22").unwrap(), lantern),
            "2027-03-01"
        );
        assert_eq!(
            next_reprise(UtcDate::parse("2026-12-22").unwrap(), SEASONAL_CHAPTERS[3]),
            "2027-12-01"
        );
    }

    #[test]
    fn calendar_names_every_window_and_discloses_unshipped_content() {
        let calendar = render_calendar("2026-06-10");
        assert!(calendar.contains("Lantern Bloom"));
        assert!(calendar.contains("Tideglass Days"));
        assert!(calendar.contains("published chapter window is open"));
        assert!(calendar.contains("2026-09-01"));
        assert!(calendar.contains(
            "seasonal quests, art packs and event-specific story chapters are not yet part"
        ));
    }

    #[test]
    fn archive_evidence_is_permanent_and_independent_of_event_date() {
        let evidence = ArchiveEvidence {
            atlas_discovery: true,
            wayfarer_lantern: false,
            mosslight_seal: true,
            completed_lantern_road: false,
        };
        let archive = render_archive("2026-12-30", evidence);
        assert!(archive.contains("Lantern Bloom** — archived"));
        assert!(archive.contains("Tideglass Days** — not yet recorded"));
        assert!(archive.contains("Starfall Observances** — archived"));
        assert!(archive.contains("Hearthlight Gathering** — not yet recorded"));
        assert!(archive.contains("Every route stays available year-round"));
        let later_archive = render_archive("2027-06-15", evidence);
        assert_eq!(archive.replace("2026-12-30", "2027-06-15"), later_archive);
    }

    #[test]
    fn invalid_dates_are_reported_without_panicking() {
        assert!(render_calendar("not-a-date").contains("temporarily unavailable"));
        assert!(
            render_archive("2026-02-30", ArchiveEvidence::default()).contains("unavailable UTC")
        );
    }
}
