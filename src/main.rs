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
use std::fs;
use std::path::Path;

// Rule persistence configuration
const RULES_DB_FILE: &str = "parental_rules.json";
const RULE_NAME_PREFIX: &str = "[PUC]"; // Parental UniFi Control prefix for UniFi rules

// Persistent rule storage
#[derive(Serialize, Deserialize, Clone)]
struct RuleDatabase {
    rules: Vec<ActiveRule>,
    created_at: String,
    last_updated: String,
}

impl RuleDatabase {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn load() -> Self {
        if Path::new(RULES_DB_FILE).exists() {
            match fs::read_to_string(RULES_DB_FILE) {
                Ok(content) => {
                    match serde_json::from_str::<RuleDatabase>(&content) {
                        Ok(db) => {
                            println!("📂 Loaded {} rules from persistent storage", db.rules.len());
                            return db;
                        }
                        Err(e) => println!("⚠️ Failed to parse rules database: {}", e),
                    }
                }
                Err(e) => println!("⚠️ Failed to read rules database: {}", e),
            }
        }
        println!("📂 Creating new rules database");
        RuleDatabase::new()
    }

    fn save(&mut self) -> Result<(), String> {
        self.last_updated = chrono::Utc::now().to_rfc3339();
        match serde_json::to_string_pretty(self) {
            Ok(content) => {
                match fs::write(RULES_DB_FILE, content) {
                    Ok(_) => {
                        println!("💾 Saved {} rules to persistent storage", self.rules.len());
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to write rules database: {}", e))
                }
            }
            Err(e) => Err(format!("Failed to serialize rules database: {}", e))
        }
    }

    fn add_rule(&mut self, rule: ActiveRule) -> Result<(), String> {
        // Check for duplicate IDs
        if self.rules.iter().any(|r| r.id == rule.id) {
            return Err("Rule with this ID already exists".to_string());
        }
        self.rules.push(rule);
        self.save()
    }

    fn remove_rule(&mut self, rule_id: &str) -> Option<ActiveRule> {
        if let Some(pos) = self.rules.iter().position(|r| r.id == rule_id) {
            let rule = self.rules.remove(pos);
            let _ = self.save();
            Some(rule)
        } else {
            None
        }
    }

    fn update_rule(&mut self, rule_id: &str, updated_rule: ActiveRule) -> Result<(), String> {
        if let Some(pos) = self.rules.iter().position(|r| r.id == rule_id) {
            self.rules[pos] = updated_rule;
            self.save()
        } else {
            Err("Rule not found".to_string())
        }
    }

    fn clear_all(&mut self) -> Result<(), String> {
        self.rules.clear();
        self.save()
    }

    fn get_rules(&self) -> &Vec<ActiveRule> {
        &self.rules
    }
}

// OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        login_handler,
        get_devices,
        create_block_rule,
        unblock_rule,
        unblock_all_rules,
        get_rules,
        sync_rules,
        cleanup_rules
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
        <div class="endpoint">
            <span class="method">POST</span> /api/sync
            <p>Manually synchronize rules with the UniFi controller.</p>
        </div>
        <div class="endpoint">
            <span class="method">POST</span> /api/cleanup
            <p>Manually clean up orphaned rules in the UniFi controller.</p>
        </div>
    </div>

    <div class="section">
        <h2>🔧 Quick Start</h2>
        <pre><code># 1. Connect to UniFi
curl -X POST http://localhost:3000/api/login \\
  -H "Content-Type: application/json" \\
  -d '{"url":"https://192.168.1.1:8443","username":"admin","password":"YOUR_PASSWORD"}'

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
    "password": "YOUR_SECURE_PASSWORD"
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

#[derive(Serialize, Deserialize, Clone, ToSchema)]
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

// Enhanced rule management
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

        // Load persistent rules
        let rules_db = RuleDatabase::load();

        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            unifi_url: Arc::new(Mutex::new(None)),
            session_cookies: Arc::new(Mutex::new(None)),
            app_id_map,
            rules_db: Arc::new(Mutex::new(rules_db)),
        }
    }

    // Sync rules with UniFi controller
    async fn sync_rules_with_unifi(&self) -> Result<(), String> {
        let unifi_url = self.unifi_url.lock().await.clone();
        let cookies = self.session_cookies.lock().await.clone();

        let (url, cookie_header) = match (unifi_url, cookies) {
            (Some(url), Some(cookies)) => (url, cookies),
            _ => return Err("Not authenticated with UniFi".to_string()),
        };

        // Get all firewall rules from UniFi
        let firewall_url = if url.contains("/proxy/network") {
            format!("{}/api/s/default/rest/firewallrule", url)
        } else {
            format!("{}/proxy/network/api/s/default/rest/firewallrule", url)
        };

        println!("🔄 Syncing rules with UniFi controller...");
        
        match self.client.get(&firewall_url)
            .header(header::COOKIE, cookie_header)
            .send()
            .await 
        {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    let empty_vec = vec![];
                    let unifi_rules = json["data"].as_array().unwrap_or(&empty_vec);
                    
                    // Find our rules by name prefix
                    let our_unifi_rules: Vec<&serde_json::Value> = unifi_rules
                        .iter()
                        .filter(|rule| {
                            rule["name"].as_str()
                                .map(|name| name.starts_with(RULE_NAME_PREFIX))
                                .unwrap_or(false)
                        })
                        .collect();

                    println!("🔍 Found {} UniFi rules created by our tool", our_unifi_rules.len());

                    // Update our database with UniFi rule IDs
                    let mut rules_db = self.rules_db.lock().await;
                    for our_rule in rules_db.rules.iter_mut() {
                        if our_rule.unifi_rule_id.is_none() {
                            // Try to match by name
                            let expected_name = format!("{} {}", RULE_NAME_PREFIX, our_rule.apps.join(", "));
                            if let Some(unifi_rule) = our_unifi_rules.iter().find(|r| {
                                r["name"].as_str() == Some(&expected_name)
                            }) {
                                our_rule.unifi_rule_id = unifi_rule["_id"].as_str().map(|s| s.to_string());
                                println!("🔗 Linked rule {} to UniFi rule {}", our_rule.id, 
                                    our_rule.unifi_rule_id.as_ref().unwrap());
                            }
                        }
                    }

                    let _ = rules_db.save();
                    Ok(())
                } else {
                    Err("Failed to parse UniFi firewall rules".to_string())
                }
            }
            Err(e) => Err(format!("Failed to fetch UniFi rules: {}", e))
        }
    }

    // Clean orphaned UniFi rules (rules in UniFi but not in our database)
    async fn cleanup_orphaned_rules(&self) -> Result<u32, String> {
        let unifi_url = self.unifi_url.lock().await.clone();
        let cookies = self.session_cookies.lock().await.clone();

        let (url, cookie_header) = match (unifi_url, cookies) {
            (Some(url), Some(cookies)) => (url, cookies),
            _ => return Err("Not authenticated with UniFi".to_string()),
        };

        let firewall_url = if url.contains("/proxy/network") {
            format!("{}/api/s/default/rest/firewallrule", url)
        } else {
            format!("{}/proxy/network/api/s/default/rest/firewallrule", url)
        };

        match self.client.get(&firewall_url)
            .header(header::COOKIE, &cookie_header)
            .send()
            .await 
        {
            Ok(response) => {
                let status = response.status();
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    let empty_vec = vec![];
                    let unifi_rules = json["data"].as_array().unwrap_or(&empty_vec);
                    
                    // Find orphaned rules (our prefix but not in database)
                    let rules_db = self.rules_db.lock().await;
                    let our_rule_ids: Vec<&String> = rules_db.rules
                        .iter()
                        .filter_map(|r| r.unifi_rule_id.as_ref())
                        .collect();

                    let orphaned_rules: Vec<&serde_json::Value> = unifi_rules
                        .iter()
                        .filter(|rule| {
                            let name = rule["name"].as_str().unwrap_or("");
                            let rule_id = rule["_id"].as_str().unwrap_or("");
                            name.starts_with(RULE_NAME_PREFIX) && !our_rule_ids.contains(&&rule_id.to_string())
                        })
                        .collect();

                    println!("🧹 Found {} orphaned rules to clean up", orphaned_rules.len());

                    let mut cleaned_count = 0;
                    for orphaned_rule in orphaned_rules {
                        if let Some(rule_id) = orphaned_rule["_id"].as_str() {
                            let delete_url = if url.contains("/proxy/network") {
                                format!("{}/api/s/default/rest/firewallrule/{}", url, rule_id)
                            } else {
                                format!("{}/proxy/network/api/s/default/rest/firewallrule/{}", url, rule_id)
                            };

                            match self.client.delete(&delete_url)
                                .header(header::COOKIE, &cookie_header)
                                .send()
                                .await
                            {
                                Ok(resp) if resp.status().is_success() => {
                                    cleaned_count += 1;
                                    println!("🗑️ Cleaned orphaned rule: {}", 
                                        orphaned_rule["name"].as_str().unwrap_or("unknown"));
                                }
                                Ok(_) => {
                                    println!("⚠️ Failed to delete orphaned rule: HTTP {}", 
                                        status);
                                }
                                Err(e) => {
                                    println!("⚠️ Error deleting orphaned rule: {}", e);
                                }
                            }
                        }
                    }

                    Ok(cleaned_count)
                } else {
                    Err("Failed to parse UniFi firewall rules".to_string())
                }
            }
            Err(e) => Err(format!("Failed to fetch UniFi rules: {}", e))
        }
    }
}

// Application state with persistent storage
#[derive(Clone)]
struct AppState {
    client: Client,
    unifi_url: Arc<Mutex<Option<String>>>,
    session_cookies: Arc<Mutex<Option<String>>>,
    app_id_map: HashMap<String, String>,
    rules_db: Arc<Mutex<RuleDatabase>>,
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

    // For UniFi OS (UDM devices), use the correct auth endpoint
    let login_url = if request.url.contains("/proxy/network") {
        // User provided proxy/network URL - use traditional controller login
        format!("{}/api/login", request.url)
    } else {
        // Regular UniFi OS URL - use the UniFi OS auth endpoint that works
        format!("{}/api/auth/login", request.url)
    };

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

                println!("✅ Login successful via {}", login_url);
                Json(ApiResponse {
                    success: true,
                    error: None,
                    message: Some("Connected successfully to UniFi OS".to_string()),
                })
            } else {
                println!("❌ Login failed: HTTP {}", response.status());
                Json(ApiResponse {
                    success: false,
                    error: Some(format!("Authentication failed: {}. Try using the UniFi OS auth endpoint.", response.status())),
                    message: None,
                })
            }
        }
        Err(e) => {
            println!("❌ Connection error: {}", e);
            Json(ApiResponse {
                success: false,
                error: Some(format!("Connection failed: {}. Verify the UniFi controller is accessible.", e)),
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

    // For UniFi OS, we need to use the proxy endpoint to access network data
    let devices_url = if url.contains("/proxy/network") {
        // Traditional controller path
        format!("{}/api/s/default/stat/sta", url)
    } else {
        // UniFi OS path - use proxy to access network controller
        format!("{}/proxy/network/api/s/default/stat/sta", url)
    };
    
    println!("🔍 Discovering devices at: {}", devices_url);
    
    match state.client.get(&devices_url)
        .header(header::COOKIE, cookie_header)
        .send()
        .await 
    {
        Ok(response) => {
            println!("📱 Device discovery response: HTTP {}", response.status());
            if let Ok(json) = response.json::<serde_json::Value>().await {
                println!("📊 Device data received: {}", json.to_string().chars().take(200).collect::<String>());
                
                let devices: Vec<DeviceInfo> = json["data"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|device| DeviceInfo {
                        mac: device["mac"].as_str().unwrap_or("").to_string(),
                        name: device["hostname"].as_str()
                            .or(device["name"].as_str())
                            .or(device["display_name"].as_str())
                            .map(|s| s.to_string()),
                        device_type: device["oui"].as_str()
                            .or(device["manufacturer"].as_str())
                            .map(|s| s.to_string()),
                    })
                    .filter(|d| !d.mac.is_empty()) // Only include devices with MAC addresses
                    .collect();

                println!("✅ Found {} devices", devices.len());
                Json(DevicesResponse {
                    success: true,
                    devices,
                })
            } else {
                println!("❌ Failed to parse device response");
                Json(DevicesResponse {
                    success: false,
                    devices: vec![],
                })
            }
        }
        Err(e) => {
            println!("❌ Device discovery error: {}", e);
            Json(DevicesResponse {
                success: false,
                devices: vec![],
            })
        }
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

    // For UniFi OS, use the proxy endpoint to access network firewall rules
    let firewall_url = if url.contains("/proxy/network") {
        // Traditional controller path
        format!("{}/api/s/default/rest/firewallrule", url)
    } else {
        // UniFi OS path - use proxy to access network controller
        format!("{}/proxy/network/api/s/default/rest/firewallrule", url)
    };
    
    println!("🔥 Creating firewall rule at: {}", firewall_url);

    let firewall_rule = serde_json::json!({
        "name": format!("{} {}", RULE_NAME_PREFIX, rule.apps.join(", ")),
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
            println!("🔥 Firewall rule response: HTTP {}", response.status());
            if response.status().is_success() {
                // Parse response to get the created rule ID
                let unifi_rule_id = if let Ok(json) = response.json::<serde_json::Value>().await {
                    println!("📊 Firewall response data: {}", json.to_string().chars().take(200).collect::<String>());
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

                state.rules_db.lock().await.add_rule(active_rule).unwrap();

                println!("✅ Block rule created successfully");
                Json(ApiResponse {
                    success: true,
                    error: None,
                    message: Some("Block rule created successfully".to_string()),
                })
            } else {
                // Try to get the error message from response
                let status = response.status();
                let error_msg = if let Ok(text) = response.text().await {
                    format!("Failed to create firewall rule: HTTP {} - {}", status, text.chars().take(200).collect::<String>())
                } else {
                    format!("Failed to create firewall rule: HTTP {}", status)
                };
                println!("❌ {}", error_msg);
                Json(ApiResponse {
                    success: false,
                    error: Some(error_msg),
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

    // Find and remove the rule from our persistent storage
    let mut rules_db = state.rules_db.lock().await;
    if let Some(rule) = rules_db.remove_rule(&request.rule_id) {
        // If we have a UniFi rule ID, delete it from the controller
        if let Some(ref unifi_rule_id) = rule.unifi_rule_id {
            let delete_url = if url.contains("/proxy/network") {
                format!("{}/api/s/default/rest/firewallrule/{}", url, unifi_rule_id)
            } else {
                format!("{}/proxy/network/api/s/default/rest/firewallrule/{}", url, unifi_rule_id)
            };
            
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
                        let _ = rules_db.add_rule(rule);
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
                    let _ = rules_db.add_rule(rule);
                    Json(ApiResponse {
                        success: false,
                        error: Some(format!("Error deleting rule: {}", e)),
                        message: None,
                    })
                }
            }
        } else {
            // No UniFi rule ID, just removed from local state
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

    let mut rules_db = state.rules_db.lock().await;
    let mut failed_deletions = Vec::new();

    // Delete all UniFi rules
    for rule in rules_db.get_rules().iter() {
        if let Some(unifi_rule_id) = &rule.unifi_rule_id {
            let delete_url = if url.contains("/proxy/network") {
                format!("{}/api/s/default/rest/firewallrule/{}", url, unifi_rule_id)
            } else {
                format!("{}/proxy/network/api/s/default/rest/firewallrule/{}", url, unifi_rule_id)
            };
            
            if let Err(e) = state.client.delete(&delete_url)
                .header(header::COOKIE, &cookie_header)
                .send()
                .await 
            {
                failed_deletions.push(format!("Rule {}: {}", rule.id, e));
            }
        }
    }

    // Clear all rules from persistent storage
    let _ = rules_db.clear_all();

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
    let rules_db = state.rules_db.lock().await;
    let rules = rules_db.get_rules().clone();
    
    Json(RulesResponse {
        success: true,
        rules,
    })
}

/// Sync rules with UniFi controller
///
/// Synchronizes local rule database with UniFi controller firewall rules.
/// This ensures consistency between our tool and the UniFi controller.
#[utoipa::path(
    post,
    path = "/api/sync",
    tag = "rules",
    responses(
        (status = 200, description = "Rules synchronized successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated", body = ApiResponse)
    )
)]
async fn sync_rules(State(state): State<AppState>) -> impl IntoResponse {
    println!("🔄 Manual rule synchronization requested");
    
    match state.sync_rules_with_unifi().await {
        Ok(_) => Json(ApiResponse {
            success: true,
            error: None,
            message: Some("Rules synchronized successfully".to_string()),
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            error: Some(e),
            message: None,
        })
    }
}

/// Clean orphaned rules
///
/// Removes UniFi firewall rules created by this tool but no longer tracked in our database.
/// This helps maintain a clean UniFi configuration.
#[utoipa::path(
    post,
    path = "/api/cleanup",
    tag = "rules",
    responses(
        (status = 200, description = "Orphaned rules cleaned successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated", body = ApiResponse)
    )
)]
async fn cleanup_rules(State(state): State<AppState>) -> impl IntoResponse {
    println!("🧹 Manual rule cleanup requested");
    
    match state.cleanup_orphaned_rules().await {
        Ok(count) => Json(ApiResponse {
            success: true,
            error: None,
            message: Some(format!("Cleaned {} orphaned rules", count)),
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            error: Some(e),
            message: None,
        })
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    // Perform initial sync on startup if authenticated
    println!("🚀 Starting Parental UniFi Quick Set...");

    let app = Router::new()
        .route("/", get(index))
        .route("/api/login", post(login_handler))
        .route("/api/devices", get(get_devices))
        .route("/api/block", post(create_block_rule))
        .route("/api/unblock", post(unblock_rule))
        .route("/api/unblock-all", post(unblock_all_rules))
        .route("/api/rules", get(get_rules))
        .route("/api/sync", post(sync_rules))
        .route("/api/cleanup", post(cleanup_rules))
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/docs", get(docs_page))
        .with_state(state);

    println!("🚀 Parental UniFi Quick Set running on http://0.0.0.0:3000");
    println!("📱 Mobile-friendly interface with beautiful styling");
    println!("🛡️ Easy parental controls for your UniFi network");
    println!("📚 API Documentation available at http://0.0.0.0:3000/docs");
    println!("💾 Persistent rule storage enabled");
    println!("🔄 Automatic rule synchronization with UniFi");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
        
    axum::serve(listener, app).await.unwrap();
}