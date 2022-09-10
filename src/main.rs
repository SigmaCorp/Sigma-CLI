mod handlers;
mod subcommands;
use clap::{Parser, Subcommand};
use std::collections::HashMap;

/// Sigma-CLI es una interfaz por consola que permite utilizar los endpoints de busqueda de datos Sigma a través de una terminal.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login(subcommands::Login),
    BuscarDNIStandard(subcommands::BuscarDNIStandard),
    BuscarCelularesDNI(subcommands::BuscarCelualaresDNI),
    BuscarPatente(subcommands::BuscarPatente),
    BuscarPatenteDNI(subcommands::BuscarPatenteDNI),
    BuscarLeaks(subcommands::BuscarLeaks),
    BuscarDNIProfesional(subcommands::BuscarDNIProfesional),
    BuscarNombre(subcommands::BuscarNombre),
    BuscarMovistar(subcommands::BuscarMovistar),
    BuscarVecinos(subcommands::BuscarVecinos),
    BuscarCelular(subcommands::BuscarCelular),
    BuscarCelularesMagic(subcommands::BuscarCelularesMagic),
    BuscarCBU(subcommands::BuscarCBU),
    BuscarEmail(subcommands::BuscarEmail),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::BuscarDNIStandard(dni) => {
            handlers::handle_buscar_dni(&dni.dni).await;
        }
        Commands::BuscarCelularesDNI(dni) => {
            handlers::handle_buscar_celulares_dni(&dni.dni).await;
        }
        Commands::BuscarPatente(patente) => {
            handlers::handle_buscar_patentes(&patente.patente, true).await;
        }
        Commands::BuscarPatenteDNI(dni) => {
            handlers::handle_buscar_patentes(&dni.dni, false).await;
        }

        Commands::BuscarLeaks(query) => {
            handlers::handle_buscar_leaks(&query.query).await;
        }

        Commands::BuscarDNIProfesional(input) => {
            handlers::handle_buscar_dni_profesional(&input.dni, &input.genero).await;
        }

        Commands::BuscarNombre(input) => {
            let mut params = HashMap::new();

            if input.provincia.is_some() {
                params.insert(
                    "provincia_nombre",
                    input.provincia.as_ref().unwrap().to_string(),
                );
            }

            if input.localidad.is_some() {
                params.insert("localidad", input.localidad.as_ref().unwrap().to_string());
            }

            if input.edadmin.is_some() {
                params.insert("edad_desde", input.edadmin.as_ref().unwrap().to_string());
            }

            if input.edadmax.is_some() {
                params.insert("edad_hasta", input.edadmax.as_ref().unwrap().to_string());
            }

            handlers::handle_buscar_nombre(&input.nombre, params).await;
        }

        Commands::BuscarMovistar(num) => {
            handlers::handle_buscar_movistar(&num.numero).await;
        }

        Commands::BuscarVecinos(direccion) => {
            handlers::handle_buscar_vecinos(&direccion.direccion).await;
        }

        Commands::BuscarCelular(num) => {
            handlers::handle_buscar_celular(&num.numero).await;
        }

        Commands::BuscarCelularesMagic(num) => {
            handlers::handle_buscar_celular_magic(&num.numero).await;
        }

        Commands::BuscarCBU(cbu_alias) => {
            handlers::handle_buscar_cbu(&cbu_alias.cbu).await;
        }

        Commands::BuscarEmail(email) => {
            handlers::handle_buscar_email(&email.email).await;
        }

        Commands::Login(creds) => {
            handlers::handle_login(&creds.usuario, &creds.password).await;
        }
    }
}
