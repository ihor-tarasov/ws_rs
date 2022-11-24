use std::{cmp::Ordering, collections::BTreeMap, ops::RangeInclusive};

struct MapRange<T>(RangeInclusive<T>);

impl<T: Ord> PartialEq for MapRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T: Ord> Eq for MapRange<T> {}

impl<T: Ord> PartialOrd for MapRange<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for MapRange<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.start() > other.0.end() {
            Ordering::Greater
        } else if self.0.end() < other.0.start() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

pub struct RangeMap<K, V> {
    map: BTreeMap<MapRange<K>, V>,
}

impl<K: Ord + Copy, V> RangeMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, range: RangeInclusive<K>, v: V) {
        debug_assert!(range.start() <= range.end(), "Reversed range");
        debug_assert!(
            !self.map.contains_key(&MapRange(range.clone())),
            "Range already present or overlaps"
        );
        self.map.insert(MapRange(range), v);
    }

    pub fn get(&self, k: K) -> Option<&V> {
        self.map.get(&MapRange(k..=k))
    }

    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        self.map.get_mut(&MapRange(k..=k))
    }
}
