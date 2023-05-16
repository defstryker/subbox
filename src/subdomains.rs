use serde_derive::Deserialize;
use serde_derive::Serialize;


pub type SubdomainList = Vec<Subdomain>;

pub trait SubdomainWrapper {
    fn get_unique_common_names(&self) -> Vec<String>;
    fn get_strict_subdomains(&self, domain: String) -> Vec<String>;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subdomain {
    #[serde(rename = "issuer_ca_id")]
    issuer_ca_id: i64,
    #[serde(rename = "issuer_name")]
    issuer_name: String,
    #[serde(rename = "common_name")]
    pub common_name: String,
    #[serde(rename = "name_value")]
    name_value: String,
    id: i64,
    #[serde(rename = "entry_timestamp")]
    entry_timestamp: String,
    #[serde(rename = "not_before")]
    not_before: String,
    #[serde(rename = "not_after")]
    not_after: String,
    #[serde(rename = "serial_number")]
    serial_number: String,
}

impl SubdomainWrapper for SubdomainList {
    fn get_unique_common_names(&self) -> Vec<String> {
        let mut temp = self
            .into_iter()
            .map(
                |subd| 
                subd.common_name.clone()
            )
            .collect::<Vec<String>>();

        temp.sort();
        temp.dedup();
        temp
    }

    fn get_strict_subdomains(&self, domain: String) -> Vec<String> {
        self
            .get_unique_common_names()
            .into_iter()
            .filter(
                |subd|
                subd.ends_with(domain.as_str())
            )
            .collect()
    }
}