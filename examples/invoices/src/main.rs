extern crate lexoffice_api;
use lexoffice_api::apis::configuration::Configuration;
use lexoffice_api::apis::invoices_api;
use std::env;

#[tokio::main]
async fn main() {
    // Get LexOffice API-Key from environment
    let api_key = env::var("LEXOFFICE_APIKEY")
        .expect("Environment variable LEXOFFICE_APIKEY must be set");
    
    // Authentication with Bearer Token
    let mut api_config = Configuration::new();
    api_config.bearer_access_token = Some(api_key);
    
    // TODO: use the id of an invoice which exists in your LexOffice account here
    let invoice = invoices_api::invoices_id_get(&api_config, "92034df4-a0eb-41e7-970b-128d37c5cc0a")
        .await
        .expect("Error getting invoice by ID");
    println!("{:?}", invoice);
}