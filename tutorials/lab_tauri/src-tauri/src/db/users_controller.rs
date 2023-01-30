use sqlx::PgPool;

use crate::model::user::{dao::user_entity::UserEntity, user::User};

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let user_entities = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT *
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(user_entities.into_iter().map(User::from).collect())
}

pub async fn get_user_by_login(pool: &PgPool, login: &String) -> Result<Option<User>, sqlx::Error> {
    let user_entity = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT *
        FROM users
        WHERE login = $1
        ORDER BY id
        "#,
        login
    )
    .fetch_optional(pool)
    .await?;

    Ok(user_entity.map(User::from))
}

pub async fn add_user(pool: &PgPool, user: User) -> Result<i64, sqlx::Error> {
    if user.login().is_empty() || user.password().is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }
    let rec = sqlx::query!(
        r#"
        INSERT INTO users (login, password)
        VALUES ( $1, crypt($2, gen_salt('bf')) )
        RETURNING id
        "#,
        user.login(),
        user.password()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn remove_user(pool: &PgPool, user_id: i64) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}

pub async fn try_login_user(
    pool: &PgPool,
    login: String,
    password: String,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT *
        FROM users
        WHERE login = $1
        AND password = crypt($2, password)
        "#,
        login,
        password
    )
    .fetch_optional(pool)
    .await?;

    Ok(user.map(User::from))
}
