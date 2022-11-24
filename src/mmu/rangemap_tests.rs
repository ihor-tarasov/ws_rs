use crate::mmu::rangemap::RangeMap;

#[test]
fn insert_and_get() {
    let mut m = RangeMap::new();
    m.insert(0..=10, 100);
    m.insert(12..=12, 200);
    assert_eq!(m.get(5), Some(&100));
    assert_eq!(m.get(0), Some(&100));
    assert_eq!(m.get(10), Some(&100));
    assert_eq!(m.get(11), None);
    assert_eq!(m.get(12), Some(&200));
}

#[test]
#[should_panic]
fn overlap_in() {
    let mut m = RangeMap::new();
    m.insert(2..=6, 100);
    m.insert(3..=5, 200);
}

#[test]
#[should_panic]
fn overlap_out() {
    let mut m = RangeMap::new();
    m.insert(2..=6, 100);
    m.insert(1..=7, 100);
}

#[test]
#[should_panic]
fn overlap_similar() {
    let mut m = RangeMap::new();
    m.insert(2..=6, 100);
    m.insert(2..=6, 100);
}

#[test]
#[should_panic]
fn reversed_range() {
    let mut m = RangeMap::new();
    m.insert(6..=3, 100);
}
