use actix_session::{Session, CookieSession};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
// use actix_http::


pub fn login(session: Session) -> Result<HttpResponse, Error> {
    // access session data
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        println!("SESSION value: {}", count);
        session.set("counter", count + 1).unwrap();
    } else {
        session.set("counter", 1).unwrap();
    }

    Ok(HttpResponse::Ok().body("jiji"))
}
