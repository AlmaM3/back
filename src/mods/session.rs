use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use actix_http::
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Login {
    rfc: String,
    password: String,
}

impl Login {
    /// Para instanciar un struct `Contrib`.
    pub fn new(rfc: String, password: String) -> Login {
        Login { rfc, password }
    }
}

pub async fn login(
    session: Session,
    req: HttpRequest,
    rfc: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    // access session data
    //  println!("{:?}", req);

    // RequestSession trait is used for session access

    if check_password(rfc.clone(), password) {
        session.set("RFC", rfc)?;
        println!("SESSION value: {:?}", session.get::<String>("RFC").unwrap());

        Ok(HttpResponse::Ok().body("Se inició sesión correctamente"))
    } else {
        session.set("RFC", rfc)?;
        Ok(HttpResponse::Ok().body("Faiiiiiil"))
    }
}

pub fn check_password(rfc: String, password: String) -> bool {
    let u1 = Login::new(rfc, password);

    let mut datos_login = HashSet::new();

    datos_login.insert(Login {
        rfc: "GOHE951031B16".to_string(),
        password: "AmoALosMichis".to_string(),
    });
    datos_login.insert(Login {
        rfc: "MAPA960503IV8".to_string(),
        password: "Perritus".to_string(),
    });
    datos_login.insert(Login {
        rfc: "A".to_string(),
        password: "ABC123".to_string(),
    });
    datos_login.insert(Login {
        rfc: "B".to_string(),
        password: "123ABC".to_string(),
    });

    if datos_login.contains(&u1) {
        true
    } else {
        false
    }
}
