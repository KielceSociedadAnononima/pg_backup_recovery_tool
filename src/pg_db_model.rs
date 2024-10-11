use std::{ fs::File, io::{self, BufReader, Error, Read}};
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize)]
pub struct PostgresDB {
    pub port:i32,
    pub folder_instance: String,
    pub folder_output: String,
    pub deledbname: Vec<String>
}


///Implementacion estandar para el modelo PostgresDB
impl PostgresDB {
    #[allow(dead_code)]
    pub fn load_from_json(path:&str)-> Result<PostgresDB, Error>
    {
        let archivo = match File::open(path) {
            Ok(data) => data,
            Err(error) =>{
               let err = error.to_string();
               return Err(io::Error::new(io::ErrorKind::NotFound,err)) 
            } 
        };
        let mut buffer = BufReader::new(archivo);
        let mut data = String::new();
        if let Err(bad) = buffer.read_to_string(&mut data) {
            return Err(io::Error::new(io::ErrorKind::NotFound,bad)) 
        }
        let obj : PostgresDB = serde_json::from_str(&data).unwrap();        
        Ok(obj)
    }
}