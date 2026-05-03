use async_trait::async_trait;
use crate::domain::entity::table::{Table, Row, ResultSet};
use crate::domain::entity::value::Value;
use thiserror::Error;
use std::sync::Arc;

/// テーブルリポジトリのエラー型
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Table {0} not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TableAlreadyExists(String),

    #[error("Column {0} not found in table {1}")]
    ColumnNotFound(String, String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Data error: {0}")]
    DataError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// クエリフィルタの条件
#[derive(Debug, Clone)]
pub enum FilterCondition {
    /// 単一条件 (カラム名、演算子、値)
    Simple {
        column: String,
        operator: FilterOperator,
        value: Value,
    },

    /// 複数条件のAND結合
    And(Vec<FilterCondition>),

    /// 複数条件のOR結合
    Or(Vec<FilterCondition>),
}

/// フィルタの演算子
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Like,
}

/// テーブルリポジトリ (データベーステーブルの永続化と取得のための抽象インタフェース)
#[async_trait]
pub trait TableRepository: Send + Sync {
    /// データベースに新しいテーブルを作成する
    async fn create_table(&self, table: &Table) -> Result<(), RepositoryError>;

    /// テーブルが存在するかどうかを確認する
    async fn table_exists(&self, table_name: &str) -> Result<bool, RepositoryError>;

    /// テーブルを削除する
    async fn drop_table(&self, table_name: &str) -> Result<(), RepositoryError>;

    /// 名前でテーブルを取得する
    async fn get_table(&self, table_name: &str) -> Result<Table, RepositoryError>;

    /// すべてのテーブル名を取得する
    async fn get_table_names(&self) -> Result<Vec<String>, RepositoryError>;

    /// テーブルに1行のデータを挿入する
    async fn intert(&self, table_name: &str, row: &Row) -> Result<(), RepositoryError>;

    /// 複数行のデータを一括挿入する
    async fn insert_rows(&self, table_name: &str, rows: &[Row]) -> Result<(), RepositoryError>;

    /// テーブルからデータを取得する
    async fn select(
        &self,
        table_name: &str,
        columns: &[String],
        filter: Option<&FilterCondition>,
    ) -> Result<ResultSet, RepositoryError>;

    /// 条件に合致する行を更新する
    async fn update(
        &self,
        table_name: &str,
        updates: &[(String, Value)],
        filter: Option<&FilterCondition>,
    ) -> Result<usize, RepositoryError>;

    /// 条件に合致する行を削除する
    async fn delete(
        &self,
        table_name: &str,
        filter: Option<&FilterCondition>,
    ) -> Result<usize, RepositoryError>;
}
