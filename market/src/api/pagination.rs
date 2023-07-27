use std::fmt::Debug;

use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Pagination<T: Debug> {
    /// Found results within [offset; offset + limit) range
    ///
    /// when the requested `limit == 0` an empty list is returned
    pub results: Vec<T>,
    /// How many results returned
    pub size: usize,
    /// How many total results matched the query
    pub total: i64,
    /// How many results skipped
    pub offset: Option<i64>,
    /// Max number of results returned
    pub limit: Option<i64>,
}

impl<T> Pagination<T>
where
    T: Debug,
{
    #[must_use]
    pub fn new(results: Vec<T>, total: i64, pagination: PaginationQuery) -> Self {
        let size = results.len();
        Self {
            results,
            size,
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        }
    }
}
