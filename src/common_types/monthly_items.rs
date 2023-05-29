use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use std::collections::HashMap;

use super::date_time_range::DateTimeRange;

#[derive(Debug, Clone)]
pub struct MonthlyItems<T>(pub(super) HashMap<u32, Vec<T>>);

impl<T> MonthlyItems<T> {
    pub fn new(
        items: Vec<T>,
        naive_date_getter: fn(&T) -> NaiveDate,
        timeframe: DateTimeRange,
    ) -> Self {
        let mut items = items
            .into_iter()
            .into_group_map_by(|it| naive_date_getter(it).month());

        for month in timeframe.months_iter() {
            items.entry(month).or_insert_with(Vec::new);
        }

        MonthlyItems(items)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Vec<T>)> {
        self.0.iter()
    }
}
