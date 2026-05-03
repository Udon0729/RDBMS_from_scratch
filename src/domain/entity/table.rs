use crate::domain::entity::column::Column;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// データベーステーブルを表現する構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
/// テーブルはカラム定義のリストを持つ。
pub struct Table {
    /// テーブル名
    pub name: String,

    /// テーブルのカラム
    pub columns: Vec<Column>,
}

/// 1行のデータを表現する
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 行はカラム名と値のマッピングを持つ。
pub struct Row {
    /// カラム名と値のマッピング
    pub values: HashMap<String, crate::domain::entity::value::Value>,
}

/// クエリ結果セットを表現する
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// クエリ結果はカラム定義と複数の行データを持つ。
pub struct ResultSet {
    /// 結果セットのスキーマ (カラム定義)
    pub columns: Vec<Column>,

    /// 結果の行データ
    pub rows: Vec<Row>,
}
