use actix_web::{web, HttpResponse};
use regex::Regex;
use std::path::PathBuf;

use rusqlite::Result;

/// Verifica si `filename` tiene extensión. En ese caso, despliega un `HttpResponse`
/// indicando si dicho archivo existe en la carpeta static o no.
/// En otro caso, despliega un archivo html.
pub async fn index1(filename: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    // Ruta del archivo filename en la carpeta static.
    let path: PathBuf = PathBuf::from(format!(
        //"/home/mapa9653/Escritorio/sat/Kixtia/static/{}",
        "/home/mapa9653/Escritorio/sat/Kixtia/static/{}",
        filename
    ));

    // Expresión regular para archivos con extensión.
    let re = Regex::new(r"^(.*)\.(.*)$").unwrap();
    // Verifica si filename tiene el formato de la expresión regular
    if re.is_match(filename.as_str()) {
        if path.exists() {
            Ok(HttpResponse::Ok().body("Archivo existente."))
        } else {
            Ok(HttpResponse::NotFound().body("Archivo inexistente."))
        }
    } else {
        // Despliega el contenido de index.html con el código 200.
        Ok(HttpResponse::Ok().body(include_str!(
            //"/home/mapa9653/Escritorio/sat/kixtia/src/index.html"
            "/home/mapa9653/Escritorio/sat/kixtia/src/index.html"
        )))
    }
}

/*

archivo------existe--------hacemos la conexion--------no existe tabla-------crear tabla-----endpoints
|                                   |
|                                   |
|                                   |
no existe                       existe tabla---------nombre tabla/col diferente-------panic: renombrar
|                                   |
|                                   |
|                                   |
hacer la conexion              nombre tabla/col adecuados                    |
y crear la tabla                    |
|                                endpoints
|
|
endpoints

*/
