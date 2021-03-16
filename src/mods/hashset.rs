use actix_web::{web, HttpResponse, Result};
use std::collections::HashSet;

/// Esta estructura permite almacenar el RFC de un
/// contribuyente así como su status (autorizado o no autorizado).
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Contrib {
    /// RFC del contribueynte.
    pub rfc: String,
    /// `true` indica que el `rfc` está protegido y `false` indica que no lo está.
    pub stat: bool,
}

impl Contrib {
    /// Para instanciar un struct `Contrib`.
    pub fn new(rfc: String, stat: bool) -> Contrib {
        Contrib { rfc, stat }
    }
}

/// Regresa un `HashSet<Contrib>` con algunos RFCs hardcodeados con un status.
// true: RFC protegido (no autorizado); false: RFC no protegido (autorizado).
pub fn hash_rfc() -> HashSet<Contrib> {
    // HashSet
    let mut datos_rfc = HashSet::new();

    datos_rfc.insert(Contrib {
        rfc: "GOHE951031B16".to_string(),
        stat: true,
    });
    datos_rfc.insert(Contrib {
        rfc: "MAPA960503IV8".to_string(),
        stat: true,
    });
    datos_rfc.insert(Contrib {
        rfc: "A".to_string(),
        stat: false,
    });
    datos_rfc.insert(Contrib {
        rfc: "B".to_string(),
        stat: false,
    });
    datos_rfc.insert(Contrib {
        rfc: "C".to_string(),
        stat: true,
    });
    datos_rfc.insert(Contrib {
        rfc: "D".to_string(),
        stat: false,
    });
    datos_rfc.insert(Contrib {
        rfc: "E".to_string(),
        stat: true,
    });
    datos_rfc.insert(Contrib {
        rfc: "F".to_string(),
        stat: false,
    });
    datos_rfc
}

/// Devuelve `Some(bool)` si el RFC está hardcodeado en el `HashSet`
/// (donde el booleano indica el status del RFC). Si no, devuelve `None`.
pub fn consulta(rfc: String) -> Option<bool> {
    let c1 = Contrib::new(rfc.clone(), true);
    let c2 = Contrib::new(rfc.clone(), false);

    if hash_rfc().contains(&c1) {
        Some(c1.stat)
    } else if hash_rfc().contains(&c2) {
        Some(c2.stat)
    } else {
        None
    }
}

/// Responde si el RFC ingresado en el endpoint `/allowed/{rfc}` está protegido (o no) con el
/// código 200 y con el código 400 si el RFC no existe.
pub async fn rfc_protegido(info: web::Path<String>) -> Result<HttpResponse> {
    match consulta(info.to_string()) {
        Some(t) => {
            if t {
                Ok(HttpResponse::Ok().body(format!("El RFC {} está protegido.", info)))
            } else {
                Ok(HttpResponse::Ok().body(format!("El RFC {} no está protegido.", info)))
            }
        }
        None => Ok(HttpResponse::NotFound().body(format!("No se encontró {}.", info))),
    }
}
