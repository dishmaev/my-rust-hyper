use super::{connectors, errors, settings};
use base64;
use std::collections::HashMap;

pub struct AccessChecker {
    sa: HashMap<String, String>,
    cba: HashMap<String, String>,
    cba_common: Option<String>,
}

impl AccessChecker {
    pub fn get_basic_authorization_token(user: &String, password: &String) -> String {
        format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", user, password))
        )
    }

    pub fn get_client_basic_authorization_token(
        &self,
        service_name: String,
    ) -> connectors::Result<String> {
        if self.cba.contains_key(&service_name) {
            Ok(self.cba.get(&service_name).unwrap().clone())
        } else if self.cba_common.is_some() {
            Ok(self.cba_common.as_ref().unwrap().clone())
        } else {
            return Err(errors::UnknownServiceNameError.into());
        }
    }

    pub async fn _from_app_settings(
        access: &settings::Access,
    ) -> connectors::Result<AccessChecker> {
        let mut sa: HashMap<String, String> = HashMap::new();
        for item in &access.authentication.server {
            sa.insert(
                AccessChecker::get_basic_authorization_token(&item.0, &item.1),
                item.0.to_string(),
            );
        }
        debug!("{} server users", sa.len());
        let mut cba: HashMap<String, String> = HashMap::new();
        let mut cba_common = None;
        for item in &access.authentication.client {
            let token =
                AccessChecker::get_basic_authorization_token(&item.usr_name, &item.usr_password);
            if item.service_name == "*" {
                cba_common = Some(token.clone());
            }
            cba.insert(token, item.service_name.to_string());
        }
        debug!("{} client users", cba.len());
        Ok(AccessChecker {
            sa: sa,
            cba: cba,
            cba_common: cba_common,
        })
    }

    pub async fn from_data_connector(
        dc: &connectors::DataConnector,
        authentication: &settings::Authentication,
    ) -> connectors::Result<AccessChecker> {
        let items = dc.usr.get(None).await?;
        let mut server_authorization: HashMap<String, String> = HashMap::new();
        for item in items {
            server_authorization.insert(
                AccessChecker::get_basic_authorization_token(&item.usr_name, &item.usr_password),
                item.usr_name,
            );
        }
        debug!("{} server users", server_authorization.len());
        let mut cba: HashMap<String, String> = HashMap::new();
        let mut cba_common = None;
        for item in &authentication.client {
            let token =
                AccessChecker::get_basic_authorization_token(&item.usr_name, &item.usr_password);
            if item.service_name == "*" {
                cba_common = Some(token.clone());
            }
            cba.insert(token, item.service_name.to_string());
        }
        debug!("{} client users", cba.len());
        Ok(AccessChecker {
            sa: server_authorization,
            cba: cba,
            cba_common: cba_common,
        })
    }

    pub fn is_authorized_by_header(&self, header: &str) -> bool {
        *&self.sa.contains_key(header)
    }
}
