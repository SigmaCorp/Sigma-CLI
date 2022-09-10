extern crate rusigma;
extern crate tabled;
use rusigma::client::SigmaClient;
use std::collections::HashMap;
use std::fs;
use tabled::{builder::Builder, Style};

pub async fn handle_login(username: &String, password: &String) {
    let config_home = match home::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => panic!("Impossible to get your home directory"),
    };

    let mut sclient = SigmaClient::new();
    sclient
        .login_with_credentials(username.to_string(), password.to_string())
        .await
        .expect("Unable to log-in with the given credentials");

    let credentials = format!(
        "{}:{}",
        sclient.http.token_str.unwrap(),
        sclient.http.plan_id.unwrap(),
    );
    fs::write(format!("{}/.sigma.conf", config_home), credentials).expect("Could not save token");
    println!(
        "Estas logueado y tu token ha sido guardada en {}/.sigma.conf",
        config_home
    );
}

pub fn read_credentials() -> Result<String, String> {
    let config_home = match home::home_dir() {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => panic!("Impossible to get your home directory"),
    };
    Ok(fs::read_to_string(format!("{}/.sigma.conf", config_home)).expect("Could not read token"))
}

pub async fn handle_buscar_dni(dni: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_standard_dni(dni.to_string()).await {
        Ok(v) => println!("{:#?}", v),
        Err(e) => println!("Error: {}", e),
    };
}

pub async fn handle_buscar_celulares_dni(dni: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_phones_by_dni(dni.to_string()).await {
        Ok(v) => {
            let mut builder = Builder::default();
            builder.set_columns([
                "Documento",
                "Numero",
                "Nombre",
                "Localidad",
                "Provincia",
                "CP",
                "Empresa",
            ]);
            for pn in v {
                builder.add_record([
                    pn.documento,
                    pn.numero,
                    pn.nombre,
                    pn.localidad,
                    pn.provincia,
                    pn.codigo_postal,
                    pn.empresa,
                ]);
            }

            let table = builder.build().with(Style::rounded());
            println!("{}", table.to_string());
        }
        Err(e) => println!("Error: {}", e),
    };
}

pub async fn handle_buscar_patentes(input: &String, is_plate: bool) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    let response: Result<Vec<rusigma::response::PlateHistory>, rusigma::Error>;
    if is_plate {
        response = client.search_plate(input.to_string()).await;
    } else {
        response = client.search_plate_by_dni(input.to_string()).await;
    }
    match response {
        Ok(v) => {
            let mut builder_p1 = Builder::default();
            let mut builder_p2 = Builder::default();

            builder_p1.set_columns([
                "Patente",
                "Vehiculo",
                "Marca",
                "Año",
                "Porcentaje",
                "Transferencia",
            ]);

            builder_p2.set_columns([
                "Patente",
                "Documento",
                "Titular",
                "Calle",
                "Altura",
                "Piso",
                "Depto",
                "Localidad",
                "CP",
            ]);

            for pn in v {
                let patente = pn.patente.unwrap_or(String::from("-"));
                builder_p1.add_record([
                    patente.clone(),
                    pn.vehiculo.unwrap_or(String::from("-")),
                    pn.marca.unwrap_or(String::from("-")),
                    pn.anio.unwrap_or(String::from("-")),
                    pn.porcentaje.unwrap_or(String::from("-")),
                    pn.transferencia.unwrap_or(String::from("-")),
                ]);

                builder_p2.add_record([
                    patente.clone(),
                    pn.documento.unwrap_or(String::from("-")),
                    pn.titular.unwrap_or(String::from("-")),
                    pn.calle.unwrap_or(String::from("-")),
                    pn.altura.unwrap_or(String::from("-")),
                    pn.piso.unwrap_or(String::from("-")),
                    pn.depto.unwrap_or(String::from("-")),
                    pn.localidad.unwrap_or(String::from("-")),
                    pn.codigo_postal.unwrap_or(String::from("-")),
                ]);
            }

            let table_p1 = builder_p1.build().with(Style::rounded());
            let table_p2 = builder_p2.build().with(Style::rounded());
            println!("{}\n{}", table_p1.to_string(), table_p2.to_string());
        }
        Err(e) => println!("Error: {}", e),
    };
}

pub async fn handle_buscar_leaks(query: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_leaks(query.to_string()).await {
        Ok(r) => {
            let mut builder = Builder::default();
            builder.set_columns(["Email", "Password"]);

            for pw in r {
                builder.add_record([pw.username, pw.password]);
            }

            let table = builder.build().with(Style::rounded());
            println!("{}", table.to_string());
        }
        Err(e) => println!("Error: {}", e),
    };
}

pub async fn handle_buscar_dni_profesional(dni: &String, gender: &usize) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client
        .search_profesional_dni(dni.to_string(), *gender)
        .await
    {
        Ok(r) => {
            let mut builder_datos = Builder::default();
            let mut builder_localidad = Builder::default();
            let mut builder_coberturas = Builder::default();

            builder_datos.set_columns([
                "DNI",
                "Nombres",
                "Apellido",
                "Pais",
                "Fecha Nac.",
                "Fallecido",
                "Edad",
                "CUIT",
                "Emision",
            ]);

            builder_localidad.set_columns([
                "Provincia",
                "Ciudad",
                "Municipio",
                "Barrio",
                "Area",
                "Calle",
                "Altura",
                "Depto",
                "Piso",
                "CP",
            ]);

            builder_coberturas.set_columns(["Cobertura", "Nombre", "DNI", "Sexo"]);

            builder_datos.add_record([
                r.documento.unwrap_or(String::default()),
                r.nombres.unwrap_or(String::default()),
                r.apellido.unwrap_or(String::default()),
                r.pais.unwrap_or(String::default()),
                r.fecha_nacimiento.unwrap_or(String::default()),
                r.fallecido.unwrap_or(String::default()),
                r.edad.unwrap_or(0).to_string(),
                r.cuil.unwrap_or(String::default()),
                r.emision.unwrap_or(String::default()),
            ]);

            builder_localidad.add_record([
                r.provincia.unwrap_or(String::default()),
                r.ciudad.unwrap_or(String::default()),
                r.municipio.unwrap_or(String::default()),
                r.barrio.unwrap_or(String::default()),
                r.monoblock.unwrap_or(String::default()),
                r.calle.unwrap_or(String::default()),
                r.numero.unwrap_or(String::default()),
                r.departamento.unwrap_or(String::default()),
                r.piso.unwrap_or(String::default()),
                r.codigo_postal.unwrap_or(String::default()),
            ]);
            let coberturas = r.cobertura.unwrap_or(Vec::default());
            for cob in coberturas {
                builder_coberturas.add_record([
                    cob.cobertura.unwrap_or(String::default()),
                    cob.nombre.unwrap_or(String::default()),
                    cob.documento.unwrap_or(String::default()),
                    cob.sexo.unwrap_or(String::default()),
                ]);
            }

            let table_datos = builder_datos.build().with(Style::rounded());
            let table_localidad = builder_localidad.build().with(Style::rounded());
            let table_coberturas = builder_coberturas.build().with(Style::rounded());
            println!(
                "{}\n{}\n{}",
                table_datos.to_string(),
                table_localidad.to_string(),
                table_coberturas.to_string(),
            );
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_nombre(nombre: &String, params: HashMap<&str, String>) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_name(nombre.to_string(), Some(params)).await {
        Ok(r) => {
            let mut builder_results = Builder::default();
            builder_results.set_columns(["CUIT", "Nombre", "Provincia"]);

            for per in r {
                builder_results.add_record([per.documento, per.nombre, per.provincia]);
            }

            let table_results = builder_results.build().with(Style::rounded());
            println!("{}", table_results.to_string());
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_movistar(numero: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_movistar_email(numero.to_string()).await {
        Ok(r) => {
            let mut builder_results = Builder::default();
            builder_results.set_columns(["Numero", "Email"]);
            builder_results.add_record([r.numero, r.email]);
            let table_results = builder_results.build().with(Style::rounded());
            println!("{}", table_results.to_string());
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_vecinos(direccion: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_by_address(direccion.to_string()).await {
        Ok(r) => {
            let mut builder_vecinos_datos = Builder::default();
            let mut builder_vecinos_direccion = Builder::default();

            builder_vecinos_datos.set_columns(["Documento", "Nombre", "Numero", "Empresa"]);
            builder_vecinos_direccion.set_columns([
                "Documento",
                "Provincia",
                "Localidad",
                "Direccion",
                "CP",
            ]);

            for vc in r {
                let documento = vc.documento.unwrap_or(String::default());
                builder_vecinos_datos.add_record([
                    documento.clone(),
                    vc.nombre.unwrap_or(String::default()),
                    vc.numero.unwrap_or(String::default()),
                    vc.empresa.unwrap_or(String::default()),
                ]);

                builder_vecinos_direccion.add_record([
                    documento.clone(),
                    vc.provincia.unwrap_or(String::default()),
                    vc.localidad.unwrap_or(String::default()),
                    vc.direccion.unwrap_or(String::default()),
                    vc.codigo_postal.unwrap_or(String::default()),
                ]);
            }

            let table_vecinos_datos = builder_vecinos_datos.build().with(Style::rounded());
            let table_vecinos_direccion = builder_vecinos_direccion.build().with(Style::rounded());
            println!(
                "{}\n{}",
                table_vecinos_direccion.to_string(),
                table_vecinos_datos.to_string(),
            );
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_celular(numero: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_phone(numero.to_string()).await {
        Ok(r) => {
            let mut builder_celulares = Builder::default();
            builder_celulares.set_columns([
                "Documento",
                "Nombre",
                "Provincia",
                "Localidad",
                "Direccion",
                "CP",
                "Numero",
                "Empresa",
            ]);
            for vc in r {
                builder_celulares.add_record([
                    vc.documento,
                    vc.nombre,
                    vc.provincia,
                    vc.localidad,
                    vc.direccion,
                    vc.codigo_postal,
                    vc.celular,
                    vc.empresa,
                ]);
            }

            let table_celulares = builder_celulares.build().with(Style::rounded());
            println!("{}", table_celulares.to_string());
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_celular_magic(numero: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_phone_magic(numero.to_string()).await {
        Ok(vc) => {
            let mut builder_results = Builder::default();
            builder_results.set_columns(["Nombre", "Apellido", "Email", "Numero"]);
            builder_results.add_record([vc.nombre, vc.apellido, vc.email, vc.numero]);

            let table_results = builder_results.build().with(Style::rounded());
            println!("{}", table_results.to_string());
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_cbu(cbu_alias: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_cbu(cbu_alias.to_string()).await {
        Ok(vc) => {
            let mut builder_results = Builder::default();
            builder_results.set_columns(["Nombre", "CUIT", "Banco", "CBU", "Tipo"]);
            builder_results.add_record([vc.nombre, vc.cuit, vc.banco, vc.cbu, vc.cuenta_tipo]);

            let table_results = builder_results.build().with(Style::rounded());
            println!("{}", table_results.to_string());
        }
        Err(e) => println!("{}", e),
    };
}

pub async fn handle_buscar_email(email: &String) {
    let mut client = SigmaClient::from_creds(read_credentials().unwrap());
    match client.search_email(email.to_string()).await {
        Ok(vc) => {
            let mut builder_results = Builder::default();
            builder_results.set_columns(["Nombre", "Apellido", "Email"]);
            builder_results.add_record([vc.nombre, vc.apellido, vc.email]);

            let table_results = builder_results.build().with(Style::rounded());
            println!("{}", table_results.to_string());
        }
        Err(e) => println!("{}", e),
    };
}
