use tokio::sync::RwLock;
use std::sync::Arc;
use sea_orm::DbConn;
use crate::database::schedule::{Query, ScheduleQueryResult};

pub struct ScheduleState {
    schedules: Arc<RwLock<Vec<ScheduleQueryResult>>>,
}

impl ScheduleState {
    pub fn new() -> Self {
        Self {
            schedules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn refresh(&self, db: &DbConn) -> Result<(), sea_orm::DbErr> {
        let schedules = Query::get_all_schedule(db).await?;
        let mut write_guard = self.schedules.write().await;
        *write_guard = schedules;
        Ok(())
    }

    pub async fn get_schedules(&self) -> Vec<ScheduleQueryResult> {
        let read_guard = self.schedules.read().await;
        read_guard.clone()
    }
} 