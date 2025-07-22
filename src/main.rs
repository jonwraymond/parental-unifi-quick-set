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
use utoipa::{OpenApi, ToSchema};

// OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        login_handler,
        get_devices,
        create_block_rule,
        unblock_rule,
        unblock_all_rules,
        get_rules
    ),
    components(
        schemas(LoginRequest, BlockRule, UnblockRequest, ApiResponse, DevicesResponse, DeviceInfo, RulesResponse, ActiveRule)
    ),
    tags(
        (name = "authentication", description = "UniFi controller authentication"),
        (name = "devices", description = "Network device management"),
        (name = "rules", description = "Parental control rule management")
    ),
    info(
        title = "Parental UniFi Quick Set API",
        version = "1.0.0",
        description = "A modern API for managing parental controls on UniFi networks. Block apps like Fortnite, Roblox, YouTube with flexible scheduling options.",
        contact(
            name = "GitHub Repository",
            url = "https://github.com/jonwraymond/parental-unifi-quick-set"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
        (url = "/", description = "Current server")
    )
)]
struct ApiDoc;

async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

async fn docs_page() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>API Documentation</title>
    <style>
        body { 
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; 
            max-width: 800px; 
            margin: 0 auto; 
            padding: 40px 20px; 
            line-height: 1.6;
            background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
        }
        .header { 
            text-align: center; 
            background: white; 
            padding: 40px; 
            border-radius: 16px; 
            box-shadow: 0 4px 12px rgba(0,0,0,0.1); 
            margin-bottom: 30px;
        }
        .header h1 { 
            color: #0369a1; 
            margin-bottom: 10px; 
        }
        .section { 
            background: white; 
            padding: 30px; 
            border-radius: 16px; 
            box-shadow: 0 4px 12px rgba(0,0,0,0.1); 
            margin-bottom: 20px;
        }
        .endpoint { 
            background: #f8fafc; 
            padding: 15px; 
            border-radius: 8px; 
            margin: 10px 0; 
            border-left: 4px solid #0ea5e9;
        }
        .method { 
            font-weight: bold; 
            color: #0ea5e9; 
            text-transform: uppercase; 
        }
        pre { 
            background: #1e293b; 
            color: #e2e8f0; 
            padding: 15px; 
            border-radius: 8px; 
            overflow-x: auto; 
        }
        .button {
            display: inline-block;
            background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
            color: white;
            padding: 12px 24px;
            border-radius: 8px;
            text-decoration: none;
            font-weight: 500;
            margin: 10px 10px 0 0;
        }
        .button:hover {
            background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🛡️ Parental Controls API</h1>
        <p>RESTful API for managing UniFi network parental controls</p>
        <a href="/api-docs/openapi.json" class="button">📄 OpenAPI JSON</a>
        <a href="https://editor.swagger.io/" target="_blank" class="button">🔧 Swagger Editor</a>
    </div>

    <div class="section">
        <h2>🔐 Authentication</h2>
        <div class="endpoint">
            <span class="method">POST</span> /api/login
            <p>Connect to your UniFi controller with local admin credentials.</p>
        </div>
    </div>

    <div class="section">
        <h2>📱 Device Management</h2>
        <div class="endpoint">
            <span class="method">GET</span> /api/devices
            <p>Discover all devices connected to your UniFi network.</p>
        </div>
    </div>

    <div class="section">
        <h2>🚫 Rule Management</h2>
        <div class="endpoint">
            <span class="method">POST</span> /api/block
            <p>Create a new blocking rule for apps (Fortnite, YouTube, etc.) with flexible scheduling.</p>
        </div>
        <div class="endpoint">
            <span class="method">GET</span> /api/rules
            <p>Get all active blocking rules with their current status.</p>
        </div>
        <div class="endpoint">
            <span class="method">POST</span> /api/unblock
            <p>Remove a specific blocking rule by ID.</p>
        </div>
        <div class="endpoint">
            <span class="method">POST</span> /api/unblock-all
            <p>Emergency unblock - remove all active rules at once.</p>
        </div>
    </div>

    <div class="section">
        <h2>🔧 Quick Start</h2>
        <pre><code># 1. Connect to UniFi
curl -X POST http://localhost:3000/api/login \\
  -H "Content-Type: application/json" \\
  -d '{"url":"https://192.168.1.1:8443","username":"admin","password":"password"}'

# 2. Block gaming apps
curl -X POST http://localhost:3000/api/block \\
  -H "Content-Type: application/json" \\
  -d '{"id":"1234","apps":["fortnite","roblox"],"type":"permanent","devices":["all"],"status":"active","created":"2024-01-01T12:00:00Z"}'

# 3. List active rules
curl http://localhost:3000/api/rules</code></pre>
    </div>

    <div class="section">
        <h2>📚 Supported Apps</h2>
        <p>🎮 <strong>Gaming:</strong> Fortnite, Roblox, Minecraft, Twitch, Discord</p>
        <p>📺 <strong>Video:</strong> YouTube, TikTok, Netflix</p>
        <p>📷 <strong>Social:</strong> Instagram, Snapchat</p>
    </div>
</body>
</html>
    "#)
}

// Request/Response structures with OpenAPI schemas
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "url": "https://192.168.1.1:8443",
    "username": "parental-control-app",
    "password": "your-secure-password"
}))]
struct LoginRequest {
    /// UniFi controller URL (e.g., https://192.168.1.1:8443)
    url: String,
    /// Local admin username (recommended over cloud accounts)
    username: String,
    /// Local admin password
    password: String,
}

#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "1642781234567",
    "apps": ["fortnite", "roblox"],
    "type": "permanent",
    "devices": ["all"],
    "status": "active",
    "created": "2024-01-01T12:00:00Z",
    "duration": 2,
    "endTime": "2024-01-01T14:00:00Z",
    "scheduleType": "bedtime"
}))]
struct BlockRule {
    /// Unique identifier for the rule
    id: String,
    /// List of app names to block (fortnite, roblox, youtube, etc.)
    apps: Vec<String>,
    #[serde(rename = "type")]
    /// Type of blocking: permanent, duration, until, schedule
    rule_type: String,
    /// Target devices (MAC addresses or "all")
    devices: Vec<String>,
    /// Rule status: active or disabled
    status: String,
    /// ISO timestamp when rule was created
    created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration in hours (for duration type)
    duration: Option<u32>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    /// End time for the rule (ISO timestamp)
    end_time: Option<String>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    /// Schedule type for recurring rules (bedtime, homework, etc.)
    schedule_type: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "ruleId": "1642781234567"
}))]
struct UnblockRequest {
    #[serde(rename = "ruleId")]
    /// ID of the rule to unblock/remove
    rule_id: String,
}

#[derive(Serialize, ToSchema)]
struct ApiResponse {
    /// Whether the operation was successful
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Error message if operation failed
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Success message if operation succeeded
    message: Option<String>,
}

#[derive(Serialize, ToSchema)]
struct DevicesResponse {
    /// Whether the operation was successful
    success: bool,
    /// List of discovered network devices
    devices: Vec<DeviceInfo>,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "mac": "aa:bb:cc:dd:ee:ff",
    "name": "iPhone 12",
    "type": "Apple"
}))]
struct DeviceInfo {
    /// Device MAC address
    mac: String,
    /// Friendly device name (if available)
    name: Option<String>,
    #[serde(rename = "type")]
    /// Device manufacturer or type
    device_type: Option<String>,
}

#[derive(Serialize, ToSchema)]
struct RulesResponse {
    /// Whether the operation was successful
    success: bool,
    /// List of active blocking rules
    rules: Vec<ActiveRule>,
}

#[derive(Serialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "1642781234567",
    "apps": ["fortnite", "roblox"],
    "rule_type": "permanent",
    "devices": ["all"],
    "status": "active",
    "created": "2024-01-01T12:00:00Z",
    "unifi_rule_id": "61d1234567890abcdef12345"
}))]
struct ActiveRule {
    /// Unique identifier for the rule
    id: String,
    /// List of blocked app names
    apps: Vec<String>,
    /// Type of blocking rule
    rule_type: String,
    /// Target devices
    devices: Vec<String>,
    /// Current rule status
    status: String,
    /// When the rule was created
    created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration in hours (for duration rules)
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// End time for the rule
    end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Schedule type for recurring rules
    schedule_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UniFi firewall rule ID (internal)
    unifi_rule_id: Option<String>,
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

/// Authenticate with UniFi controller
///
/// Connects to your UniFi controller using local admin credentials.
/// Cloud accounts are not recommended due to MFA requirements.
#[utoipa::path(
    post,
    path = "/api/login",
    tag = "authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse),
        (status = 401, description = "Authentication failed", body = ApiResponse),
        (status = 400, description = "Invalid request format", body = ApiResponse)
    )
)]
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

/// Get network devices
///
/// Discovers all devices connected to the UniFi network.
/// Requires authentication first.
#[utoipa::path(
    get,
    path = "/api/devices",
    tag = "devices",
    responses(
        (status = 200, description = "Devices retrieved successfully", body = DevicesResponse),
        (status = 401, description = "Not authenticated", body = DevicesResponse)
    )
)]
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

/// Create a blocking rule
///
/// Creates a new rule to block specified apps with flexible scheduling.
/// Supports permanent blocks, duration-based blocks, time-based blocks, and recurring schedules.
#[utoipa::path(
    post,
    path = "/api/block",
    tag = "rules",
    request_body = BlockRule,
    responses(
        (status = 200, description = "Rule created successfully", body = ApiResponse),
        (status = 400, description = "Invalid rule configuration", body = ApiResponse),
        (status = 401, description = "Not authenticated", body = ApiResponse)
    )
)]
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

/// Remove a blocking rule
///
/// Removes a specific blocking rule by ID. This will unblock the apps
/// for the specified devices and remove the rule from the UniFi controller.
#[utoipa::path(
    post,
    path = "/api/unblock",
    tag = "rules",
    request_body = UnblockRequest,
    responses(
        (status = 200, description = "Rule removed successfully", body = ApiResponse),
        (status = 404, description = "Rule not found", body = ApiResponse),
        (status = 401, description = "Not authenticated", body = ApiResponse)
    )
)]
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

/// Remove all blocking rules
///
/// Removes all active blocking rules at once. This is useful for
/// emergency situations where you need to quickly unblock everything.
#[utoipa::path(
    post,
    path = "/api/unblock-all",
    tag = "rules",
    responses(
        (status = 200, description = "All rules removed successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated", body = ApiResponse)
    )
)]
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

/// Get all active rules
///
/// Returns a list of all currently active blocking rules.
/// This includes rule details, schedules, and target devices.
#[utoipa::path(
    get,
    path = "/api/rules",
    tag = "rules",
    responses(
        (status = 200, description = "Rules retrieved successfully", body = RulesResponse)
    )
)]
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
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/docs", get(docs_page))
        .with_state(state);

    println!("🚀 Parental UniFi Quick Set running on http://0.0.0.0:3000");
    println!("📱 Mobile-friendly interface with beautiful styling");
    println!("🛡️ Easy parental controls for your UniFi network");
    println!("📚 API Documentation available at http://0.0.0.0:3000/docs");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
        
    axum::serve(listener, app).await.unwrap();
}