use crate::domain::entity::{Column, DataType, Value};
use crate::domain::repository::{FilterCondition, FilterOperator};
use sqlparser::ast::{
    Expr, Ident, ObjectName, Query, SelectItem, SetExpr, Statement, TableFactor, TableWithJoins,
    Value as SqlValue, Values,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use thiserror::Error;

/// SQL解析エラー
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("SQL syntax error: {0}")]
    SyntaxError(String),

    #[error("Unsupported SQL statement: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid data type: {0}")]
    InvalidDataType(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Internal parser error: {0}")]
    InternalError(String),
}

/// SQLパーサ
/// SQLを分析して、内部表現　(ParsedStatement) に変換するクレート。
pub struct SqlParser {
    dialect: GenericDialect,
}

/// 解析されたSQLクエリ
pub enum ParsedStatement {
    CreateTable(CreateTableStatement),
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    DropTable(DropTableStatement),
}
