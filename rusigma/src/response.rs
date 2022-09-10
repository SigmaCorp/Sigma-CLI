extern crate reqwest;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: bool,
    #[serde(rename = "mensaje")]
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub plan: u8,
}

#[derive(Deserialize, Debug)]
pub struct DNIStandardResponse {
    #[serde(rename = "doc")]
    pub documento: String,
    pub apellido: String,
    pub nombres: String,
    pub calle: String,
    pub seccion: String,
    pub circuito: String,
    pub tipo_doc: String,
    pub localidad: String,
    pub provincia: String,
    pub codigo_postal: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PhoneNumber {
    #[serde(rename = "doc")]
    pub documento: String,
    pub numero: String,
    pub nombre: String,
    pub localidad: String,
    pub provincia: String,
    pub codigo_postal: String,
    pub empresa: String,
}

#[derive(Deserialize, Debug)]
pub struct PlateHistory {
    pub patente: Option<String>,
    pub documento: Option<String>,
    pub vehiculo: Option<String>,
    pub marca: Option<String>,
    pub anio: Option<String>,
    pub titular: Option<String>,
    pub porcentaje: Option<String>,
    pub calle: Option<String>,
    pub altura: Option<String>,
    pub piso: Option<String>,
    pub depto: Option<String>,
    pub codigo_postal: Option<String>,
    pub localidad: Option<String>,
    pub transferencia: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BreachCredentials {
    #[serde(rename = "usuario")]
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ObraSocial {
    pub cobertura: Option<String>,
    pub nombre: Option<String>,
    #[serde(rename = "dni")]
    pub documento: Option<String>,
    pub sexo: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DNIProfesional {
    pub emision: Option<String>,
    pub apellido: Option<String>,
    pub nombres: Option<String>,
    pub cuil: Option<String>,
    pub calle: Option<String>,
    pub numero: Option<String>,
    pub piso: Option<String>,
    pub departamento: Option<String>,
    pub barrio: Option<String>,
    pub monoblock: Option<String>,
    pub ciudad: Option<String>,
    pub municipio: Option<String>,
    pub provincia: Option<String>,
    pub pais: Option<String>,
    pub foto: Option<String>,
    pub tramite: Option<String>,
    pub documento: Option<String>,
    pub fallecido: Option<String>,
    pub codigo_postal: Option<String>,
    pub cobertura: Option<Vec<ObraSocial>>,
    pub fecha_nacimiento: Option<String>,
    pub edad: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct PersonaNombre {
    pub nombre: String,
    pub documento: String,
    pub provincia: String,
}

#[derive(Deserialize, Debug)]
pub struct MovistarEmail {
    #[serde(rename = "num")]
    pub numero: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct PersonaDireccion {
    pub numero: Option<String>,
    #[serde(rename = "doc")]
    pub documento: Option<String>,
    pub nombre: Option<String>,
    pub direccion: Option<String>,
    pub localidad: Option<String>,
    pub provincia: Option<String>,
    pub codigo_postal: Option<String>,
    pub empresa: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PersonaFromNumero {
    pub celular: String,
    pub documento: String,
    pub nombre: String,
    pub direccion: String,
    pub localidad: String,
    pub provincia: String,
    pub codigo_postal: String,
    pub empresa: String,
}

#[derive(Deserialize, Debug)]
pub struct PersonaFromNumeroMagic {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub numero: String,
}

#[derive(Deserialize, Debug)]
pub struct EmailResultados {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct TitularCBU {
    pub nombre: String,
    pub cuit: String,
    pub banco: String,
    pub cbu: String,
    pub cuenta_tipo: String,
}
