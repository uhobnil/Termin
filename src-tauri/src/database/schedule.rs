use ::entity::{schedule, schedule::Entity as Schedule};
use chrono::prelude::*;
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait,
    FromQueryResult, QueryFilter, Set,
};
use serde::Serialize;

use crate::commands::schedule::CreateScheduleInput;

#[derive(FromQueryResult, Serialize, Debug, Clone)]
pub struct ScheduleQueryResult {
    pub id: i32,
    pub content: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTimeUtc,
    pub remind: bool,
    pub repeat: schedule::Repeat,
}

pub struct Query;

impl Query {
    pub async fn get_all_schedule(db: &DbConn) -> Result<Vec<ScheduleQueryResult>, DbErr> {
        let schedule: Vec<ScheduleQueryResult> = Schedule::find().into_model().all(db).await?;
        Ok(schedule)
    }

    pub async fn get_schedule_by_month(
        db: &DbConn,
        year: u32,
        month: u32,
    ) -> Result<Vec<ScheduleQueryResult>, DbErr> {
        let start_date = Local
            .with_ymd_and_hms(year as i32, month, 1, 0, 0, 0)
            .unwrap()
            .to_utc();
        let end_date = if month == 12 {
            Local
                .with_ymd_and_hms(year as i32 + 1, 1, 1, 0, 0, 0)
                .unwrap()
                .to_utc()
        } else {
            Local
                .with_ymd_and_hms(year as i32, month + 1, 1, 0, 0, 0)
                .unwrap()
                .to_utc()
        };

        let schedule: Vec<ScheduleQueryResult> = Schedule::find()
            .filter(schedule::Column::Date.gte(start_date))
            .filter(schedule::Column::Date.lt(end_date))
            .into_model()
            .all(db)
            .await?;

        Ok(schedule)
    }

    pub async fn get_schedule_today(db: &DbConn) -> Result<Vec<ScheduleQueryResult>, DbErr> {
        let now = Local::now();
        let start_time_of_today = Local
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
            .unwrap()
            .to_utc();
        let end_time_of_today = Local
            .with_ymd_and_hms(now.year(), now.month(), now.day() + 1, 0, 0, 0)
            .unwrap()
            .to_utc();
        let schedule: Vec<ScheduleQueryResult> = Schedule::find()
            .filter(schedule::Column::Date.gte(start_time_of_today))
            .filter(schedule::Column::Date.lt(end_time_of_today))
            .into_model()
            .all(db)
            .await?;
        Ok(schedule)
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_schedule(
        db: &DbConn,
        data: CreateScheduleInput,
    ) -> Result<schedule::Model, DbErr> {
        let schedule = schedule::ActiveModel {
            content: Set(data.content),
            date: Set(data.date),
            remind: Set(data.remind.into()),
            repeat: Set(data.repeat),
            ..Default::default()
        };
        let schedule = schedule.insert(db).await?;
        Ok(schedule)
    }

    pub async fn update_schedule(
        db: &DbConn,
        id: i32,
        data: CreateScheduleInput,
    ) -> Result<schedule::Model, DbErr> {
        let schedule = schedule::ActiveModel {
            content: Set(data.content),
            date: Set(data.date),
            remind: Set(data.remind.into()),
            repeat: Set(data.repeat),
            ..Default::default()
        };
        
        Schedule::update_many()
            .filter(schedule::Column::Id.eq(id))
            .set(schedule)
            .exec(db)
            .await?;
            
        Schedule::find_by_id(id).one(db).await?
            .ok_or(DbErr::Custom("Schedule not found".to_string()))
    }

    pub async fn delete_schedule(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Schedule::delete_many()
            .filter(schedule::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }
}