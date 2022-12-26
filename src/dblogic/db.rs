use sea_orm::*;

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    match std::env::var("DATABASE_URL") {
        Ok(dburl) => return Database::connect(dburl).await,
        Err(err) => {
            panic!("{:?}", err)
        }
    };
}
