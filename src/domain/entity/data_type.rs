use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumString;

/// データベースでサポートされるデータ型
#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString, Serialize, Deserialize)]
/// DataType enumは整数、浮動小数点、文字列、真偽値、タイムスタンプ、NULL値をサポートする。
pub enum DataType {
    /// 整数型 (64bi符号付き整数)
    /// strumはDataTypeを文字列から変換できるようにするもの
    #[strum(serialize = "INTEGER")]
    Integer,

    /// 浮動小数点型 (64bit)
    #[strum(serialize = "FLOAT")]
    Float,

    /// 文字列型 (UTF-8)
    #[strum(serialize = "TEXT")]
    Text,

    /// 真偽値型
    #[strum(serialize = "BOOLEAN")]
    Boolean,

    /// 日付時刻型
    #[strum(serialize = "TIMESTAMP")]
    Timestamp,

    /// NULL値が許容される型を表す装飾子
    #[strum(serialize = "NULL")]
    Null,

    /// SQL型制約の表現
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    /// Constraint enumはカラムに設定できる制約を表現する。
    pub enum Constraint {
        /// プライマリキー制約
        PrimaryKey,

        /// 一意制約
        Unique,

        /// 非NULL制約
        NotNull,

        /// デフォルト値制約
        Default(String),
    }
}
