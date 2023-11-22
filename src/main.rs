mod apiconfig;
mod opi;
use apiconfig::apiconfig;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

#[cfg(test)]
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .configure(apiconfig)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
