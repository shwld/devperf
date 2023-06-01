use chrono::NaiveDate;
use itertools::Itertools;
use std::collections::HashMap;

use super::date_time_range::DateTimeRange;

#[derive(Debug, Clone)]
pub struct DailyItems<T>(pub(super) HashMap<NaiveDate, Vec<T>>);

impl<T> DailyItems<T> {
    pub fn new(
        items: Vec<T>,
        naive_date_getter: fn(&T) -> NaiveDate,
        timeframe: DateTimeRange,
    ) -> Self {
        let mut items = items.into_iter().into_group_map_by(naive_date_getter);

        for dt in timeframe.days_iter() {
            items.entry(dt.date_naive()).or_insert_with(Vec::new);
        }

        DailyItems(items)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&NaiveDate, &Vec<T>)> {
        self.0.iter()
    }
    pub fn nonempty_days(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0
            .iter()
            .filter(|(_, days)| !days.is_empty())
            .map(|(_, days)| days)
    }
}
