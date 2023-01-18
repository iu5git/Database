use sqlx::PgPool;

use crate::model::task::{dao::task_entity::TaskEntity, task::Task};

pub async fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let task_entities = sqlx::query_as!(
        TaskEntity,
        r#"
        SELECT *
        FROM tasks
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(task_entities.into_iter().map(Task::from).collect())
}

pub async fn get_user_tasks(pool: &PgPool, user_id: i64) -> Result<Vec<Task>, sqlx::Error> {
    let task_entities = sqlx::query_as!(
        TaskEntity,
        r#"
        SELECT *
        FROM tasks
        WHERE created_by = $1
        ORDER BY id
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(task_entities.into_iter().map(Task::from).collect())
}

pub async fn add_task(pool: &PgPool, task: Task) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO tasks (description, done, due_time, created_by)
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
        "#,
        task.description(),
        task.done(),
        task.due_time(),
        task.created_by()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn set_task_done(pool: &PgPool, task_id: i64, done: bool) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE tasks
        SET done = $1
        WHERE id = $2
        "#,
        done,
        task_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}

pub async fn remove_task(pool: &PgPool, task_id: i64) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM tasks
        WHERE id = $1
        "#,
        task_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}
