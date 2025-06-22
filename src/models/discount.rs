use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total_items: u64,
    pub total_pages: u32,
}
