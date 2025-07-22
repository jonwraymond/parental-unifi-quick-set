use axum::{
    extract::{Json, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Request/Response structures
#[derive(Deserialize)]
struct LoginRequest {
    url: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct BlockRule {
    id: String,
    apps: Vec<String>,
    #[serde(rename = "type")]
    rule_type: String,
    devices: Vec<String>,
    status: String,
    created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<String>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    schedule_type: Option<String>,
}

#[derive(Deserialize)]
struct UnblockRequest {
    #[serde(rename = "ruleId")]
    rule_id: String,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct DevicesResponse {
    success: bool,
    devices: Vec<DeviceInfo>,
}

#[derive(Serialize)]
struct DeviceInfo {
    mac: String,
    name: Option<String>,
    #[serde(rename = "type")]
    device_type: Option<String>,
}

#[derive(Serialize)]
struct RulesResponse {
    success: bool,
    rules: Vec<ActiveRule>,
}

#[derive(Serialize, Clone)]
struct ActiveRule {
    id: String,
    apps: Vec<String>,
    rule_type: String,
    devices: Vec<String>,
    status: String,
    created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unifi_rule_id: Option<String>, // Track UniFi firewall rule ID
}

// Application state
#[derive(Clone)]
struct AppState {
    client: Client,
    unifi_url: Arc<Mutex<Option<String>>>,
    session_cookies: Arc<Mutex<Option<String>>>,
    app_id_map: HashMap<String, String>,
    active_rules: Arc<Mutex<Vec<ActiveRule>>>,
}

impl AppState {
    fn new() -> Self {
        let mut app_id_map = HashMap::new();
        
        // Extended app mapping with more popular apps
        app_id_map.insert("fortnite".to_string(), "655369".to_string());
        app_id_map.insert("roblox".to_string(), "851993".to_string());
        app_id_map.insert("youtube".to_string(), "851969".to_string());
        app_id_map.insert("tiktok".to_string(), "855327".to_string());
        app_id_map.insert("instagram".to_string(), "655311".to_string());
        app_id_map.insert("snapchat".to_string(), "655301".to_string());
        app_id_map.insert("netflix".to_string(), "655324".to_string());
        app_id_map.insert("twitch".to_string(), "655328".to_string());
        app_id_map.insert("discord".to_string(), "655365".to_string());
        app_id_map.insert("minecraft".to_string(), "655370".to_string());

        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            unifi_url: Arc::new(Mutex::new(None)),
            session_cookies: Arc::new(Mutex::new(None)),
            app_id_map,
            active_rules: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../index.html"))
}

async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    println!("🔐 Login attempt to: {}", request.url);
    
    // Validate URL format
    if !request.url.starts_with("https://") && !request.url.starts_with("http://") {
        return Json(ApiResponse {
            success: false,
            error: Some("Invalid URL format. Must start with https:// or http://".to_string()),
            message: None,
        });
    }

    let login_url = format!("{}/api/login", request.url);
    let login_data = serde_json::json!({
        "username": request.username,
        "password": request.password
    });

    match state.client.post(&login_url)
        .json(&login_data)
        .send()
        .await 
    {
        Ok(response) => {
            if response.status().is_success() {
                // Extract cookies for session management
                let cookies = response
                    .headers()
                    .get_all(header::SET_COOKIE)
                    .iter()
                    .filter_map(|hv| hv.to_str().ok())
                    .collect::<Vec<_>>()
                    .join("; ");

                // Update state
                *state.unifi_url.lock().await = Some(request.url);
                *state.session_cookies.lock().await = Some(cookies);

                println!("✅ Login successful");
                Json(ApiResponse {
                    success: true,
                    error: None,
                    message: Some("Connected successfully".to_string()),
                })
            } else {
                println!("❌ Login failed: HTTP {}", response.status());
                Json(ApiResponse {
                    success: false,
                    error: Some(format!("Authentication failed: {}", response.status())),
                    message: None,
                })
            }
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            Json(ApiResponse {
                success: false,
                error: Some(format!("Connection failed: {}", e)),
                message: None,
            })
        }
    }
}

async fn get_devices(State(state): State<AppState>) -> impl IntoResponse {
    let unifi_url = state.unifi_url.lock().await.clone();
    let cookies = state.session_cookies.lock().await.clone();

    let (url, cookie_header) = match (unifi_url, cookies) {
        (Some(url), Some(cookies)) => (url, cookies),
        _ => {
            return Json(DevicesResponse {
                success: false,
                devices: vec![],
            });
        }
    };

    let devices_url = format!("{}/api/s/default/stat/sta", url);
    
    match state.client.get(&devices_url)
        .header(header::COOKIE, cookie_header)
        .send()
        .await 
    {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                let devices: Vec<DeviceInfo> = json["data"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|device| DeviceInfo {
                        mac: device["mac"].as_str().unwrap_or("").to_string(),
                        name: device["hostname"].as_str().or(device["name"].as_str()).map(|s| s.to_string()),
                        device_type: device["oui"].as_str().map(|s| s.to_string()),
                    })
                    .collect();

                Json(DevicesResponse {
                    success: true,
                    devices,
                })
            } else {
                Json(DevicesResponse {
                    success: false,
                    devices: vec![],
                })
            }
        }
        Err(_) => Json(DevicesResponse {
            success: false,
            devices: vec![],
        }),
    }
}

async fn create_block_rule(
    State(state): State<AppState>,
    Json(rule): Json<BlockRule>,
) -> impl IntoResponse {
    println!("🚫 Creating block rule for apps: {:?}", rule.apps);

    let unifi_url = state.unifi_url.lock().await.clone();
    let cookies = state.session_cookies.lock().await.clone();

    let (url, cookie_header) = match (unifi_url, cookies) {
        (Some(url), Some(cookies)) => (url, cookies),
        _ => {
            return Json(ApiResponse {
                success: false,
                error: Some("Not logged in to UniFi".to_string()),
                message: None,
            });
        }
    };

    // Convert app names to UniFi app IDs
    let app_ids: Vec<String> = rule.apps
        .iter()
        .filter_map(|app| state.app_id_map.get(app).cloned())
        .collect();

    if app_ids.is_empty() {
        return Json(ApiResponse {
            success: false,
            error: Some("No valid apps selected for blocking".to_string()),
            message: None,
        });
    }

    // Create firewall rule in UniFi
    let firewall_url = format!("{}/api/s/default/rest/firewallrule", url);
    let firewall_rule = serde_json::json!({
        "name": format!("Parental Block - {}", rule.apps.join(", ")),
        "ruleset": "WAN_IN",
        "rule_index": 2000,
        "action": "drop",
        "protocol_match_excepted": false,
        "logging": false,
        "state_established": false,
        "state_invalid": false,
        "state_new": false,
        "state_related": false,
        "ipsec": "",
        "src_firewallgroup_ids": [],
        "src_mac_address": "",
        "src_address": "",
        "src_port": "",
        "dst_firewallgroup_ids": [],
        "dst_address": "",
        "dst_port": "",
        "icmp_typename": "",
        "app_category_ids": app_ids,
        "enabled": rule.status == "active"
    });

    match state.client.post(&firewall_url)
        .header(header::COOKIE, cookie_header)
        .json(&firewall_rule)
        .send()
        .await 
    {
        Ok(response) => {
            if response.status().is_success() {
                // Parse response to get the created rule ID
                let unifi_rule_id = if let Ok(json) = response.json::<serde_json::Value>().await {
                    json["data"][0]["_id"].as_str().map(|s| s.to_string())
                } else {
                    None
                };

                // Store rule in our state
                let active_rule = ActiveRule {
                    id: rule.id.clone(),
                    apps: rule.apps.clone(),
                    rule_type: rule.rule_type.clone(),
                    devices: rule.devices.clone(),
                    status: rule.status.clone(),
                    created: rule.created.clone(),
                    duration: rule.duration,
                    end_time: rule.end_time.clone(),
                    schedule_type: rule.schedule_type.clone(),
                    unifi_rule_id,
                };

                state.active_rules.lock().await.push(active_rule);

                println!("✅ Block rule created successfully");
                Json(ApiResponse {
                    success: true,
                    error: None,
                    message: Some("Block rule created successfully".to_string()),
                })
            } else {
                println!("❌ Failed to create firewall rule: HTTP {}", response.status());
                Json(ApiResponse {
                    success: false,
                    error: Some("Failed to create firewall rule in UniFi".to_string()),
                    message: None,
                })
            }
        }
        Err(e) => {
            println!("❌ Error creating firewall rule: {}", e);
            Json(ApiResponse {
                success: false,
                error: Some(format!("Error creating rule: {}", e)),
                message: None,
            })
        }
    }
}

async fn unblock_rule(
    State(state): State<AppState>,
    Json(request): Json<UnblockRequest>,
) -> impl IntoResponse {
    println!("🔓 Unblocking rule: {}", request.rule_id);

    let unifi_url = state.unifi_url.lock().await.clone();
    let cookies = state.session_cookies.lock().await.clone();

    let (url, cookie_header) = match (unifi_url, cookies) {
        (Some(url), Some(cookies)) => (url, cookies),
        _ => {
            return Json(ApiResponse {
                success: false,
                error: Some("Not logged in to UniFi".to_string()),
                message: None,
            });
        }
    };

    // Find and remove the rule from our state
    let mut rules = state.active_rules.lock().await;
    let rule_position = rules.iter().position(|r| r.id == request.rule_id);
    
    if let Some(pos) = rule_position {
        let rule = rules.remove(pos);
        
        // If we have a UniFi rule ID, delete it from the controller
        if let Some(ref unifi_rule_id) = rule.unifi_rule_id {
            let delete_url = format!("{}/api/s/default/rest/firewallrule/{}", url, unifi_rule_id);
            
            match state.client.delete(&delete_url)
                .header(header::COOKIE, cookie_header)
                .send()
                .await 
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("✅ Rule unblocked successfully");
                        Json(ApiResponse {
                            success: true,
                            error: None,
                            message: Some("Rule unblocked successfully".to_string()),
                        })
                    } else {
                        println!("❌ Failed to delete UniFi rule: HTTP {}", response.status());
                        // Re-add the rule since deletion failed
                        rules.push(rule);
                        Json(ApiResponse {
                            success: false,
                            error: Some("Failed to delete rule from UniFi".to_string()),
                            message: None,
                        })
                    }
                }
                Err(e) => {
                    println!("❌ Error deleting UniFi rule: {}", e);
                    // Re-add the rule since deletion failed
                    rules.push(rule);
                    Json(ApiResponse {
                        success: false,
                        error: Some(format!("Error deleting rule: {}", e)),
                        message: None,
                    })
                }
            }
        } else {
            // No UniFi rule ID, just remove from local state
            Json(ApiResponse {
                success: true,
                error: None,
                message: Some("Rule removed from local state".to_string()),
            })
        }
    } else {
        Json(ApiResponse {
            success: false,
            error: Some("Rule not found".to_string()),
            message: None,
        })
    }
}

async fn unblock_all_rules(State(state): State<AppState>) -> impl IntoResponse {
    println!("🔓 Unblocking all rules");

    let unifi_url = state.unifi_url.lock().await.clone();
    let cookies = state.session_cookies.lock().await.clone();

    let (url, cookie_header) = match (unifi_url, cookies) {
        (Some(url), Some(cookies)) => (url, cookies),
        _ => {
            return Json(ApiResponse {
                success: false,
                error: Some("Not logged in to UniFi".to_string()),
                message: None,
            });
        }
    };

    let mut rules = state.active_rules.lock().await;
    let mut failed_deletions = Vec::new();

    // Delete all UniFi rules
    for rule in rules.iter() {
        if let Some(unifi_rule_id) = &rule.unifi_rule_id {
            let delete_url = format!("{}/api/s/default/rest/firewallrule/{}", url, unifi_rule_id);
            
            if let Err(e) = state.client.delete(&delete_url)
                .header(header::COOKIE, &cookie_header)
                .send()
                .await 
            {
                failed_deletions.push(format!("Rule {}: {}", rule.id, e));
            }
        }
    }

    // Clear all rules from local state
    rules.clear();

    if failed_deletions.is_empty() {
        Json(ApiResponse {
            success: true,
            error: None,
            message: Some("All rules unblocked successfully".to_string()),
        })
    } else {
        Json(ApiResponse {
            success: false,
            error: Some(format!("Some rules failed to delete: {}", failed_deletions.join(", "))),
            message: None,
        })
    }
}

async fn get_rules(State(state): State<AppState>) -> impl IntoResponse {
    let rules = state.active_rules.lock().await.clone();
    
    Json(RulesResponse {
        success: true,
        rules,
    })
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/login", post(login_handler))
        .route("/api/devices", get(get_devices))
        .route("/api/block", post(create_block_rule))
        .route("/api/unblock", post(unblock_rule))
        .route("/api/unblock-all", post(unblock_all_rules))
        .route("/api/rules", get(get_rules))
        .with_state(state);

    println!("🚀 Parental UniFi Quick Set running on http://0.0.0.0:3000");
    println!("📱 Mobile-friendly interface with beautiful styling");
    println!("🛡️ Easy parental controls for your UniFi network");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
        
    axum::serve(listener, app).await.unwrap();
}