use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedView<T> {
    pub results: Vec<T>,
    pub page_count: i64,
}

impl<T> PaginatedView<T> {
    pub fn new(results: Vec<T>, page_count: i64) -> Self {
        PaginatedView {
            results,
            page_count,
        }
    }
}
