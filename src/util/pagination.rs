use std::{cmp::min, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(15),
        }
    }
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub start: i64,
    pub end: i64,
    pub page: i64,
    pub page_size: i64,
    pub has_next: bool,
    pub has_prev: bool,
}
impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: i64, page_size: i64) -> Self {
        let offset = (page - 1) * page_size;
        let has_prev = page > 1;
        let has_next = offset + (items.len() as i64) < total;
        let start = (page - 1) * page_size + 1;
        let end = min(page * page_size, total);

        Self {
            items,
            total,
            start,
            end,
            page,
            page_size,
            has_next,
            has_prev,
        }
    }
}
impl<T> Default for PaginatedResponse<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            total: 0,
            page: 1,
            start: 1,
            end: 1,
            page_size: 10,
            has_next: false,
            has_prev: false,
        }
    }
}

#[async_trait]
pub trait Paginatable:
    Sized + Send + Unpin + for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Serialize
{
    fn count_query() -> &'static str;
    fn page_query() -> &'static str;

    /// Helps you paginate anything in the table. Do not include the "WHERE",
    /// that will be automatically inserted before the query you send in.
    ///
    /// Example:
    /// ```
    /// User::paginate(&db, &paging, "first_name LIKE ?", "%am%")
    /// ```
    async fn paginate(
        pool: &Arc<SqlitePool>,
        pagination: &Pagination,
        where_clause: Option<&str>,
        args: Vec<&str>,
    ) -> Result<PaginatedResponse<Self>, sqlx::Error> {
        let page = pagination.page.unwrap_or(1);
        let page_size = pagination.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;

        // count total
        let count_sql = if let Some(wc) = where_clause {
            format!("{} WHERE {}", Self::count_query(), wc)
        } else {
            Self::count_query().to_string()
        };

        let mut total_query = sqlx::query_as(&count_sql);
        for arg in args.iter() {
            total_query = total_query.bind(arg);
        }
        let total: (i64,) = total_query.fetch_one(pool.as_ref()).await?;

        // fetch rows
        let page_sql = if let Some(wc) = where_clause {
            format!("{} WHERE {} LIMIT ? OFFSET ?", Self::page_query(), wc)
        } else {
            format!("{} LIMIT ? OFFSET ?", Self::page_query())
        };
        let mut rows_query = sqlx::query_as::<_, Self>(&page_sql);
        for arg in args.iter() {
            rows_query = rows_query.bind(arg);
        }
        let rows = rows_query
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool.as_ref())
            .await?;

        Ok(PaginatedResponse::new(rows, total.0, page, page_size))
    }
}
