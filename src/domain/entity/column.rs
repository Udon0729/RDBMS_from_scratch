use crete::domain::entity:data_type::{Constraint, DataType};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// テーブルカラムを表現する構造体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Column {
    /// カラム名
    pub name: String,

    /// カラムのデータ型
    pub data_type: DataType,

    /// カラムに適用される制約のリスト
    #[builder(default)]
    pub constraints: Vec<Constraint>,
}
