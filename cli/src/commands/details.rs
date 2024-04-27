use clap::Parser;
use emfcamp_schedule_api::schedule::{event::Event, Schedule};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Parser)]
pub(crate) struct EventDetailsOptions {
    #[arg(long, default_value = "auto")]
    color: ColorChoice,

    /// ID of the event to show details of
    event: u32,
}

pub(crate) fn run(args: EventDetailsOptions, schedule: Schedule) {
    let mut stdout = StandardStream::stdout(args.color);

    match schedule.events.iter().find(|event| event.id == args.event) {
        Some(event) => print_verbose_event_details(&mut stdout, event),
        None => println!("Failed to find event with ID {}", args.event),
    }
}

fn print_verbose_event_details(stdout: &mut StandardStream, event: &Event) {
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        .unwrap();
    println!("ID/slug  : {} / {}", event.id, event.slug);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .unwrap();
    println!("Title    : {}", event.title);

    if let Some(pronouns) = &event.pronouns {
        println!("Speaker  : {} ({})", event.speaker, pronouns);
    } else {
        println!("Speaker  : {}", event.speaker);
    }

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    println!("Type     : {}", event.kind);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
        .unwrap();
    println!("Start    : {}", event.start);
    println!("End      : {}", event.end);

    let duration = event.end - event.start;
    println!("Duration : {}m", duration.num_minutes());

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    println!("Venue    : {}", event.venue);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        .unwrap();
    println!("URL      : {}", event.link);

    stdout.set_color(&ColorSpec::default()).unwrap();
    println!();

    println!("{}", event.description);
}
