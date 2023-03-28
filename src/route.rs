use actix_web::web;
use crate::handlers::hr;

pub fn config_route(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/emps")
        .service(
        web::resource("")
            .route(web::get().to(hr::gets))
            .route(web::put().to(hr::update))
            .route(web::post().to(hr::insert)),
        )
        .service(
            web::scope("/{empid}")
            .service(
                web::resource("")
                .route(web::get().to(hr::getbyempid))
                .route(web::delete().to(hr::delete)),
            )
        ),
    );
}