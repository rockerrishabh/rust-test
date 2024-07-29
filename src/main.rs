use self::models::*;
use actix_web::{get, App, HttpServer};
use backend::*;
use diesel::prelude::*;

#[get("/")]
async fn index() -> String {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(email.eq("rishabh.6542@gmail.com".to_string()))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    let user = &results[0];

    format!("Displaying {} users", user.email)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
