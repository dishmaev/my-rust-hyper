use super::{connectors, models};
use base64;
use std::collections::HashMap;

pub struct AccessChecker {
    user_authorization: HashMap<String, String>,
}

impl AccessChecker {
    fn get_basic_authorization(user: &String, password: &String) -> String {
        format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", user, password))
        )
    }
    
    pub fn _from_app_settings(app_settings: &models::AppSettings) -> AccessChecker {
        let mut authorization: HashMap<String, String> = HashMap::new();
        for item in &app_settings.authentication {
            authorization.insert(
                AccessChecker::get_basic_authorization(&item.0, &item.1),
                item.1.to_string(),
            );
        }
        debug!("{} users", authorization.len());
        AccessChecker {
            user_authorization: authorization,
        }
    }

    pub async fn from_data_connector(
        dc: &connectors::DataConnector,
    ) -> connectors::Result<AccessChecker> {
        let items = dc.usr.get(None).await?;
        let mut authorization: HashMap<String, String> = HashMap::new();
        for item in items {
            authorization.insert(
                AccessChecker::get_basic_authorization(&item.usr_name, &item.usr_password),
                item.usr_name,
            );
        }
        debug!("{} users", authorization.len());
        Ok(AccessChecker {
            user_authorization: authorization,
        })
    }

    pub fn is_authorized_by_header(&self, header: &str) -> bool {
        *&self.user_authorization.contains_key(header)
    }
}
