use clap::{command, Arg, ArgAction, Command};

mod backup_postgres;
mod pg_db_model;

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
