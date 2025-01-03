use crate::{
    database::schedule::{Mutation, Query, ScheduleQueryResult},
    errors::AppError,
    state::ScheduleState
};
use entity::schedule;
use sea_orm::{prelude::DateTimeUtc, DbConn};
use serde::Deserialize;
use tauri::State;

#[derive(Deserialize, Debug)]
pub struct CreateScheduleInput {
    pub content: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTimeUtc,
    pub remind: bool,
    pub repeat: schedule::Repeat,
}

#[tauri::command]
pub async fn get_all_schedule(db: State<'_, DbConn>) -> Result<Vec<ScheduleQueryResult>, AppError> {
    let res = Query::get_all_schedule(&db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn get_schedule_by_month(
    db: State<'_, DbConn>,
    year: u32,
    month: u32,
) -> Result<Vec<ScheduleQueryResult>, AppError> {
    let res = Query::get_schedule_by_month(&db, year, month).await?;
    Ok(res)
}

#[tauri::command]
pub async fn get_schedule_today(
    db: State<'_, DbConn>,
) -> Result<Vec<ScheduleQueryResult>, AppError> {
    let res = Query::get_schedule_today(&db).await?;
    Ok(res)
}

#[tauri::command]
pub async fn create_schedule(
    state: State<'_, ScheduleState>,
    db: State<'_, DbConn>,
    data: CreateScheduleInput,
) -> Result<schedule::Model, String> {
    let result = Mutation::create_schedule(&db, data)
        .await
        .map_err(|e| e.to_string())?;
    
    // 刷新内存中的数据
    state.refresh(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn update_schedule(
    state: State<'_, ScheduleState>,
    db: State<'_, DbConn>,
    id: i32,
    data: CreateScheduleInput,
) -> Result<schedule::Model, String> {
    let result = Mutation::update_schedule(&db, id, data)
        .await
        .map_err(|e| e.to_string())?;
    
    state.refresh(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn delete_schedule(
    state: State<'_, ScheduleState>,
    db: State<'_, DbConn>,
    id: i32,
) -> Result<(), String> {
    Mutation::delete_schedule(&db, id)
        .await
        .map_err(|e| e.to_string())?;
    
    state.refresh(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
