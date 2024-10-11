use clap::{command, Arg, ArgAction, Command};
use pg_db_model::PostgresDB;


//Importacion de modulos
mod backup_postgres;
mod pg_db_model;
mod restore_psql;

fn main()
{
    /**************************************************************************** */
    /***********************  Evaluo los comandos de la cli  ******************** */
    /**************************************************************************** */
    //
    let opciones = command!()
    .subcommand( 
        Command::new("backup")
        .about("Genera un dump all de la base de datos.")
        .short_flag('b')
        .arg(
            Arg::new("config")
                .action(ArgAction::Append)
                .help("Proveer un Json con el modelo de datos, indicando puerto, carpeta de la instanica de BBDD, y ruta de salida.")
                .required(true)

        )
    )
    .subcommand(
        Command::new("restore")
        .about("Restaura un dump all desde el archivo. Elimina las dos bases de produccion antes de restaurar.EL MODULO LOCAL DEBE ESTAR CERRADO.")
        .short_flag('r')
        .arg(
            Arg::new("config")
            .action(ArgAction::Append)
            .help("Proveer un Json con el modelo de datos, indicando puerto, carpeta de la instanica de BBDD, y ruta de salida.")
            .required(true)
        )
    )
    .get_matches();

    /**************************************************************************** */
    /***********************  Redirecciono a la funcion  ************************ */
    /**************************************************************************** */
    match opciones.subcommand() {
        Some(("backup", sub_m)) => {
            if let Some(_ruta) = sub_m.get_one::<String>("config"){
                match PostgresDB::load_from_json(_ruta) {
                    Ok(model) =>{
                        backup_postgres::generate_dump_all(model);
                    },
                    Err(_wrong)=>{
                        eprintln!("{:?}",_wrong);
                    }
                }
            }else {
                eprintln!("Ingrese un parametro valido. Corra el programa con -h para ver las opciones.");
            }
        }
        Some(("restore", sub_m)) => {
            if let Some(_ruta) = sub_m.get_one::<String>("config"){
            }else {
                eprintln!("Ingrese un parametro valido. Corra el programa con -h para ver las opciones.");
            }
        }
        _ => {
            eprintln!("Ingrese un parametro valido. Corra el programa con -h para ver las opciones.");
        }
    }

}
