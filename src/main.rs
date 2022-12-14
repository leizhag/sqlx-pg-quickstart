use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL")
        .expect("Env var DATABASE_URL is required for this example.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
