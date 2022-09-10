extern crate reqwest;

use crate::errors::Error;
use crate::response::{
    BreachCredentials, DNIProfesional, DNIStandardResponse, EmailResultados, ErrorResponse,
    LoginResponse, MovistarEmail, PersonaDireccion, PersonaFromNumero, PersonaFromNumeroMagic,
    PersonaNombre, PhoneNumber, PlateHistory, TitularCBU,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

const PLAN_LIST: [&str; 5] = ["free", "profesional", "medium", "standard", "comunidades"];

pub struct HTTPClient {
    http_client: reqwest::Client,
    pub token_str: Option<String>,
    pub plan_id: Option<usize>,
    base_url: String,
    base_endpoint: String,
}

impl HTTPClient {
    pub fn new() -> HTTPClient {
        let http_client = reqwest::Client::new();
        HTTPClient {
            http_client,
            token_str: None,
            plan_id: None,
            base_url: "https://sigma-search.io".to_string(),
            base_endpoint: "/api/v2".to_string(),
        }
    }

    /// Set API token and plan if you already have one
    pub fn set_api_token(&mut self, token_str: &str, plan_id: usize) {
        self.token_str = Some(token_str.to_string());
        self.plan_id = Some(plan_id)
    }

    fn get_url(&mut self, path: &str, plan: Option<usize>) -> String {
        format!(
            "{}{}/{}{}",
            self.base_url,
            self.base_endpoint,
            PLAN_LIST[plan.unwrap_or_else(|| self.plan_id.unwrap())],
            path
        )
    }

    /// Sends an HTTP request, returns JSON
    pub async fn request<T: DeserializeOwned>(
        &self,
        endpoint: String,
        data: HashMap<&str, String>,
    ) -> Result<T, Error> {
        if self.token_str.is_none() {
            return Err(Error::NoLoginToken);
        }
        let response = self
            .http_client
            .post(endpoint)
            .header("sigma-key", self.token_str.as_ref().unwrap())
            .json(&data)
            .send()
            .await?;
        if response.status().is_success() {
            let rjson: T = response.json().await?;
            Ok(rjson)
        } else {
            let rjson_error: ErrorResponse = response.json().await?;
            Err(Error::ApiError(rjson_error))
        }
    }

    /// Login with username and password, retrive token and plan
    pub async fn login_with_credentials(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let endpoint = format!("{}/api/sigma/client/login", self.base_url);
        let data = HashMap::from([("username", username), ("password", password)]);
        let response = self.http_client.post(endpoint).json(&data).send().await?;
        if response.status().is_success() {
            let rjson: LoginResponse = response.json().await?;
            self.token_str = Some(rjson.token);
            self.plan_id = Some(usize::from(rjson.plan));
            Ok(())
        } else {
            let rjson_error: ErrorResponse = response.json().await?;
            Err(Error::ApiError(rjson_error))
        }
    }

    pub async fn get_data_from_dni(&mut self, dni: String) -> Result<DNIStandardResponse, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/dni", None);
        let data = HashMap::from([("dni", dni)]);
        let result: DNIStandardResponse = self.request(endpoint, data).await?;
        Ok(result)
    }

    pub async fn get_phones_from_dni(&mut self, dni: String) -> Result<Vec<PhoneNumber>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/dni_celular", None);
        let data = HashMap::from([("dni", dni)]);
        let result: Vec<PhoneNumber> = self.request(endpoint, data).await?;
        Ok(result)
    }

    pub async fn get_plate(&mut self, plate: String) -> Result<Vec<PlateHistory>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/patente", None);
        let data = HashMap::from([("patente", plate)]);
        let result: Vec<PlateHistory> = self.request(endpoint, data).await?;
        Ok(result)
    }

    pub async fn get_plate_from_dni(&mut self, dni: String) -> Result<Vec<PlateHistory>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/patente_dni", None);
        let data = HashMap::from([("dni", dni)]);
        let result: Vec<PlateHistory> = self.request(endpoint, data).await?;
        Ok(result)
    }

    pub async fn get_query_data_breach(
        &mut self,
        query: String,
    ) -> Result<Vec<BreachCredentials>, Error> {
        let endpoint = self.get_url("/osint/argentina/search_engine/data_breach", None);
        let data = HashMap::from([("query", query)]);
        let result: Vec<BreachCredentials> = self.request(endpoint, data).await?;
        Ok(result)
    }

    pub async fn get_data_from_dni_profesional(
        &mut self,
        dni: String,
        gender: String,
    ) -> Result<DNIProfesional, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/dni_two", None);
        let data = HashMap::from([("dato", format!("{}:{}", dni, gender))]);
        let response: DNIProfesional = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_names(
        &mut self,
        name: String,
        parameters: Option<HashMap<&str, String>>,
    ) -> Result<Vec<PersonaNombre>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/nombre", None);
        let mut data = HashMap::from([("nombre", name)]);
        if !parameters.is_none() {
            data.extend(parameters.unwrap());
        }
        let response: Vec<PersonaNombre> = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_movistar_email(&mut self, phone: String) -> Result<MovistarEmail, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/movistar", None);
        let data = HashMap::from([("num", phone)]);
        let response: MovistarEmail = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_people_by_address(
        &mut self,
        address: String,
    ) -> Result<Vec<PersonaDireccion>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/direccion", None);
        let data = HashMap::from([("direccion", address)]);
        let response: Vec<PersonaDireccion> = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_data_by_number(
        &mut self,
        number: String,
    ) -> Result<Vec<PersonaFromNumero>, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/celular", None);
        let data = HashMap::from([("num", number)]);
        let response: Vec<PersonaFromNumero> = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_data_by_number_magic(
        &mut self,
        number: String,
    ) -> Result<PersonaFromNumeroMagic, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/magic", None);
        let data = HashMap::from([("dato", number), ("tipo", "buscar_celular".to_string())]);
        let response: PersonaFromNumeroMagic = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_data_by_cvu(&mut self, cvu_alias: String) -> Result<TitularCBU, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/magic", None);
        let data = HashMap::from([
            ("dato", cvu_alias),
            ("tipo", "buscar_cbu_alias".to_string()),
        ]);
        let response: TitularCBU = self.request(endpoint, data).await?;
        Ok(response)
    }

    pub async fn get_data_by_email(&mut self, email: String) -> Result<EmailResultados, Error> {
        let endpoint = self.get_url("/osint/argentina/resolver/magic", None);
        let data = HashMap::from([("dato", email), ("tipo", "buscar_email".to_string())]);
        let response: EmailResultados = self.request(endpoint, data).await?;
        Ok(response)
    }
}
