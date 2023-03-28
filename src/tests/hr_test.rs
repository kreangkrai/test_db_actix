
#[cfg(test)]
mod unit_tests{
    use crate::handlers::hr::gets;
    use actix_web::{http::{header::ContentType,StatusCode},test};
    #[actix_web::test]
    async fn test_gets_ok(){
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = gets(req).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
    #[actix_web::test]
    async fn test_gets_not_ok(){
        let req = test::TestRequest::default().to_http_request();
        let resp = gets(req).await.unwrap();
        assert_eq!(resp.status(),StatusCode::BAD_REQUEST);
    }
}

#[cfg(test)]
mod integration_tests{
    use actix_web::{http::{header::ContentType,StatusCode},test,web,App};
    use crate::handlers::hr::*;
    use actix_service::Service;
    #[actix_web::test]
    async fn test_get(){
        let app = test::init_service(App::new()
            .route("/", web::get().to(gets))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(),StatusCode::OK);
    }
    #[actix_web::test]
    async fn test_post(){
        let app = test::init_service(App::new()
            .service(web::resource("/").to(gets))).await;
        let req = test::TestRequest::with_uri("/").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(),StatusCode::BAD_REQUEST);
    }
}