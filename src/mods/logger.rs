extern crate log;
extern crate chrono;
extern crate regex;

use regex::Regex;
use log::{Log, Record, Level, Metadata};
use std::fs::{File, OpenOptions, read_dir};
use std::path::PathBuf;
use std::io::{prelude::*, BufReader};

// Para la comunicación con el escritor de archivos
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::thread;

// Para imprimir el tiempo adecuadamente
use chrono::prelude::*;

/// Estructura que escribe a los archivos de bitácora.
pub struct LogWriter {
    /// Dirección en que deben generarse los archivos
    pub log_path: PathBuf,
    /// Número de líneas escritas al archivo actual
    pub line_count: u64,
    /// Momento en que debe cambiarse de archivo de bitácora
    pub line_maximum: u64,
    /// Número de la bitácora actual
    pub current_log_file_number: i64,
    /// Archivo actual de la bitácora
    pub log_file: File,
    //pub log_file: BufWriter<File>,
    /// Uso interno, para comunicación entre las llamadas log, warn y error con esta estructura
    pub rx: Receiver<String>
}

impl LogWriter {
    // Genera el archivo de bitácora.
    pub fn new(log_path: PathBuf, rx: Receiver<String>, line_maximum: u64) -> LogWriter {
        if !log_path.as_path().exists() {
            panic!("Log folder does not exist!");
        }

        let reg = Regex::new(r"(?x)^trace-(?P<number>[0-9]+)\.log$").unwrap();
        // We look for the current log file number.
        let dir_iter = match read_dir(&log_path) {
            Ok(v) => v,
            Err(_) => panic!("Could not iterate through files in log dir")
        };
        let mut current_log_file_number: i64 = -1;
        for dir in dir_iter {
            match dir {
                Ok(v) => {
                    let filename = String::from(v.file_name().to_string_lossy());
                    match reg.captures(&filename).and_then(|cap| {
                        cap.name("number").map(|number| number.as_str().parse::<i64>().unwrap())
                    }) {
                        Some(v) => {
                            if v > current_log_file_number {
                                current_log_file_number = v;
                            }
                        },
                        None => ()
                    };
                },
                Err(_) => panic!("Could not see file inside log dir")
            };
        }
        // We will check the number of lines of the current log file.
        let (line_count, log_file_path) = if current_log_file_number == -1 {
            // No hay bitácora aún
            current_log_file_number += 1;
            let mut file_path = log_path.clone();
            file_path.push(format!("trace-{}.log", current_log_file_number));
            (0, file_path.clone())
        } else {
            let mut log_file_path = log_path.clone();
            log_file_path.push(format!("trace-{}.log", current_log_file_number));
            match File::open(&log_file_path) {
                Ok(f) => {
                    let mut curr_line_count = 0;
                    for _line in BufReader::new(f).lines() {
                        curr_line_count += 1;
                    }
                    // Finally, we check how many lines it has
                    if curr_line_count < line_maximum {
                        (curr_line_count, log_file_path.clone())
                    } else {
                        // Time to create a new file
                        current_log_file_number += 1;
                        let mut file_path = log_path.clone();
                        file_path.push(format!("trace-{}.log", current_log_file_number));
                        (0, file_path.clone())
                    }
                },
                Err(e) => {
                    panic!("Could not count file lines: {}", e);
                }
            }
        };
        
        // With the chosen log path, we continue the log
        let f = if log_file_path.as_path().exists() {
            match OpenOptions::new()
                .write(true)
                .append(true)
                .open(&log_file_path) {
                Ok(v) => v,
                Err(_) => panic!("Imposible sobreescribir la bitácora.")
            }
        } else {
            match File::create(&log_file_path) {
                Ok(v) => v,
                Err(e) => panic!("Could not create log file {}! ({})", log_file_path.as_os_str().to_string_lossy(), e)
            }
        };
        LogWriter{
            log_path,
            //log_file: BufWriter::with_capacity(2000, f),
            log_file: f,
            rx: rx,
            line_count,
            line_maximum,
            current_log_file_number
        }
        //LogWriter{log_file: f, rx: rx}
    }

    pub fn run(&mut self){
        for record in &self.rx {
            match self.log_file.write((record+"\n").as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Could not write to log file: {}", e)
                }
            }
            self.line_count = (self.line_count + 1) % self.line_maximum;
            if self.line_count == 0 {
                // Time to swap logs
                match self.log_file.flush() {
                    Ok(_) => (),
                    Err(_) => panic!("Could not flush contents")
                };
                self.current_log_file_number += 1;
                let mut log_file_path = self.log_path.clone();
                log_file_path.push(format!("trace-{}.log", self.current_log_file_number));
                self.log_file = match File::create(&log_file_path) {
                    //Ok(v) => BufWriter::with_capacity(2000, v),
                    Ok(v) => v,
                    Err(e) => panic!("Could not create log file {}! ({})", log_file_path.as_os_str().to_string_lossy(), e)
                }
            }
        }
    }
}

/// Estructura que configura las bitácoras
pub struct KixtiaLogger {
    /// Indica si la bitácora debe escribirse a salida estándar
    print_to_term: bool,
    /// Indica si la bitácora deberá escribirse a un archivo
    print_to_file: bool,
    /// Comunicación entre el thread encargado de las bitácoras y el resto de Kixtia
    tx: SyncSender<String>
}

impl Log for KixtiaLogger {
    /// Para indicar qué nivel de bitácora se usará
    fn enabled(&self, metadata: &Metadata) -> bool {
        match metadata.level() {
            Level::Error => true,
            Level::Warn => true,
            Level::Info => true,
            Level::Debug => true,
            Level::Trace => true
        }
    }

    /// Función principal de las bitácoras
    fn log(&self, record: &Record) {
        // Sólo hacemos caso a los mensajes que planeamos guardar en la bitácora
        if self.enabled(record.metadata()) {
            // Ahora sí, escribimos el mensaje.
            let dt = Local::now();//Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
            let registry = match record.level() {
                Level::Error => {
                    format!("{} kixtia[\u{001b}[0;31m{}\u{001b}[0m]: {}", dt.format("%b %e %T"), record.level(), record.args())
                },
                Level::Warn => {
                    format!("{} kixtia[\u{001b}[0;33m{}\u{001b}[0m]: {}", dt.format("%b %e %T"), record.level(), record.args())
                }, 
                Level::Info => {
                    format!("{} kixtia[\u{001b}[0;34m{}\u{001b}[0m]: {}", dt.format("%b %e %T"), record.level(), record.args())
                }
                Level::Debug => {
                    format!("{} kixtia[\u{001b}[0;36m{}\u{001b}[0m]: {}", dt.format("%b %e %T"), record.level(), record.args())
                }
                _ => format!("{} kixtia[{}]: {}", dt.format("%b %e %T"), record.level(), record.args())
            };
            if self.print_to_term {
                println!("{}",&registry);
            }
            if self.print_to_file {
                match self.tx.send(registry) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Cannot write anymore to log file (thread crashed)");
                        panic!("Could not write anymore to the log file");
                    }
                };
            }
        }
    }
    fn flush(&self) {}
}

impl KixtiaLogger {
    /// Genera una nueva instancia del gestor de bitácoras. Los parámetros indican si se
    /// imprimirá la bitácora a `stdout`, y una ruta opcional para depositar las bitácoras.
    pub fn new(print_to_term: bool, log_path: Option<PathBuf>) -> KixtiaLogger {
        if let Some(log_path) = log_path {
            if !log_path.as_path().exists() {
                panic!("Could not find log path ({})", log_path.as_os_str().to_string_lossy());
            }
            // We generate the communication channel
            let (sender, receiver) = sync_channel::<String>(0);
            // This thread will receive all log messages
            thread::spawn(move || {
                LogWriter::new(log_path, receiver, 10_000_000).run()
            });
            KixtiaLogger{print_to_term, print_to_file: true, tx: sender}
        } else {
            let (sender, _) = sync_channel::<String>(0);
            // The sender will anyways never be used
            KixtiaLogger{print_to_term, print_to_file: false, tx: sender}
        }
    }
}