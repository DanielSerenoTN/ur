use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize, Serialize)]
struct ZohoTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZohoProduct {
    pub id: String,
    #[serde(default)]
    pub Product_Name: Option<String>,
    #[serde(default)]
    pub Estatus_venta: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
struct ZohoSearchResponse {
    data: Vec<ZohoProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoMapAccess {
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct ZohoMapAccessResponse {
    data: Vec<ZohoMapAccess>,
}

pub async fn get_access_token() -> Result<String> {
    dotenv().ok();

    let refresh_token = env::var("ZOHO_REFRESH_TOKEN").context("ZOHO_REFRESH_TOKEN is missing in .env")?;
    let client_id = env::var("ZOHO_CLIENT_ID").context("ZOHO_CLIENT_ID is missing in .env")?;
    let client_secret = env::var("ZOHO_CLIENT_SECRET").context("ZOHO_CLIENT_SECRET is missing in .env")?;
    let accounts_url = env::var("ZOHO_ACCOUNTS_URL").unwrap_or_else(|_| "https://accounts.zoho.com".to_string());

    let client = Client::new();
    let url = format!("{}/oauth/v2/token", accounts_url);

    let response = client
        .post(&url)
        .form(&[
            ("refresh_token", refresh_token.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .await
        .context("Failed to send request to Zoho to obtain access token")?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow!("Zoho access token request failed with status {}: {}", status, error_body));
    }
    let token_response: ZohoTokenResponse = response
        .json()
        .await
        .context("Failed to parse Zoho access token response")?;
    println!("Petition a zoho (token)");
    Ok(token_response.access_token)
}


pub async fn get_paginated_products(page: usize, per_page: usize) -> Result<Vec<ZohoProduct>> {
    dotenv().ok();

    let api_domain = env::var("ZOHO_API_DOMAIN").context("ZOHO_API_DOMAIN is missing in .env")?;
    let access_token = get_access_token().await.context("Failed to get Zoho access token")?;

    let url = format!(
        "{}/Products?page={}&per_page={}&fields=id,Product_Name,Estatus_venta",
        api_domain, page, per_page
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .bearer_auth(&access_token)
        .send()
        .await
        .context("Failed to send request to Zoho to search products")?;

    if response.status() == reqwest::StatusCode::NO_CONTENT {
        println!("Zoho API returned 204 No Content - No products found");
        return Ok(vec![]);
    }

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow!("Zoho product search request failed with status {}: {}", status, error_body));
    }

    let search_response: ZohoSearchResponse = response
        .json()
        .await
        .context("Failed to parse Zoho product search response")?;

    if search_response.data.is_empty() {
        return Ok(vec![]);
    }

    println!("Retrieved {} products from page {}", search_response.data.len(), page);
    Ok(search_response.data)
}

pub async fn search_map_access_by_name(name: &str) -> Result<ZohoMapAccess> {
    dotenv().ok();

    let api_domain = env::var("ZOHO_API_DOMAIN").context("ZOHO_API_DOMAIN is missing in .env")?;
    let access_token = get_access_token().await.context("Failed to get Zoho access token")?;
    let url = format!("{}/Acceso_a_Mapas", api_domain);

    let client = Client::new();
    println!("Sending request to Zoho API for map access: {}", url);

    let response = client
        .get(&url)
        .query(&[("fields", "Name")])
        .bearer_auth(&access_token)
        .send()
        .await
        .context("Failed to send request to Zoho for map access search")?;

    let status = response.status();
    let body = response.text().await.context("Failed to read Zoho API response body")?;

    println!("Zoho API Response Body: {}", body);

    if !status.is_success() {
        return Err(anyhow!(
            "Zoho map access search request failed with status {}: {}",
            status,
            body
        ));
    }

    let map_access_response: ZohoMapAccessResponse = serde_json::from_str(&body)
        .context("Failed to parse Zoho map access search response")?;

    if map_access_response.data.is_empty() {
        return Err(anyhow!("No map access found for the given name: {}", name));
    }

    println!("Successfully retrieved map access for: {}", name);
    Ok(map_access_response.data.into_iter().next().unwrap())
}
