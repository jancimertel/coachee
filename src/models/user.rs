use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: bool,
}

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_users(conn: &mut PgConnection) -> Result<Option<Vec<User>>, DbError> {
    use crate::schema::users::dsl::*;

    let results = users.limit(5).load::<User>(conn).optional()?;

    Ok(results)
}
