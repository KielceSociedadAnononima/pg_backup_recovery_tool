
use std::process::{exit, Command};
use crate::pg_db_model::PostgresDB;

#[allow(dead_code)]
pub fn generate_dump_all(model_db: PostgresDB)
{
    let program = format!("{}\\{}",model_db.folder_instance, "pg_dumpall.exe");
    match Command::new(program)
        .arg("-U")
        .arg("postgres")
        .arg("-P")
        .arg(model_db.port.to_string())
        .arg("-f")
        .arg(format!("{}.respaldo_modulo.sql",model_db.folder_output))
        .spawn() {
        Ok(mut res) => {
            let estado = res.wait();
            if estado.is_ok() {
                exit(0)
            }
            exit(1)
        },
        Err(err) => {
            eprint!("{:?}",err);
            exit(1)
        }
    }
}