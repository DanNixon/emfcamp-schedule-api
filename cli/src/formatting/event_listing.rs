use ascii_table::AsciiTable;
use clap::ValueEnum;
use emfcamp_schedule_api::schedule::event::{Event, Kind};
use std::fmt::Display;

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Column {
    Id,
    Slug,
    Type,
    Start,
    StartVerbose,
    End,
    EndVerbose,
    Venue,
    Presenter,
    Title,
    Link,
}

impl Column {
    fn format_event_data(&self, event: &Event) -> String {
        match self {
            Column::Id => event.id.to_string(),
            Column::Slug => event.slug.clone(),
            Column::Type => match event.kind {
                Kind::Talk => "Talk",
                Kind::Workshop(_) => "Workshop",
                Kind::YouthWorkshop => "Youth Workshop",
                Kind::Performance => "Performance",
            }
            .to_string(),
            Column::Start => event.start.format("%a %H:%M").to_string(),
            Column::StartVerbose => event.start.format("%Y-%m-%d %H:%M").to_string(),
            Column::End => event.end.format("%a %H:%M").to_string(),
            Column::EndVerbose => event.end.format("%Y-%m-%d %H:%M").to_string(),
            Column::Venue => event.venue.clone(),
            Column::Presenter => event.speaker.clone(),
            Column::Title => event.title.clone(),
            Column::Link => event.link.to_string(),
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Column::Id => "ID",
            Column::Slug => "Slug",
            Column::Type => "Type",
            Column::Start => "Start",
            Column::StartVerbose => "Start",
            Column::End => "End",
            Column::EndVerbose => "End",
            Column::Venue => "Venue",
            Column::Presenter => "Presenter",
            Column::Title => "Title",
            Column::Link => "Link",
        };
        write!(f, "{text}")
    }
}

pub(crate) fn default_columns() -> Vec<Column> {
    vec![
        Column::Id,
        Column::Start,
        Column::End,
        Column::Venue,
        Column::Type,
        Column::Title,
    ]
}

pub(crate) fn print_table(max_width: usize, columns: &[Column], events: &[Event]) {
    let mut table = AsciiTable::default();

    table.set_max_width(max_width);

    for (i, col) in columns.iter().enumerate() {
        table.column(i).set_header(col.to_string());
    }

    let table_data: Vec<_> = events
        .iter()
        .map(|event| {
            columns
                .iter()
                .map(|col| col.format_event_data(event))
                .collect::<Vec<_>>()
        })
        .collect();

    table.print(table_data);
}
