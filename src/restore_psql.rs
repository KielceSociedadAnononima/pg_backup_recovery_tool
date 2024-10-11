use std::{path::{Path, PathBuf}, process::{exit, Command}};

use crate::pg_db_model::PostgresDB;




#[allow(dead_code)]
pub fn restaurar_base_datos(model_db: PostgresDB,ruta_file : &PathBuf)
{   
    let ruta = Path::new(model_db.folder_instance.as_str());
    let psql = &ruta.join("psql.exe");
    //Borrar las bases de datos que estan registradas    
    for db_name in &model_db.deledbname {

        let query_sql = format!("DROP DATABASE {};",db_name);
        match  Command::new(&psql)
        .arg("-U") // Usuario de PostgreSQL
        .arg("postgres")
        .arg("-p")
        .arg(model_db.port.to_string())
        .arg("-c") // Comando a ejecutar
        .arg(query_sql)
        .spawn()
        {
            Ok(_good) =>{
                println!(r#"Schema {db_name} eliminado con correctamente"#)
            },
            Err(_wrong) => {
                eprintln!(r#"Fallo el borrado del schema {db_name} La base de datos esta en un estado inconsistente. Busque soporte para repararla."#);
                exit(1)
            }
            
        }         
    }
    //Mando el commando de restore

    match Command::new(&psql)
        .arg("-U")
        .arg("postgres")
        .arg("-p")
        .arg(model_db.port.to_string())
        .arg("-f")
        .arg(ruta_file)
        .spawn() {
        Ok(_good) =>{
            println!(r#"Base de datos restaurada correctamente"#)
        },
        Err(_wrong) => {
            eprintln!(r#"Fallo la restauracion de la base de datos, por lo que esta en un estado inconsistente. Busque soporte para repararla."#);
            exit(1)
        }
    }

}