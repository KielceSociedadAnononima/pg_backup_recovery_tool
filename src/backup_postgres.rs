
use std::{path::Path, process::{exit, Command, Stdio}};

use crate::pg_db_model::PostgresDB;

#[allow(dead_code)]
///Ejecuta un commando sobre la instancia de Postgres definida en el model, pra generar una copia de seguridad.
pub fn generate_dump_all(model_db: PostgresDB)
{
    let ruta = Path::new(model_db.folder_instance.as_str());
    let j1 = &ruta.join("pg_dumpall.exe");
    match Command::new(j1.to_str().unwrap())
        .arg("-U")
        .arg("postgres")
        .arg("-p")
        .arg(model_db.port.to_string())
        .arg("-f")
        .arg(format!("{}.respaldo_modulo.sql",model_db.folder_output))
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
                    eprintln!("{:?}",bad);
                    exit(1)
                }
            }
        },
        Err(err) => {
            eprintln!("{:?}",err);
            exit(1)
        }
    }
}