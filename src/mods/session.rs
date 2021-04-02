use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    pub rfc: String,
    pub password: String,
   
}

pub async fn login(rec: web::Json<LoginData>, req: HttpRequest, session: Session) -> Result <HttpResponse, actix_web::Error> {

    let web::Json(data): web::Json<LoginData> = rec;

    session.set("rfc", data.rfc)?;
    println!("SESSION value: {:?}", session.get::<String>("rfc").unwrap());




    println!("{:?}", req);

    Ok(HttpResponse::Ok().body("hola"))

}