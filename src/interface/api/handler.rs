use axum::{
    extract::{Extention, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

use crate::domain::repository::TableRepository;
use crate::infrastructure::parser::{ParsedStatement, SqlParser};

// SQLクエリのリクエスト
#[derive(Deserialize)]
struct QueryRequest {
    sql: String,
}

// クエリ実行結果
#[derive(Serialize)]
pub struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<Vec<serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    affected_rows: Option<usize>,

    statement_type: String,
}

// SQL実行ハンドラ
pub async fn execute_sql_handler(
    Extension(repository): Extension<Arc<TableRepository>>,
    Extention(parser): Extension<Arc<SqlParser>>,
    Json(payload): Json<QueryRequest>,
) -> Result<Json<QueryResult>, ApiError> {
    // SQLの解析と実行
}
