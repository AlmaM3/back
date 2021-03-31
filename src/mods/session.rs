// use actix_session::{CookieSession, Session};
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// // use actix_http::
// use std::collections::HashSet;
// use serde::{Deserialize,Serialize};
// use super::add_remove::ServerData;

// #[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
// pub struct Login {
//     rfc: String,
//     password: String,
// }

// impl Login {
//     /// Para instanciar un struct `Contrib`.
//     pub fn new(rfc: String, password: String) -> Login {
//         Login { rfc, password }
//     }
// }

// pub async fn login(
//     received: web::Json<Login>,
//     session: Session,
//     //req: HttpRequest,
//     bd: web::Data<ServerData>
// ) -> Result<HttpResponse, actix_web::Error> {
//     // access session data
//     //  println!("{:?}", req);

//     // RequestSession trait is used for session access

//     let web::Json(data): web::Json<Login> = received;
//     println!("{:?}", data);
//     println!("SESSION value: {:?}", session.get::<String>("RFC").unwrap());

    

//     if check_password(&data) {
//         session.set("RFC", &data.rfc)?;
//         println!("SESSION value: {:?}", session.get::<String>("RFC").unwrap());

//         Ok(HttpResponse::Ok().body("Se inició sesión correctamente"))
//     } else {
//         Ok(HttpResponse::Ok().body("Faiiiiiil"))
//     }
// }

// pub fn check_password(data: &Login) -> bool {


//     let mut datos_login = HashSet::new();

//     datos_login.insert(Login {
//         rfc: "GOHE951031B16".to_string(),
//         password: "AmoALosMichis".to_string(),
//     });
//     datos_login.insert(Login {
//         rfc: "MAPA960503IV8".to_string(),
//         password: "Perritus".to_string(),
//     });
//     datos_login.insert(Login {
//         rfc: "A".to_string(),
//         password: "ABC123".to_string(),
//     });
//     datos_login.insert(Login {
//         rfc: "B".to_string(),
//         password: "123ABC".to_string(),
//     });

//     if datos_login.contains(data) {
//         true
//     } else {
//         false
//     }
// }
