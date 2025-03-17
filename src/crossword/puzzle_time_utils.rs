use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;

use serenity::model::Timestamp;
use Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};

const REFERENCE_TIMEZONE: Tz = chrono_tz::America::New_York;

trait CutoffTime {
    fn cutoff_hour(&self) -> u32;
}

impl CutoffTime for Weekday {
    fn cutoff_hour(&self) -> u32 {
        match *self {
            Mon | Tue | Wed | Thu | Fri => 22,
            Sat | Sun => 18,
        }
    }
}

pub fn puzzle_date_from_datetime(timestamp: DateTime<Utc>) -> NaiveDate {
    let ny_datetime = timestamp.with_timezone(&REFERENCE_TIMEZONE);

    if ny_datetime.hour() < ny_datetime.weekday().cutoff_hour() {
        ny_datetime.date_naive()
    } else {
        ny_datetime
            .date_naive()
            .succ_opt()
            .expect("somehow this program survived beyond the time this machine can represent")
    }
}

pub fn puzzle_date_from_timestamp(timestamp: Timestamp) -> NaiveDate {
    let dt = DateTime::from_timestamp(timestamp.unix_timestamp(), 0)
        .expect("Can't parse discord timestamp");
    puzzle_date_from_datetime(dt)
}

pub fn puzzle_period(date: &NaiveDate) -> (DateTime<Tz>, DateTime<Tz>) {
    let prev = date.pred_opt().unwrap();

    let start_local = prev
        .and_hms_opt(prev.weekday().cutoff_hour(), 0, 0)
        .unwrap();
    let end_local = date
        .and_hms_opt(date.weekday().cutoff_hour(), 0, 0)
        .unwrap();

    // The earliest() exists because *technically* when daylight savings happens, a certain local
    // time can exist twice. I don't care though because cutoff times don't happen at that point in
    // the night.
    let start_zoned = REFERENCE_TIMEZONE
        .from_local_datetime(&start_local)
        .earliest()
        .unwrap();
    let end_zoned = REFERENCE_TIMEZONE
        .from_local_datetime(&end_local)
        .earliest()
        .unwrap();

    (start_zoned, end_zoned)
}
