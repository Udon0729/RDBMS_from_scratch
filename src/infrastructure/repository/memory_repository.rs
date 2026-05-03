use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entity::{Table, Row, Value, ResultSet};
use crate::domain::repository::{TableRepository, RepositoryError, FilterCondition};
use crate::infrastructure::storrage::{MemoryStorage, StorageError};

/// インメモリリポジトリ
/// MemoryStorageを使用して、テーブルとデータを保存・取得する。非同期メソッドを提供し、ストレージエラーをリポジトリエラーに変換する。
pub struct MemoryTableRepository {
    storage: Arc<MemoryStorage>,
}

#[async_trait]
impl TableRepository for MemoryTableRepository {
    // TableRepositoryトレイトのメソッド
    async fn create_table(&self, table: Table) -> Result<(), RepositoryError> {
        // 今後の実装
    }
    // 他のメソッドもここに実装
}
