use std::{
    cmp::{max, min},
    ops::Range,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct SortBy {
    id: String,
    pub desc: bool,
}

#[wasm_bindgen]
impl SortBy {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, desc: bool) -> Self {
        SortBy { id, desc }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

pub struct Paginate<'a> {
    total_count: i32,
    page_number: i32,
    page_size: i32,
    sort_by: &'a SortBy,
}

impl<'a> Paginate<'a> {
    pub fn new(total_count: i32, page_number: i32, page_size: i32, sort_by: &'a SortBy) -> Self {
        Paginate {
            total_count,
            page_number,
            page_size,
            sort_by,
        }
    }
}

impl<'a> IntoIterator for Paginate<'a> {
    type Item = i32;

    type IntoIter = Range<i32>;

    fn into_iter(self) -> Self::IntoIter {
        let page_count = self.page_size / 10;
        let total_page_count =
            self.total_count / 10 + if self.total_count % 10 != 0 { 1 } else { 0 };

        let range = match self.sort_by.desc {
            true => {
                let mut start = total_page_count + 1 - (self.page_number) * page_count;
                let end = total_page_count + 1 - (self.page_number - 1) * page_count;

                if self.total_count % 10 != 0 {
                    start -= 1;
                }

                start..end
            }
            false => 1 + (self.page_number - 1) * page_count..self.page_number * page_count + 1,
        };

        max(range.start, 1)..min(range.end, total_page_count + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{Paginate, SortBy};

    #[test]
    fn asc_ten_rows() {
        let sort_by = SortBy::new("attestations_count".to_string(), false);
        let page1 = Paginate::new(33596, 1, 10, &sort_by);
        let page2 = Paginate::new(33596, 2, 10, &sort_by);
        let page3 = Paginate::new(33596, 3, 10, &sort_by);
        assert_eq!(page1.into_iter(), 1..2);
        assert_eq!(page2.into_iter(), 2..3);
        assert_eq!(page3.into_iter(), 3..4);
    }

    #[test]
    fn asc_thirty_rows() {
        let sort_by = SortBy::new("attestations_count".to_string(), false);
        let page1 = Paginate::new(33596, 1, 30, &sort_by);
        let page2 = Paginate::new(33596, 2, 30, &sort_by);
        let page3 = Paginate::new(33596, 3, 30, &sort_by);
        assert_eq!(page1.into_iter(), 1..4);
        assert_eq!(page2.into_iter(), 4..7);
        assert_eq!(page3.into_iter(), 7..10);
    }

    #[test]
    fn desc_ten_rows() {
        let sort_by = SortBy::new("attestations_count".to_string(), true);
        let page1 = Paginate::new(33600, 1, 10, &sort_by);
        let page2 = Paginate::new(33600, 2, 10, &sort_by);
        let page3 = Paginate::new(33600, 3, 10, &sort_by);
        assert_eq!(page1.into_iter(), 3360..3361);
        assert_eq!(page2.into_iter(), 3359..3360);
        assert_eq!(page3.into_iter(), 3358..3359);
    }

    #[test]
    fn desc_ten_rows_overlap() {
        let sort_by = SortBy::new("attestations_count".to_string(), true);
        let page1 = Paginate::new(33596, 1, 10, &sort_by);
        let page2 = Paginate::new(33596, 2, 10, &sort_by);
        let page3 = Paginate::new(33596, 3, 10, &sort_by);
        assert_eq!(page1.into_iter(), 3359..3361);
        assert_eq!(page2.into_iter(), 3358..3360);
        assert_eq!(page3.into_iter(), 3357..3359);
    }

    #[test]
    fn desc_thirty_rows() {
        let sort_by = SortBy::new("attestations_count".to_string(), true);
        let page1 = Paginate::new(33600, 1, 30, &sort_by);
        let page2 = Paginate::new(33600, 2, 30, &sort_by);
        let page3 = Paginate::new(33600, 3, 30, &sort_by);
        assert_eq!(page1.into_iter(), 3358..3361);
        assert_eq!(page2.into_iter(), 3355..3358);
        assert_eq!(page3.into_iter(), 3352..3355);
    }

    #[test]
    fn desc_thirty_rows_overlap() {
        let sort_by = SortBy::new("attestations_count".to_string(), true);
        let page1 = Paginate::new(33596, 1, 30, &sort_by);
        let page2 = Paginate::new(33596, 2, 30, &sort_by);
        let page3 = Paginate::new(33596, 3, 30, &sort_by);
        assert_eq!(page1.into_iter(), 3357..3361);
        assert_eq!(page2.into_iter(), 3354..3358);
        assert_eq!(page3.into_iter(), 3351..3355);
    }
}
