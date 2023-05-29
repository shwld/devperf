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
