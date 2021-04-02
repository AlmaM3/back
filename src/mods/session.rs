use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub rfc: String,
    pub password: String,
   
}

pub async fn login(rec: web::Json<LoginData>, req: HttpRequest, session: Session) -> Result <HttpResponse, actix_web::Error> {

    let web::Json(data): web::Json<LoginData> = rec;

    if check_password(&data) {
        session.set("RFC", &data.rfc)?;
        println!("SESSION value: {:?}", session.get::<String>("RFC").unwrap());

        Ok(HttpResponse::Ok().body("Se inició sesión correctamente"))
    } else {
        Ok(HttpResponse::Ok().body("Faiiiiiil"))
    }
    // session.set("rfc", data.rfc)?;
    // println!("SESSION value: {:?}", session.get::<String>("rfc").unwrap());


    // println!("{:?}", req);

    // Ok(HttpResponse::Ok().body("hola"))

}

pub fn check_password(data: &LoginData) -> bool {


    let mut datos_login = HashSet::new();

    datos_login.insert(LoginData {
        rfc: "GOHE".to_string(),
        password: "AmoALosMichis".to_string(),
    });
    datos_login.insert(LoginData {
        rfc: "MAPA".to_string(),
        password: "Perritus".to_string(),
    });
    datos_login.insert(LoginData {
        rfc: "A".to_string(),
        password: "ABC123".to_string(),
    });
datos_login.insert(LoginData {
        rfc: "B".to_string(),
        password: "123ABC".to_string(),
    });

    if datos_login.contains(data) {
        true
    } else {
        false
    }
}