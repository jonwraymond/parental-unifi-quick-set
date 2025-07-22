use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json as AxumJson},
    routing::{get, post},
    Router,
};
use chrono::{Local, NaiveTime, Duration as ChronoDuration};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    client: Client,
    unifi_url: String,
    site: String,
    token: Arc<Mutex<Option<String>>>,
    app_id_map: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct Network {
    name: String,
    vlan_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct UniFiClient {
    mac: String,
    hostname: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut app_id_map = HashMap::new();
    app_id_map.insert("Fortnite".to_string(), "655369".to_string());
    app_id_map.insert("Roblox".to_string(), "851993".to_string());
    app_id_map.insert("YouTube".to_string(), "851969".to_string());
    // Add more as needed

    let state = AppState {
        client: Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
        unifi_url: "https://192.168.1.1:8443".to_string(),  // Update to your UniFi controller
        site: "default".to_string(),
        token: Arc::new(Mutex::new(None)),
        app_id_map,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/login", post(login))
        .route("/api/create_rule", post(create_rule))
        .route("/api/networks", get(get_networks))
        .route("/api/clients", get(get_clients))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(Arc::new(state));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../index.html"))
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

async fn login(State(state): State<Arc<AppState>>, Json(input): Json<LoginInput>) -> impl IntoResponse {
    let url = format!("{}/api/auth/login", state.unifi_url);
    let payload = serde_json::json!({
        "username": input.username,
        "password": input.password,
    });

    match state.client.post(&url).json(&payload).send().await {
        Ok(resp) if resp.status().is_success() => {
            let token = resp.headers().get("x-csrf-token").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
            *state.token.lock().await = token;
            (StatusCode::OK, "Logged in successfully".to_string())
        }
        _ => (StatusCode::UNAUTHORIZED, "Login failed".to_string()),
    }
}

#[derive(Deserialize)]
struct RuleInput {
    apps: Vec<String>,
    block_type: String,  // "permanent", "hourly", "until", "recurring"
    duration_hours: Option<f64>,  // For hourly
    until_time: Option<String>,  // HH:MM for until
    recurring_days: Option<Vec<String>>,  // For recurring
    recurring_start: Option<String>,  // HH:MM
    recurring_end: Option<String>,  // HH:MM
    networks: Vec<String>,  // Network names or VLAN IDs
    devices: Vec<String>,  // MACs
}

async fn create_rule(State(state): State<Arc<AppState>>, Json(input): Json<RuleInput>) -> impl IntoResponse {
    let token_guard = state.token.lock().await;
    if token_guard.is_none() {
        return (StatusCode::UNAUTHORIZED, "Not logged in".to_string());
    }

    let app_ids: Vec<String> = input.apps.iter().filter_map(|app| state.app_id_map.get(app).cloned()).collect();
    if app_ids.is_empty() {
        return (StatusCode::BAD_REQUEST, "Invalid apps selected".to_string());
    }

    let mut payload = serde_json::json!({
        "action": "block",
        "description": "App Block Rule",
        "enabled": true,
        "logging": false,
        "match": {
            "app": {
                "ids": app_ids,
            },
        },
        "source": {
            "macs": input.devices,
        },
        "target": "INTERNET",
        "target_networks": input.networks,  // Assuming API supports array of network IDs/names; adjust if needed
    });

    let is_temporary = input.block_type == "hourly" || input.block_type == "until";
    let mut schedule_enabled = false;

    if input.block_type == "recurring" {
        if let (Some(days), Some(start), Some(end)) = (input.recurring_days, input.recurring_start, input.recurring_end) {
            payload["schedule_enabled"] = serde_json::json!(true);
            payload["schedule"] = serde_json::json!({
                "type": "recurring",
                "recurring": [{
                    "days_of_week": days,
                    "start_hour": start.split(':').next().unwrap_or("0").parse::<i32>().unwrap_or(0),
                    "start_minute": start.split(':').nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0),
                    "end_hour": end.split(':').next().unwrap_or("0").parse::<i32>().unwrap_or(0),
                    "end_minute": end.split(':').nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0),
                }]
            });
            schedule_enabled = true;
        }
    } else if input.block_type == "permanent" {
        // No schedule
    }

    let url = format!("{}/proxy/network/v2/api/site/{}/trafficrules", state.unifi_url, state.site);
    let mut req = state.client.post(&url).json(&payload);
    if let Some(token) = token_guard.as_ref() {
        req = req.header(header::AUTHORIZATION, format!("Bearer {}", token));
    }

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {
            let body: serde_json::Value = resp.json().await.unwrap_or(serde_json::json!({}));
            let rule_id = body.get("_id").and_then(|id| id.as_str()).map(|s| s.to_string());

            if is_temporary && rule_id.is_some() {
                let end_time = if input.block_type == "hourly" {
                    if let Some(hours) = input.duration_hours {
                        Local::now() + ChronoDuration::hours(hours as i64)
                    } else {
                        return (StatusCode::BAD_REQUEST, "Missing duration".to_string());
                    }
                } else if let Some(time_str) = input.until_time {
                    let time = NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap_or_default();
                    let now = Local::now();
                    let mut end = now.date_naive().and_time(time).and_local_timezone(Local).unwrap();
                    if end < now {
                        end += ChronoDuration::days(1);
                    }
                    end
                } else {
                    return (StatusCode::BAD_REQUEST, "Missing until time".to_string());
                };

                let state_clone = state.clone();
                let rule_id_clone = rule_id.unwrap();
                tokio::spawn(async move {
                    let sleep_duration = (end_time - Local::now()).num_milliseconds() as u64;
                    tokio::time::sleep(std::time::Duration::from_millis(sleep_duration)).await;
                    delete_rule(&state_clone, &rule_id_clone).await;
                });
            }

            (StatusCode::OK, "Rule created successfully".to_string())
        }
        Ok(resp) => {
            let status = if resp.status().is_success() {
                StatusCode::OK
            } else if resp.status().is_client_error() {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, format!("Error: {}", resp.text().await.unwrap_or_default()))
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Request failed: {}", e)),
    }
}

async fn delete_rule(state: &AppState, rule_id: &str) {
    let token_guard = state.token.lock().await;
    if let Some(token) = token_guard.as_ref() {
        let url = format!("{}/proxy/network/v2/api/site/{}/trafficrules/{}", state.unifi_url, state.site, rule_id);
        let _ = state.client.delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await;
    }
}

async fn get_networks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let token_guard = state.token.lock().await;
    if token_guard.is_none() {
        return AxumJson(vec![]);
    }

    let url = format!("{}/proxy/network/api/s/{}/rest/networkconf", state.unifi_url, state.site);
    let mut req = state.client.get(&url);
    if let Some(token) = token_guard.as_ref() {
        req = req.header(header::AUTHORIZATION, format!("Bearer {}", token));
    }

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {
            let body: serde_json::Value = resp.json().await.unwrap_or(serde_json::json!({}));
            let networks: Vec<Network> = body["data"].as_array().unwrap_or(&vec![]).iter().map(|net| {
                Network {
                    name: net["name"].as_str().unwrap_or("").to_string(),
                    vlan_id: net["vlan"].as_u64().map(|v| v as u16),
                }
            }).collect();
            AxumJson(networks)
        }
        _ => AxumJson(vec![]),
    }
}

async fn get_clients(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let token_guard = state.token.lock().await;
    if token_guard.is_none() {
        return AxumJson(vec![]);
    }

    let url = format!("{}/proxy/network/api/s/{}/stat/sta", state.unifi_url, state.site);
    let mut req = state.client.get(&url);
    if let Some(token) = token_guard.as_ref() {
        req = req.header(header::AUTHORIZATION, format!("Bearer {}", token));
    }

    match req.send().await {
        Ok(resp) if resp.status().is_success() => {
            let body: serde_json::Value = resp.json().await.unwrap_or(serde_json::json!({}));
            let clients: Vec<UniFiClient> = body["data"].as_array().unwrap_or(&vec![]).iter().map(|client| {
                UniFiClient {
                    mac: client["mac"].as_str().unwrap_or("").to_string(),
                    hostname: client["hostname"].as_str().map(|s| s.to_string()),
                }
            }).collect();
            AxumJson(clients)
        }
        _ => AxumJson(vec![]),
    }
}