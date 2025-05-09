use rocket::http::Status;

#[macro_use]
extern crate rocket;

#[get("/healthz")]
fn healthz() -> Status {
    Status::Ok
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![healthz])
        .launch()
        .await?;

    Ok(())
}
