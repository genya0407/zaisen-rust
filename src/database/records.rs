use chrono::prelude::*;

#[derive(Queryable)]
pub struct Recruit {
    pub id: i32,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct RecruitTask {
    pub id: i32,
    pub recruit_id: i32,
    pub task_id: i32,
}

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub recruit_task_id: i32,
    pub user_id: i32,
}