use actix_web::{middleware,App,HttpServer};
use test_db_actix::route::config_route;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_service=info,actix_web=info");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
        .configure(config_route)
        .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1",8082))?
    .run();
    println!("Server running...");
    server.await
}