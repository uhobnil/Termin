use crate::database::schedule::ScheduleQueryResult;
use crate::state::ScheduleState;
use chrono::{prelude::*, Days};
use entity::schedule::Repeat;
use tauri::{AppHandle, Emitter};

pub async fn notify(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let state = tauri::Manager::state::<ScheduleState>(app);
    let schedules = state.get_schedules().await;
    let now = Local::now();

    let current_schedules: Vec<_> = schedules
        .iter()
        .filter(|schedule| should_notify(schedule, &now))
        .collect();

    if !current_schedules.is_empty() {
        app.emit("notify", current_schedules)?;
    }

    Ok(())
}

fn should_notify(schedule: &ScheduleQueryResult, now: &DateTime<Local>) -> bool {
    if !schedule.remind {
        return false;
    }

    // 检查是否在有效期内
    if !is_schedule_valid(schedule, now) {
        return false;
    }

    // 检查是否到达提醒时间
    matches_notification_time(schedule, now)
}

fn is_schedule_valid(schedule: &ScheduleQueryResult, now: &DateTime<Local>) -> bool {
    let schedule_time = schedule.date.with_timezone(now.offset());

    match schedule.repeat {
        // 一次性日程：检查是否已过期
        Repeat::ONCE => schedule_time.timestamp() >= now.timestamp(),
        // 重复性日程：检查基准时间部分
        Repeat::DAILY => true, // 每日重复永不过期
        Repeat::WEEKLY => true,
        Repeat::MONTHLY => {
            // 确保日期有效（比如避免2月30日这样的情况）
            is_valid_monthly_date(schedule_time.day(), now)
        }
        Repeat::YEARLY => {
            // 确保日期有效（比如避免2月29日在非闰年）
            is_valid_yearly_date(schedule_time.month(), schedule_time.day(), now)
        }
    }
}

fn matches_notification_time(schedule: &ScheduleQueryResult, now: &DateTime<Local>) -> bool {
    let schedule_time = schedule.date.with_timezone(now.offset());

    // 基础时间匹配：小时、分钟和秒
    let time_matches = schedule_time.hour() == now.hour()
        && schedule_time.minute() == now.minute()
        && schedule_time.second() == now.second();

    match schedule.repeat {
        Repeat::ONCE => schedule_time.timestamp() == now.timestamp(),
        Repeat::DAILY => time_matches,
        Repeat::WEEKLY => schedule_time.weekday() == now.weekday() && time_matches,
        Repeat::MONTHLY => schedule_time.day() == now.day() && time_matches,
        Repeat::YEARLY => {
            schedule_time.month() == now.month() && schedule_time.day() == now.day() && time_matches
        }
    }
}

fn is_valid_monthly_date(schedule_day: u32, now: &DateTime<Local>) -> bool {
    // 获取当月的最后一天
    let last_day = now
        .with_day(1)
        .unwrap()
        .with_month(now.month() + 1)
        .unwrap_or_else(|| now.with_month(1).unwrap())
        .checked_sub_days(Days::new(1))
        .unwrap()
        .day();

    schedule_day <= last_day
}

fn is_valid_yearly_date(schedule_month: u32, schedule_day: u32, now: &DateTime<Local>) -> bool {
    // 检查月份和日期的组合是否在当前年份有效
    now.with_month(schedule_month)
        .and_then(|dt| dt.with_day(schedule_day))
        .is_some()
}
