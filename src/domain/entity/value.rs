use crate::domain::entity::data_type::DataType;
use chrono::{DataTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// 値型エラー
#[derive(Error, Debug, PartialEq)]
pub enum ValueError {
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: DataType,
        actual: DataType,
    },

    #[error("cannot convert from {0} to {1}")]
    ConversionError(String, String),

    #[error("Null value not allowed")]
    NullValueError,
}

/// データベース内の値の表現
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 各バリアントは特定のデータ型の値を保持し、型変換や表示などのメソッドを提供する。
pub enum Value {
    /// 整数値
    Integer(i64),

    /// 浮動小数点値
    Float(f64),

    /// 文字列値
    Text(String),

    /// 真偽値
    Boolean(bool),

    /// 日時値
    Timestamp(DataTime<Utc>),

    /// NULL値
    Null,
}
