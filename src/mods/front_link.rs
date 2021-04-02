extern crate actix_rt;
extern crate actix_web;

use actix_web::{HttpRequest, HttpResponse, http};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

async fn load_file(path: PathBuf) -> HttpResponse {
    match File::open(&path) {
        Ok(mut file) => {
            let mut content = Vec::new();
            match file.read_to_end(&mut content) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy();
                let ext = match ext.as_ref() {
                    "js" => "application/javascript",
                    "map" => "application/json",
                    "ico" => "image/x-icon",
                    "json" => "application/json",
                    "html" => "text/html",
                    "txt" => "text/plain",
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "svg" => "image/svg+xml",
                    _ => "application/octet-stream",
                };
                HttpResponse::Ok()
                    //.append_header(("Content-Type", ext))
                    .header(http::header::CONTENT_TYPE, ext)
                    .body(content)
            } else {
                HttpResponse::Ok()
                   // .append_header(("Content-Type", "application/octet-stream"))
                    .header(http::header::CONTENT_TYPE, "application/octet-stream")
                    .body(content)
            }
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn index_front(req: HttpRequest) -> HttpResponse {
    let path: PathBuf = [
        PathBuf::from("./front/build"),
        PathBuf::from(match req.uri().path().strip_prefix("/") {
            Some(v) => v,
            None => req.uri().path(),
        }),
    ]
    .iter()
    .collect();
    if path.extension().is_some() {
        load_file(path).await
    } else {
        // No extension, then we hand in html
        HttpResponse::Ok()
            //.append_header(("Content-Type", "text/html"))
            .header(http::header::CONTENT_TYPE, "text/html")
            .body(include_str!("../../front/public/index.html"))
        //.body("kestapanda")
    }
}
