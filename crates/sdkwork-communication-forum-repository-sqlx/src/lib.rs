pub mod schema;
pub mod repo_impl;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct SqlxForumRepository {
    pool: PgPool,
}

impl SqlxForumRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn new_placeholder() -> Self {
        Self {
            pool: PgPool::connect_lazy("postgres://localhost:5432/forum")
                .expect("Failed to create placeholder pool"),
        }
    }
}
