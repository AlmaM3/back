use actix_web::{web, HttpResponse};
use serde::Serialize;
use serde_json;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]

struct Sent {
    rfc: String,
    fecha: String,
    modificador: String,
}

pub async fn protected() -> Result<HttpResponse, actix_web::Error> {
    // let bd = web::Data::new(Arc::new(Mutex::new(super::bd::crea_bd())));
    // let x = bd.lock();


    
    let datos = [
        Sent {
            rfc: "GOHE".to_string(),
            fecha: "01/01/01".to_string(),
            modificador: "Susana".to_string(),
        },
        Sent {
            rfc: "MAPA".to_string(),
            fecha: "02/02/02".to_string(),
            modificador: "Susana".to_string(),
        },
    ];

    let datos_serde = serde_json::json!([
        {"rfc": datos[0].rfc,
        "fecha": datos[0].fecha,
        "modificador": datos[0].modificador
    },
        {"rfc": datos[1].rfc,
        "fecha": datos[1].fecha,
        "modificador": datos[1].modificador
    }]
    );

    // match x {
    //     Ok(bd) =>{
    //         Ok(HttpResponse::Ok().body("listo para consultar los protegidos"))

    //     },
    //     _ => Ok(super::add_remove::error500()),
    // }

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(datos_serde))
}
