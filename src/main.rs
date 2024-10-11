use clap::{command, Arg, ArgAction, Command};
use pg_db_model::PostgresDB;
use restore_psql::restaurar_base_datos;


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
        .arg(
            Arg::new("archivo")
            .action(ArgAction::Append)
            .help("Proveer la ruta del archivo sql que se genero durante el respaldo.")
            .required(true)
        )
    )
    .about("PTechSoftware Tool para Respaldar/Restaurar Modulo Local de Rondanet Kielce S.A.")
    .author("Ignacio Perez Panizza")
    .get_matches();

    /**************************************************************************** */
    /***********************  Redirecciono a la funcion  ************************ */
    /**************************************************************************** */
    match opciones.subcommand() {
        Some(("backup", sub_m)) => {
            if let Some(_ruta) = sub_m.get_one::<String>("config") {
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
            if let (Some(_ruta), Some(_respaldo)) = (sub_m.get_one::<String>("config"), sub_m.get_one::<String>("archivo")) {
                match PostgresDB::load_from_json(_ruta) {
                    Ok(model) =>{
                        restaurar_base_datos(model, &_respaldo);
                    },
                    Err(_wrong)=>{
                        eprintln!("{:?}",_wrong);
                    }
                }
            }else {
                eprintln!("Ingrese un parametro valido. Corra el programa con -h para ver las opciones.");
            }
        }
        _ => {
            eprintln!("Ingrese un parametro valido. Corra el programa con -h para ver las opciones.");
        }
    }

}
