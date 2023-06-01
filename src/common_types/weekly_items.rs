use chrono::{NaiveDate, Weekday};
use itertools::Itertools;
use std::collections::HashMap;

use super::date_time_range::DateTimeRange;

#[derive(Debug, Clone)]
pub struct WeeklyItems<T>(pub(super) HashMap<NaiveDate, Vec<T>>);

impl<T> WeeklyItems<T> {
    pub fn new(
        items: Vec<T>,
        naive_date_getter: fn(&T) -> NaiveDate,
        timeframe: DateTimeRange,
    ) -> Self {
        let mut items = items
            .into_iter()
            .into_group_map_by(|it| naive_date_getter(it).week(Weekday::Mon).first_day());

        for dt in timeframe.weeks_iter() {
            items.entry(dt.date_naive()).or_insert_with(Vec::new);
        }

        WeeklyItems(items)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&NaiveDate, &Vec<T>)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common_types::date_time_range::DateTimeRange, shared::datetime_utc::parse,
        tests::factories::deployment_log::build_deployment_log,
    };

    use super::WeeklyItems;

    #[test]
    fn collect() {
        let timeframe = DateTimeRange::new(
            parse("2023-01-01 00:00:00").expect("Could not parse since"),
            parse("2023-03-31 00:00:00").expect("Could not parse since"),
        )
        .expect("Could not create timeframe");
        let weekly_items = WeeklyItems::new(
            vec![
                build_deployment_log("2023-04-01 10:00:00"),
                build_deployment_log("2023-03-29 10:00:00"),
                build_deployment_log("2023-03-28 17:30:00"),
                build_deployment_log("2023-03-27 15:00:00"),
                build_deployment_log("2023-03-22 10:00:00"),
                build_deployment_log("2023-03-21 10:00:00"),
                build_deployment_log("2023-03-14 10:00:00"),
                build_deployment_log("2023-03-08 10:00:00"),
                build_deployment_log("2023-03-07 10:00:00"),
                build_deployment_log("2023-03-01 10:00:00"),
                build_deployment_log("2023-02-28 10:00:00"),
                build_deployment_log("2023-02-22 10:00:00"),
            ],
            |it| it.deployed_at.date_naive(),
            timeframe,
        );

        assert_eq!(weekly_items.0.len(), 13);
    }
}
