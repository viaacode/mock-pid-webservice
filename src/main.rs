use actix_web::{get, web, App, HttpServer, Responder, Result};
use rand::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Pid {
    id: String,
}

#[get("/pid")]
async fn pid() -> Result<impl Responder> {
    let pid = Pid {
        id: rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>()
            .to_ascii_lowercase(),
    };
    let pids = vec![pid];
    Ok(web::Json(pids))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(pid))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
