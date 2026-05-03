use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::entity::{Column, DataType, Row, Table, Value};
use crate::domain::repository::{FilterCondition, FilterOperator};
use thiserror::Error;

/// ストレージエラー
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Table {0} not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TableAlreadyExists(String),
    // 他に記述するエラー
}

/// テーブルのデータを保持する構造体
#[derive(Debug, Clone)]
struct TableData {
    schema: Table,
    rows: Vec<Row>,
}

/// インメモリストレージの実装
/// ハッシュマップを使用して、テーブル名とテーブルデータのマッピングを保持する。
/// 読み取り/書き込みロックを使用して並行アクセスを管理する。
#[derive(Debug, Default)]
pub struct MemoryStorage {
    tables: RwLock<HashMap<String, TableData>>,
}
