use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

// this needs to match the entity in the db 1 to 1
#[allow(dead_code)]
#[derive(Debug)]
struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // this will create a connection to the db. This will need to be wrapped in an Arc for use
    // in an async framework.
    let pool = PgPoolOptions::new()
        .max_connections(5) //can be omitted
        .connect("postgres://postgres:password@localhost/newsletter")
        // the ip can be found with docker inspect <container ip>
        .await?;

    //
    let result = sqlx::query!(r#"SELECT * FROM subscriptions"#)
        .fetch_optional(&pool)
        .await?
        .map(|r| r.name);

    println!("before i inserted : {:?}", result);

    let email = "bob@admin.com";
    let name = "alice admin";
    let insert = sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        email,
        name,
        Utc::now()
    )
    .execute(&pool)
    .await;

    println!("the insert value is:{:?}", insert.unwrap());
    let subscription = sqlx::query_as!(Subscription, r#"SELECT * FROM subscriptions"#)
        .fetch_optional(&pool)
        .await?; //this is wrapped into a result<option>

    println!(
        "after I inserted : {:?}",
        subscription.expect("There should be the subscription inserted above")
    );

    Ok(())
}
