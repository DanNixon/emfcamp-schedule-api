use emfcamp_schedule_api::schedule::Schedule;

pub(crate) fn run(schedule: Schedule) {
    for venue in schedule.venues() {
        println!("{venue}");
    }
}
