use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::postgres::PgPoolOptions;

use crate::{
    infrastructure::{author_repository::PgAuthorRepository, user_repository::PgUserRepository},
    presentational::graphql::{
        query::Query, query_service::QueryServiceImpl, schema::build_schema,
    },
    use_case::interactor::{author::ShowAuthorInteractor, user::LoginInteractor},
};

pub type QSI =
    QueryServiceImpl<LoginInteractor<PgUserRepository>, ShowAuthorInteractor<PgAuthorRepository>>;

pub async fn dependency_injection() -> Schema<Query<QSI>, EmptyMutation, EmptySubscription> {
    let db_url = fetch_database_url();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    // TODO: newを使う
    let user_repository = PgUserRepository { pool: pool.clone() };
    let login_use_case = LoginInteractor::new(user_repository);
    let author_repository = PgAuthorRepository { pool: pool.clone() };
    let show_author_use_case = ShowAuthorInteractor::new(author_repository);
    let query_service = QueryServiceImpl {
        login_use_case,
        show_author_use_case,
    };
    let query = Query::new(query_service);

    build_schema(query)
}

fn fetch_database_url() -> String {
    use std::env::VarError;

    match std::env::var("DATABASE_URL") {
        Ok(s) => s,
        Err(VarError::NotPresent) => panic!("Environment variable DATABASE_URL is required."),
        Err(VarError::NotUnicode(_)) => panic!("Environment variable DATABASE_URL is not unicode."),
    }
}
