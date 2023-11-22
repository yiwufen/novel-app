use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde_derive::Serialize;

use crate::opi::{self, OAIbody};

pub fn apiconfig(ctg: &mut web::ServiceConfig) {
    ctg.service(web::scope("/api").service(sentmessages).service(index));
}

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/sentmessages")]
async fn sentmessages(oaibody: web::Json<OAIbody>) -> impl Responder {
    // println!("{:?}", oaibody);

    let message = opi::sentmessages(&oaibody).await;

    match message {
        Ok(message) => {
            println!("Message: {:?}", message);
            // web::Json(message)
            HttpResponse::Ok().json(message)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            // 返回这个错误
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(ErrorMessage {
                error: format!("Error occurred: {}", e),
            }) // 返回错误响应
        }
    }
}

// 定义一个用于错误响应的结构体
#[derive(Serialize)]
struct ErrorMessage {
    error: String,
}
