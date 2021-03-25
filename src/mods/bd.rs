extern crate rusqlite;
use rusqlite::{params, Connection, Result};
//use rusqlite::NO_PARAMS; se puede sustituir por params![]
use std::env;
use std::path::PathBuf;

/// Esta función hace ```panic!``` del programa a menos que el nombre de la tabla sea ```rfc_noa```,  
/// el nombre de la columna sea ```rfc``` y ésta última admita un tipo de dato compatible con `String` de Rust
// Ejecuta sentencias SQL para ver si los nombres de la tabla y columna de la BD coinciden
// con los que están harcodeados en las funciones insert_rfc/delete_rfc.
pub fn verificar(bd: &Connection) {
    // Esta consulta es exitosa si los nombres de la tabla y columna son adecuados.
    let contar: Result<i32> = bd.query_row("SELECT COUNT(rfc) FROM rfc_noa", params![], |row| {
        row.get(0)
    });

    match contar {
        Ok(filas) => {
            // La inserción de un "rfc" de prueba (un String) permite saber si el tipo de dato de la
            // columna de la tabla es compatible. Si no, se hace panic!
            match bd.execute(
                "INSERT INTO rfc_noa (rfc, fecha, modificador) VALUES (?1, ?2, ?3)",
                params!["Alma".to_string(), "02/02/02".to_string(), 3],
            ) {
                Ok(_) => {
                    bd.execute(
                        "DELETE FROM rfc_noa WHERE rfc = ?1",
                        params!["Alma".to_string()],
                    )
                    .unwrap();

                    println!("Base de datos lista para ser modificada. filas: {}", filas);
                }
                _ => {
                    panic!("Verifique el tipo de dato en la columna de RFCs.");
                }
            }
        }
        _ => {
            panic!("Modifique el nombre de la tabla o columna de la base de datos.");
        }
    }
}

/// Crea una tabla llamada `rfc_noa` con un columna llamada `rfc` y tipo de dato VARCHAR.
/// Además, inserta los RFCs no permitidos de todos los que están hardcodeados en un `HashSet`.
fn crea_tabla(bd: &Connection) {
    /*
        Se debería poder verificar si el RFC a insertar tiene el formato adecuado
    */

    match bd.execute(
        // tabla de RFCs no autorizados
        "CREATE TABLE IF NOT EXISTS rfc_noa (
            id_rfc      INTEGER PRIMARY KEY AUTOINCREMENT,
            rfc         VARCHAR(13) NOT NULL,
            fecha       TEXT NOT NULL,
            modificador INTEGER
        )",
        params![],
    ) {
        Ok(_tabla) => {
            println!("La creación de la tabla fue exitosa.")
            // for contrib in super::hashset::hash_rfc() {
            //     if contrib.stat {
            //         match bd.execute("INSERT INTO rfc_noa (rfc) VALUES (?1)",
            //         params![contrib.rfc]) {
            //             Ok(_insert) => {
            //                 println!();
            //                 println!("Inserción exitosa de \n{:#?}", contrib);
            //             },
            //             _ => {
            //                 panic!("Imposible insertar {:#?}.", contrib);
            //             }
            //         };
            //     }
            // }
        },
        _ => {
            panic!("Falló la creación de la tabla rfc_noa.")
        }
    };

    
}

#[doc(hidden)]
fn diag(path_dir: &String) -> String {
    let rev = path_dir.chars().rev().collect::<String>();

    if rev[0..1] == *"/" {
        format!("{}bd_rfc", path_dir)
    } else {
        format!("{}/bd_rfc", path_dir)
    }
}

/// Establece la conexión de la base de datos en la ruta por defecto.
fn conect_default(default_file: String) -> Connection {
    let bd = Connection::open(&default_file).unwrap();
    println!(
        "Conexión establecida en la ruta por defecto: {}.",
        default_file
    );
    crea_tabla(&bd);
    bd
}

/// Crea la conexión de la BBDD en ```path_file``` en caso de no necesitar permisos de administrador para esa ruta.
/// En caso contrario, establece la conexión en ```default_file```
fn permiso_ruta(f: fn(&Connection), path_file: String, default_file: String) -> Connection {
    match Connection::open(&path_file) {
        // ruta permitida
        Ok(bd) => {
            println!("Conexión exitosa de la BD en {}.", path_file);
            f(&bd);
            bd
        }
        _ => {
            // ruta no permitida. Preguntar si es mejor usar la ruta por defecto, o hacer panic!
            println!("No se pudo conectar en {}.", path_file);
            conect_default(default_file)
        }
    }
}

/// Verifica que la ruta del directorio que guarda `PATHBD` sea válida.
// Crea la BBDD en la ruta especificada si es válida, si no en una ruta por default
pub fn crea_bd() -> Connection {
    
    //let default_file = String::from("/home/mapa9653/Escritorio/sat/bd_rfc");
    let default_file = String::from("/home/gohe95av/Escritorio/sat/bd_rfc");

    // Leer la variable de entorno.
    match env::var("PATHBD") {
        Ok(path_dir) => {
            // la variable de entorno se pudo leer
            println!("Lectura exitosa de la variable de entorno.");

            // Se verifica si la var. de entorno guarda una ruta válida para el directorio.
            if PathBuf::from(&path_dir).as_path().exists() {
                println!("Directorio válido.");

                // Se define la ruta completa del archivo de la BD concatenando path_dir con el nombre del archivo
                let path_file = diag(&path_dir);

                // Se verifica si la ruta completa (con archivo) es válida (el archivo ya existe).
                if PathBuf::from(&path_file).as_path().exists() {
                    permiso_ruta(verificar, path_file, default_file)
                } else {
                    // Ruta completa no válida (el archivo aún no existe).
                    permiso_ruta(crea_tabla, path_file, default_file)
                }
            } else {
                println!("Directorio no válido.");
                conect_default(default_file)
            }
        }
        Err(_) => {
            // La variable de entorno NO se pudo leer (por ejemplo cuando no existe).
            println!("Imposible leer la variable de entorno.");
            conect_default(default_file)
        }
    }
}