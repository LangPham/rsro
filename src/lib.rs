use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct User {
    username: String,
    id: i64,
}

#[tokio::main]
pub async fn run() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/rsro")
        .await?;

    let sql_str = "SELECT * FROM users";
    let vu = sqlx::query_as::<_, User>(sql_str)
        .bind("users")
        .fetch_all(&pool);

    for i in &vu.await? {
        dbg!(i);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let user = User {
            username: "kiemtra".to_string(),
            id: 1,
        };
        dbg!(user);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/rsro");

        let p = pool.await.expect("KHong ket noi database");

        let sql_str = "SELECT * FROM users";
        let vu = sqlx::query_as::<_, User>(sql_str)
            .bind("users")
            .fetch_all(&p);

        let rvu = vu.await.expect("KHong ket noi database");
        for i in &rvu {
            dbg!(i);
        }
    }
}
