use application::user_service::UserRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::error::AppError;
use domain::user::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput,
    GetUserInput, ListUsersInput, UpdateUserInput, User,
};
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    user_id: String,
    client_id: String,
    username: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            user_id: self.user_id,
            client_id: self.client_id,
            username: self.username,
            email: self.email,
            name: self.name,
            picture: self.picture,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

fn map_db_error(error: sqlx::Error) -> AppError {
    match &error {
        sqlx::Error::Database(db_error) => match db_error.code() {
            Some(code) if code == "23505" => AppError::conflict("User", db_error.message()),
            Some(code) if code == "23503" => AppError::conflict("User", db_error.message()),
            _ => AppError::database(db_error.message().to_string()),
        },
        _ => AppError::database(error.to_string()),
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (user_id, client_id, username, email, name, picture, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING user_id, client_id, username, email, name, picture, created_at, updated_at
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.client_id)
        .bind(&input.username)
        .bind(&input.email)
        .bind(&input.name)
        .bind(&input.picture)
        .fetch_one(&self.pool)
        .await
        .map_err(map_db_error)?;

        Ok(row.into_user())
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<User, AppError> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");
        let mut separated = builder.separated(", ");

        if let Some(username) = &input.username {
            separated.push("username = ").push_bind(username);
        }
        if let Some(email) = &input.email {
            separated.push("email = ").push_bind(email);
        }
        if let Some(name) = &input.name {
            separated.push("name = ").push_bind(name);
        }
        if let Some(picture) = &input.picture {
            separated.push("picture = ").push_bind(picture);
        }

        separated.push("updated_at = NOW()");

        builder.push(" WHERE user_id = ");
        builder.push_bind(&input.user_id);
        builder.push(" RETURNING user_id, client_id, username, email, name, picture, created_at, updated_at");

        let row = builder
            .build_query_as::<UserRow>()
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_error)?;

        row.map(|row| row.into_user())
            .ok_or_else(|| AppError::not_found("User", "User not found"))
    }

    async fn delete_user(&self, input: DeleteUserInput) -> Result<i64, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(&input.user_id)
            .execute(&self.pool)
            .await
            .map_err(map_db_error)?;

        if result.rows_affected() == 0 {
            return Err(AppError::not_found("User", "User not found"));
        }

        Ok(result.rows_affected() as i64)
    }

    async fn get_user(&self, input: GetUserInput) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(&input.user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_error)?;

        Ok(row.map(|row| row.into_user()))
    }

    async fn get_user_by_email(
        &self,
        input: GetUserByEmailInput,
    ) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(&input.email)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_error)?;

        Ok(row.map(|row| row.into_user()))
    }

    async fn get_user_by_username(
        &self,
        input: GetUserByUsernameInput,
    ) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(&input.username)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_error)?;

        Ok(row.map(|row| row.into_user()))
    }

    async fn list_users(&self, input: ListUsersInput) -> Result<Vec<User>, AppError> {
        let page = input.page.unwrap_or(1).max(1);
        let limit = input.limit.unwrap_or(20).max(1).min(100);
        let offset = (page - 1) * limit;

        let rows = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE client_id = $1
            ORDER BY created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
        )
        .bind(&input.client_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_error)?;

        Ok(rows.into_iter().map(|row| row.into_user()).collect())
    }
}
