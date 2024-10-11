
use std::{path::Path, process::{exit, Command, Stdio}};

use crate::pg_db_model::PostgresDB;

#[allow(dead_code)]
///Ejecuta un commando sobre la instancia de Postgres definida en el model, pra generar una copia de seguridad.
pub fn generate_dump_all(model_db: PostgresDB)
{
    let ruta = Path::new(model_db.folder_instance.as_str());
    let j1 = &ruta.join("pg_dumpall.exe");

    let output = Path::new(model_db.folder_output.as_str()).join("respladomodulo.sql");
    match Command::new(j1.to_str().unwrap())
        .arg("-U")
        .arg("postgres")
        .arg("-p")
        .arg(model_db.port.to_string())
        .arg("-f")
        .arg(output.to_str().unwrap())
        .stderr(Stdio::inherit())
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn() {
        Ok(mut res) => {

            match res.wait() {
                Ok(_good) =>{
                    if let Some(ext_code) = _good.code() {
                        exit(ext_code)
                    }
                    exit(1)
                },
                Err(bad) => {
                    eprintln!("Error func chid arms{:?}",bad);
                    exit(1)
                }
            }
        },
        Err(err) => {
            eprintln!("Error command backup fail:{:?}",err);
            exit(1)
        }
    }
}