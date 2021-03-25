extern crate rusqlite;
use rusqlite::{params, Connection, Result};
//use rusqlite::NO_PARAMS; se puede sustituir por params![]
use actix_web::{web, HttpResponse};

use std::sync::{Arc, Mutex};

/// Implementado para indicar si una sentencia de SQL es exitosa o no.
pub enum Respuesta {
    /// Éxito de la consulta.
    Exito(i64),
    /// Fallo de la consulta (Código 500).
    Error500(HttpResponse),
}

#[doc(hidden)]
// Función que devuelve un Response con error 500.
pub fn error500() -> HttpResponse {
    HttpResponse::InternalServerError().body("An unexpected error has ocurred.")
}

/// Ejecuta una consulta para verificar si existe o no un RFC.
/// Si la consulta fue exitosa devuelve `Respuesta::Exito(i64)`
/// (el tipo de dato i64 es 1 si el RFC existe, 0 si no existe),
/// si no lo fue devuelve `Respuesta::Error500(HttpResponse)`, con un error 500.
fn exist_rfc(bd: &Connection, rfc: String) -> Respuesta {
    // Retorna un enum con el número de filas de la consulta (0 ó 1)
    // o con un HttpResponse con código 500.
    match bd.query_row(
        "SELECT COUNT(*) FROM rfc_noa WHERE rfc = ?1",
        params![rfc],
        |row| row.get(0),
    ) {
        Ok(filas) => Respuesta::Exito(filas),
        _ => Respuesta::Error500(error500()),
    }
}

/// Inserta un RFC en la base de datos y devuelve `Respuesta::Exito(i64)`
/// si la sentencia de inserción ha sido exitosa. En otro caso, devuelve
/// `Respuesta::Error500(HttpResponse)`, con un error 500.
fn insert_rfc(bd: &Connection, data: Received) -> Respuesta {
    // Devuelve un código 500 si no se ejecuta la sentencia de inserción
    match bd.execute("INSERT INTO rfc_noa (rfc, fecha, modificador) VALUES (?1, ?2, ?3)", 
    params![data.rfc.to_string(), data.fecha.to_string(), data.modificador]) {
        Ok(_insert) => Respuesta::Exito(1),
        _ => Respuesta::Error500(error500()),
    }
}

/// Elimina un RFC en la base de datos y devuelve `Respuesta::Exito(i64)`
/// si la sentencia de inserción ha sido exitosa. En otro caso, devuelve
/// `Respuesta::Error500(HttpResponse)`, con un error 500.
fn delete_rfc(bd: &Connection, data: Received) -> Respuesta {
    // Devuelve un código 500 si no se ejecuta la sentencia de borrado
    match bd.execute("DELETE FROM rfc_noa WHERE rfc = ?1", params![data.rfc]) {
        Ok(_delete) => Respuesta::Exito(1),
        _ => Respuesta::Error500(HttpResponse::InternalServerError().body("")),
    }
}

/// Añade el RFC en caso de no existir y emite un código 200. Si el RFC ya existe emite un código 404.
/// En otro caso despliega un error 500.
fn anadir_rfc(data: Received, bd: &Connection) -> Result<HttpResponse, actix_web::Error> {
    let rfc = &data.rfc;
    // exist_rfc devuelve 1 si el RFC existe en la bd y 0 si no.
    match exist_rfc(&bd, format!("{}", rfc)) {
        Respuesta::Exito(filas) => {
            if filas == 0 {
                insert_rfc(&bd, data);
                Ok(HttpResponse::Ok().body("El RFC se agregó exitosamente."))
            } else {
                Ok(HttpResponse::NotFound().body("El RFC ya existe."))
            }
        }
        Respuesta::Error500(err500) => Ok(err500),
    }
}

/// Borra el RFC en caso de existir y emite un código 200. Si el RFC no existe emite un código 404.
/// En otro caso despliega un error 500.
fn borrar_rfc(data: Received, bd: &Connection) -> Result<HttpResponse, actix_web::Error> {
    let rfc = &data.rfc;
    match exist_rfc(&bd, format!("{}", rfc)) {
        // 1 si el RFC existe en la bd y 0 si no.
        Respuesta::Exito(filas) => {
            if filas == 1 {
                delete_rfc(&bd, data);
                Ok(HttpResponse::Ok().body("El RFC se borró exitosamente."))
            } else {
                Ok(HttpResponse::NotFound().body("El RFC no existe."))
            }
        }
        Respuesta::Error500(err500) => Ok(err500),
    }
}

use serde_derive::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ServerData {
    pub connection: Arc<Mutex<Connection>>
}

#[derive(Deserialize, Serialize)]
pub struct Received {
    rfc: String,
    fecha: String,
    modificador: String,
}
/// Comparte `Mutex<Connection>` con la función ```agregar_rfc``` y la ejecuta.
/// Toma el Mutex de la conexión del thread principal,
/// si no está *envenenado* (poisoned) lo comparte con la función ```agregar_rfc``` y la ejecuta.
/// En otro caso, devuelve un error 500.  
// Función para insertar RFCs a la tabla de protegidos
pub async fn add_rfc(
    received: web::Json<Received>,
    //rfc: web::Path<String>,
    bd: web::Data<ServerData>
) -> Result<HttpResponse, actix_web::Error> {
    let web::Json(data): web::Json<Received> = received;
    //let bd = Arc::new(Mutex::new(super::bd::crea_bd()));

    //let x = bd.lock();
    //let rfc = &dato.rfc;
    match bd.connection.lock() {
        Ok(bd) =>
        //Ok(HttpResponse::Ok().body("bien")),
        {
            anadir_rfc(data, &bd)
        }
        _ => Ok(error500()),
    }

    // Ok(HttpResponse::Ok()
    //     .append_header(("Content-Type", "application/json"))
    //     .body(format!("{}/{}/{}", dato.rfc, dato.fecha, dato.modificador)))
}

/// Comparte `Mutex<Connection>` con la función ```agregar_rfc``` y la ejecuta.
/// Toma el Mutex de la conexión del thread principal,
/// si no está *envenenado* (poisoned) lo comparte con la función ```borrar_rfc``` y la ejecuta.
/// En otro caso, devuelve un error 500.
// Función para borrar RFCs a la tabla de protegidos
pub async fn remove_rfc(
    received: web::Json<Received>, //rfc: web::Path<String>,
    bd: web::Data<ServerData>
) -> Result<HttpResponse, actix_web::Error> {
    let web::Json(data): web::Json<Received> = received;
    //let bd = web::Data::new(Arc::new(Mutex::new(super::bd::crea_bd())));

   // let x = bd.lock();
    match bd.connection.lock() {
        Ok(bd) => borrar_rfc(data, &bd),
        Err(_) => Ok(error500()),
    }
}
