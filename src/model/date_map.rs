use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use chrono::NaiveDate;
use timespan::NaiveDateTimeSpan;

#[derive(PartialEq, Debug, Clone)]
pub struct OrderedNaiveDateTimeSpan(pub NaiveDateTimeSpan);

impl Deref for OrderedNaiveDateTimeSpan {
    type Target = NaiveDateTimeSpan;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ord for OrderedNaiveDateTimeSpan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for OrderedNaiveDateTimeSpan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if &self.0.end < &other.0.start {
            Some(Ordering::Less)
        } else if &self.0.start > &other.0.end {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for OrderedNaiveDateTimeSpan {}

pub struct BTreeDateMap<T>(BTreeMap<OrderedNaiveDateTimeSpan, T>);

impl<T> BTreeDateMap<T> {
    pub fn new() -> BTreeDateMap<T> {
        let inner_map = BTreeMap::new();
        BTreeDateMap(inner_map)
    }

    pub fn get_str() -> String {
        "asdf".to_string()
    }
}

impl<T> Deref for BTreeDateMap<T> {
    type Target = BTreeMap<OrderedNaiveDateTimeSpan, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BTreeDateMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {    
    use super::*;
    
    #[test]
    fn BTreeDateMap_WithOrderedNaiveDateTimeSpan_IsOrderedCorrectly() {
        let mut map = BTreeDateMap::<bool>::new();
        let start = OrderedNaiveDateTimeSpan("2017-01-01T12:00:00 - 2017-01-02T18:00:00".parse().unwrap());
        let end = OrderedNaiveDateTimeSpan("2018-01-01T12:00:00 - 2018-01-02T18:00:00".parse().unwrap());
        map.insert(start.clone(), false);
        map.insert(end, true);

        assert_eq!(map.iter().min().unwrap(), (&start, &false))
    }
}
