//#![crate_name = "Kixtia"]
//! Despliega un servidor con los endpoints:
//! * **/add/{rfc}** : permite agregar RFCs a una BBDD.
//! * **/remove/{rfc}** : permite borrar RFCs de una BBDD.
//! * **/allowed/{rfc}** : permite consultar el status de los RFCs que están hardcodeados en un ```HashSet```.
//! * **/arch/{filename}** : si `filename` contiene extensión verifica la existencia de dicho archivo,
//! si no la tiene, despliga un archivo html en el servidor.
//!
//! Se establecerá la conexión a la BBDD mencionada especificando el directorio en el que se encuentra
//! o bien, en el que se desea crear. La ruta del directorio debe guardarse en la variable de entorno ```PATHBD```.
//! Dicha variable deberá cumplir con el siguiente formato: `/home/ruta/de/ejemplo/` (la última diagonal puede omitirse).
//! Si esto no es posible en dicha ruta, esto sucederá en una ruta por defecto.
//!
//! Si se desea conectar a una BBDD existente, el archivo deberá llamarse `bd_rfc.sqlite3`. De lo contrario,
//! el programa creará un archivo nuevo con ese nombre. Adicionalmente, para evitar errores con las sentencias de SQLite,
//! la BBDD existente debe tener una tabla llamada `rfc_noa` con una única columna llamada `rfc`. Esta columna admite
//!  únicamente los tipos de datos: CHAR, VARCHAR y TEXT.
//! El sistema no se ejecutará si no se satisfacen las condiciones anteriormente mencionadas.

extern crate lazy_static;
extern crate rusqlite;
//use rusqlite::{params, Connection, Result};
extern crate actix_rt;
extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use log::LevelFilter;

use std::sync::{Arc, Mutex};
use std::{env, thread};

mod mods; // módulos
use mods::add_remove::{add_rfc, remove_rfc};
use mods::bd::crea_bd;
use mods::front_link::index_front;
use mods::hashset::rfc_protegido; // submódulo para validad si el RFC está protegido
use mods::logger::KixtiaLogger; // submódulo para el logger Kixtia
use mods::options::option_settings;
use mods::stat_serv::index1;
use mods::protected::protected;

use mods::add_remove::ServerData;


use serde::{Deserialize, Serialize};

/// Despliega el servidor
#[actix_rt::main]
async fn main() {
    //-> std::io::Result<()> {
    // Se establece la conexión por única vez para ser compartida entre las funciones
    // asíncronas de los endpoints.
    let conexion : ServerData = ServerData {
        connection: Arc::new(Mutex::new(crea_bd()))
    };
    // Acceder la dirección del socket
    //let socket = option_settings().0;
    // // Acceder al log path para guardar las bitácoras
    //lazy_static::lazy_static! { // Inicializar el logger "perezoso" (referencia estática)
    //   static ref LOGGER: KixtiaLogger = option_settings().1;
    //}

    // log::set_logger(&(*LOGGER))
    //     .map(|()| log::set_max_level(LevelFilter::Info))
    //     .unwrap();

    // Mensaje que se va a imprimir en el servidor
    // static MJE: &str = "HOLA... ADIOS u:";
    // Servidor

    HttpServer::new(
        move || {
            App::new()
                // conexion.clone() va a estar disponible para los services
                .data(conexion.clone())
                // endpoint para agregar un rfc a la tabla de protegidos
                .service(web::resource("/add/rfc").route(web::post().to(add_rfc)))
                // endpoint para borrar un rfc a la tabla de protegidos
                .service(web::resource("/remove/rfc").route(web::post().to(remove_rfc)))
                // endopoint para buscar archivos en la carpeta static
                //.service(web::resource("/archivo/{filename}").route(web::get().to(index)))
                .service(web::resource("/protected").route(web::get().to(protected)))
                // Despliega index.html para rutas no especificadas.
                .default_service(web::route().to(index_front))
        }, //web::post().to(|| {
           //HttpResponse::Ok()
           //.body(include_str!(
           //"/home/mapa9653/Escritorio/sat/kixtia/src/index.html"
           //  "./index.html"
           //))
           // }))
           // endpoint que permite saber si el rfc está protegido o no
           //.service(web::resource("/allowed/{rfc}").route(web::get().to(rfc_protegido)))
           // Ventana inicial del servidor
           //.route("/", web::to(|| HttpResponse::Ok().body(MJE)))
    )
    .bind("localhost:8091")
    .unwrap()
    //.bind(socket)?
    .run()
    .await
    .unwrap();
}

// ----------- PREGUNTAS -----------
/*
    - ¿Cuáles son los threads que actix utliza para atender llamadas HTTP y
    entre las que debemos compartir la estructura de datos (HashSet)?

*/

// #[get("/a/{name}")]
// async fn index(obj: web::Path<MyObj>) -> Result<HttpResponse, actix_web::Error> {
//     Ok(HttpResponse::Ok()

//         rfc: "gohe",
//     }))
// }
