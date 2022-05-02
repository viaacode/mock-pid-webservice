use actix_web::{get, web, Responder, Result, App, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct Pid {
    id: String,
}

#[get("/pid")]
async fn pid() -> Result<impl Responder> {
    let pid = Pid{id: String::from("1")};
    let pids = vec![pid];
    Ok(web::Json(pids))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(pid)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
