use crate::models::user_model::{User, UserDB};
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users_db = sqlx::query_as::<_, UserDB>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        
        Ok(users_db.into_iter().map(User::from).collect())
    }

    pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as::<_, UserDB>(
            "SELECT * FROM users WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool) // pakai optional karena bisa tidak ada data
        .await?;
        
        Ok(result.map(User::from)) // convert Option<UserDB> ke Option<User>
    }
}
