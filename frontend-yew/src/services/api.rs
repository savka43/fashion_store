use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;

use crate::models::{AuthRequest, LoginResponse, Outfit, OutfitPayload, RegisterResponse};

fn api_url(path: &str) -> String {
    format!("/api{path}")
}

pub fn bearer(token: &str) -> String {
    format!("Bearer {token}")
}

async fn decode_json<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    let status = response.status();
    let status_text = response.status_text();
    let body = response.text().await.map_err(|err| err.to_string())?;

    if !(200..300).contains(&status) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(detail) = value.get("detail").and_then(|detail| detail.as_str()) {
                return Err(format!("API вернул HTTP {status}: {detail}"));
            }
        }
        return Err(format!("API вернул HTTP {status}: {status_text}"));
    }

    serde_json::from_str(&body).map_err(|err| format!("Не удалось прочитать JSON API: {err}"))
}

async fn decode_empty(response: Response) -> Result<(), String> {
    let status = response.status();
    let status_text = response.status_text();
    if (200..300).contains(&status) {
        Ok(())
    } else {
        let body = response.text().await.unwrap_or_default();
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(detail) = value.get("detail").and_then(|detail| detail.as_str()) {
                return Err(format!("API вернул HTTP {status}: {detail}"));
            }
        }
        Err(format!("API вернул HTTP {status}: {status_text}"))
    }
}

pub async fn register(email: &str, password: &str) -> Result<RegisterResponse, String> {
    let response = Request::post(&api_url("/auth/register"))
        .json(&AuthRequest { email: email.to_string(), password: password.to_string() })
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn login(email: &str, password: &str) -> Result<LoginResponse, String> {
    let response = Request::post(&api_url("/auth/login"))
        .json(&AuthRequest { email: email.to_string(), password: password.to_string() })
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn get_public_outfits() -> Result<Vec<Outfit>, String> {
    let response = Request::get(&api_url("/outfits"))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn get_my_outfits(token: &str) -> Result<Vec<Outfit>, String> {
    let response = Request::get(&api_url("/outfits/mine"))
        .header("Authorization", &bearer(token))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn create_outfit(token: &str, payload: &OutfitPayload) -> Result<Outfit, String> {
    let response = Request::post(&api_url("/outfits"))
        .header("Authorization", &bearer(token))
        .json(payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn update_outfit(token: &str, id: i64, payload: &OutfitPayload) -> Result<Outfit, String> {
    let response = Request::put(&api_url(&format!("/outfits/{id}")))
        .header("Authorization", &bearer(token))
        .json(payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn delete_outfit(token: &str, id: i64) -> Result<(), String> {
    let response = Request::delete(&api_url(&format!("/outfits/{id}")))
        .header("Authorization", &bearer(token))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_empty(response).await
}

pub async fn publish_outfit(token: &str, id: i64) -> Result<Outfit, String> {
    let response = Request::post(&api_url(&format!("/outfits/{id}/publish")))
        .header("Authorization", &bearer(token))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}

pub async fn unpublish_outfit(token: &str, id: i64) -> Result<Outfit, String> {
    let response = Request::post(&api_url(&format!("/outfits/{id}/unpublish")))
        .header("Authorization", &bearer(token))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    decode_json(response).await
}
