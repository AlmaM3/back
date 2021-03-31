use actix_web::{web, HttpResponse};
use serde::Serialize;
use serde_json;
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};

use super::add_remove::{Received, ServerData};

fn query_protected(bd: &Connection) {
    let mut query = bd.prepare("SELECT * FROM rfc_noa");

    match query {
        Ok(query) => {
            let mut query2 = query;
            let query_iter = query2.query_map(params![], |row| {
                Ok(Received {
                    id: row.get(0)?,
                    rfc: row.get(1)?,
                    fecha: row.get(2)?,
                    modificador: row.get(3)?,
                })
            });

            match query_iter {
                Ok(iter) => {
                    for x in iter {
                        println!("{:?}", x.unwrap());
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    //     for x in query_iter {
    //         println!("{:?}", x.unwrap());
    //     }
}

pub async fn protected(
    received: web::Json<Received>, //rfc: web::Path<String>,
    bd: web::Data<ServerData>,
) -> Result<HttpResponse, actix_web::Error> {
    // match bd.connection.lock() {
    //     Ok(bd) => {
    //         query_protected(&bd);
    //         Ok(HttpResponse::Ok().body("cheems"))
    //     }
    //     _ => Ok(HttpResponse::Ok().body("error")),
    // }

    // let bd = web::Data::new(Arc::new(Mutex::new(super::bd::crea_bd())));
    // let x = bd.lock();

    let datos = [
            Received {
                id: 1,
                rfc: "GOHE".to_string(),
                fecha: "01/01/01".to_string(),
                modificador: 4,
            },
            Received {
                id: 2,
                rfc: "MAPA".to_string(),
                fecha: "02/02/02".to_string(),
                modificador: 4,
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
