extern crate clap; // Para implementar las flags
use clap::{App, Arg};
use std::path::PathBuf;

/// Define las opciones o *banderas* `-s, -q` y `-l`.
///
/// * **-s** : para personalizar el socket donde se desplegará el servidor.
/// Si la dirección no es válida, el programa se cerrará.
/// * **-q**: para decidir si la bitácora se imprimirá en `stdout` o no.
/// Por defecto la bitácora no se imprimirá, lo cual se puede indicar de manera
/// explícita con el valor "`N`". Si ésta se desea mostrar bastará con ingresar "`Y`".
/// * **-l**: para decidir la ruta donde se almacenará el archivo de la bitácora. Si ésta no
/// se indica el archivo se guardará en una ruta por defecto.

// Función que define las --option -o
pub fn options() -> App<'static, 'static> {
    App::new("Kixtia options")
        .version("1.0")
        .author("Erick Gómez/Alma Maguey")
        .about("")
        .arg(
            Arg::with_name("socket") // --socket option
                .short("s")
                .long("socket")
                .help("Sets a custom socket")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log-path") // --log-path option
                .short("l")
                .long("log-path")
                .help("Sets a log-path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiet") // --quiet option
                .short("q")
                .long("quiet")
                .help("Does not print to stdout")
                .takes_value(true),
        )
}

/// Toma los valores ingresados a través de las opciones `-s , -q`  y `-l`.
/// Genera una tupla que contiene el socket (`-s`) y un gestor de bitácoras (`-q` y `-l`).
// Devuelve una tupla con el socket address y el logger
pub fn option_settings() -> (String, super::logger::KixtiaLogger) {
    // Hace el parse de lo que se recibe en las option
    let matches = options().get_matches();

    // Variable para imprimir o no la bitácora en la terminal
    let mostrar: bool = match matches.value_of("quiet").unwrap_or("N") {
        "Y" => true,
        "N" => false,
        _ => panic!("¡Eureka! Ingrese Y o N. \n"),
    };

    // path para guardar el archivo de la bitácora
    let log_path = Some(PathBuf::from(
        matches
            .value_of("log-path")
            //.unwrap_or("/home/mapa9653/Escritorio/sat/"),
            .unwrap_or("/home/mapa9653/Escritorio/sat/"),
    ));

    (
        // return de la tupla
        String::from(matches.value_of("socket").unwrap_or("localhost:8080")), // socket address
        super::logger::KixtiaLogger::new(mostrar, log_path), // para instanciar el logger.
    )
}
