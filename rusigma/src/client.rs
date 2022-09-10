use crate::errors::Error;
use crate::http::HTTPClient;
use crate::response;

use std::collections::HashMap;

pub struct SigmaClient {
    pub http: HTTPClient,
}

impl SigmaClient {
    pub fn new() -> SigmaClient {
        SigmaClient {
            http: HTTPClient::new(),
        }
    }

    pub fn from_creds(credentials: String) -> SigmaClient {
        let mut client = SigmaClient::new();
        let split: Vec<&str> = credentials.split(':').collect();
        client
            .http
            .set_api_token(split[0], split[1].parse::<usize>().unwrap());
        client
    }

    pub async fn login_with_credentials(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        self.http.login_with_credentials(username, password).await?;
        Ok(())
    }

    /// Standard - Medium - Profesional
    pub async fn search_standard_dni(
        &mut self,
        dni: String,
    ) -> Result<response::DNIStandardResponse, Error> {
        Ok(self.http.get_data_from_dni(dni).await?)
    }

    pub async fn search_phones_by_dni(
        &mut self,
        dni: String,
    ) -> Result<Vec<response::PhoneNumber>, Error> {
        Ok(self.http.get_phones_from_dni(dni).await?)
    }

    /// Medium - Profesional
    pub async fn search_plate(
        &mut self,
        plate: String,
    ) -> Result<Vec<response::PlateHistory>, Error> {
        Ok(self.http.get_plate(plate).await?)
    }

    pub async fn search_plate_by_dni(
        &mut self,
        dni: String,
    ) -> Result<Vec<response::PlateHistory>, Error> {
        Ok(self.http.get_plate_from_dni(dni).await?)
    }

    pub async fn search_leaks(
        &mut self,
        query: String,
    ) -> Result<Vec<response::BreachCredentials>, Error> {
        Ok(self.http.get_query_data_breach(query).await?)
    }

    /// Profesional
    pub async fn search_profesional_dni(
        &mut self,
        dni: String,
        gender: usize,
    ) -> Result<response::DNIProfesional, Error> {
        let gender_str = match gender {
            1 => "Masculino",
            2 => "Femenino",
            _ => "Otro",
        };
        Ok(self
            .http
            .get_data_from_dni_profesional(dni, gender_str.to_string())
            .await?)
    }

    pub async fn search_name(
        &mut self,
        name: String,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Vec<response::PersonaNombre>, Error> {
        Ok(self.http.get_names(name, params).await?)
    }

    pub async fn search_movistar_email(
        &mut self,
        number: String,
    ) -> Result<response::MovistarEmail, Error> {
        Ok(self.http.get_movistar_email(number).await?)
    }

    pub async fn search_by_address(
        &mut self,
        address: String,
    ) -> Result<Vec<response::PersonaDireccion>, Error> {
        Ok(self.http.get_people_by_address(address).await?)
    }

    pub async fn search_phone(
        &mut self,
        number: String,
    ) -> Result<Vec<response::PersonaFromNumero>, Error> {
        Ok(self.http.get_data_by_number(number).await?)
    }

    pub async fn search_phone_magic(
        &mut self,
        number: String,
    ) -> Result<response::PersonaFromNumeroMagic, Error> {
        Ok(self.http.get_data_by_number_magic(number).await?)
    }

    pub async fn search_cbu(
        &mut self,
        cvu_or_alias: String,
    ) -> Result<response::TitularCBU, Error> {
        Ok(self.http.get_data_by_cvu(cvu_or_alias).await?)
    }

    pub async fn search_email(
        &mut self,
        email: String,
    ) -> Result<response::EmailResultados, Error> {
        Ok(self.http.get_data_by_email(email).await?)
    }
}
