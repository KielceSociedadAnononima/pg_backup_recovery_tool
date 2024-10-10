
use std::process::Command;
use crate::pg_db_model::PostgresDB;

pub fn generate_dump_all(model_db: PostgresDB)
{
    let program = format!("{}\\{}",model_db.folder_instance, "pg_dumpall.exe");
    match Command::new(program)
        .arg("-U postgres")
        .arg(format!("-P {}",model_db.port))
        .arg(format!("-f {}.respaldo_modulo.sql",model_db.folder_output))
        .spawn() {
        Ok(res) => {
            
        },
        Err(err) => {

        }
    }
}