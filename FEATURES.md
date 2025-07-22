# 🛡️ Parental UniFi Quick Set - Features Overview

**A modern, professional parental control interface matching UniFi's design language**

## ✨ **Key Features**

### 🎨 **UniFi-Inspired Design**
- **Clean Blue & White Theme**: Matches UniFi's professional design language perfectly
- **Minimalist Card Layout**: Subtle shadows, rounded corners, and elegant spacing
- **Professional Typography**: System fonts with proper weight and hierarchy  
- **Micro-Interactions**: Smooth hover effects and polished animations
- **Mobile-First Responsive**: Feels native on phones, tablets, and desktop

### 🛠️ **Enhanced User Experience**
- **Interactive Tooltips**: Helpful guidance on every form field with setup links
- **Real-Time Feedback**: Connection status indicator and visual state updates
- **Visual App Icons**: Easy recognition with emojis (🎮 🟩 📺 🎵 📷)
- **Parent-Friendly Language**: Clear, non-technical descriptions and actions
- **Documentation Links**: Direct access to setup guides and troubleshooting

### ⚡ **Quick Actions**
- **🎮 Block Gaming**: Instantly block Fortnite, Roblox, Minecraft with one click
- **📺 Block Videos**: Block YouTube, TikTok, Netflix with one click  
- **⏰ Schedule Rules**: Quick access to homework time, bedtime schedules
- **✅ Unblock All**: Emergency unblock - remove all rules instantly

### 🚫 **Flexible Blocking Options**
- **Permanent Blocks**: Long-term restrictions for inappropriate content
- **Duration-Based**: Block for specific hours (1-24 hours)
- **Time-Based**: Block until a specific date/time
- **Recurring Schedules**: Bedtime (8 PM - 7 AM), Homework Time (3 PM - 6 PM)
- **Custom Schedules**: Create your own recurring time blocks

### 📱 **Supported Apps & Services**
- **Gaming**: Fortnite, Roblox, Minecraft, Twitch, Discord
- **Video**: YouTube, TikTok, Netflix
- **Social**: Instagram, Snapchat
- **Extensible**: Easy to add more apps with simple configuration

### 🌐 **Device Targeting**
- **All Devices**: Apply rules to the entire network
- **Specific Devices**: Target individual phones, tablets, computers
- **Auto-Discovery**: Automatically finds all network devices
- **Friendly Names**: Shows device names instead of MAC addresses

### 📚 **Comprehensive API Documentation**
- **OpenAPI 3.0 Specification**: Complete, standards-compliant API docs
- **Interactive Documentation**: Beautiful UI at `/docs` with examples
- **JSON Schema**: Full request/response validation and examples
- **cURL Examples**: Ready-to-use command line examples
- **Swagger Compatible**: Import into any OpenAPI-compatible tool

### 🔧 **API Endpoints**
- **POST /api/login**: Authenticate with UniFi controller
- **GET /api/devices**: Discover all network devices  
- **POST /api/block**: Create new blocking rules with scheduling
- **GET /api/rules**: List all active parental control rules
- **POST /api/unblock**: Remove specific rules by ID
- **POST /api/unblock-all**: Emergency unblock all active rules

### 💾 **State Management**
- **Persistent Storage**: Rules survive application restarts
- **Local Backup**: Browser localStorage as backup for rule state
- **Real-Time Sync**: Updates immediately reflect in UniFi controller
- **Rule Tracking**: Each rule gets unique ID for precise management

### 🏗️ **Technical Excellence**
- **Rust + Axum Backend**: Fast, safe, and reliable server
- **UniFi Integration**: Direct API communication with controllers
- **Mobile-Optimized**: Progressive Web App capabilities
- **Security**: HTTPS-only connections, secure session management
- **Error Handling**: Graceful failures with helpful error messages

### 🎯 **Parent-Friendly Features**
- **One-Click Actions**: Common blocking scenarios with single button
- **Visual Status**: Clear indicators showing what's currently blocked
- **Easy Override**: Quick unblock for special occasions
- **Schedule Templates**: Pre-configured homework and bedtime rules
- **Emergency Access**: Panic button to unblock everything instantly

## 🚀 **Getting Started**

### Quick Setup (5 minutes)
1. **Download & Run**: Get the binary and start the server
2. **Open Browser**: Navigate to `http://localhost:3000`
3. **Connect UniFi**: Enter your controller URL and credentials  
4. **Start Blocking**: Use quick actions or create custom rules
5. **View API Docs**: Visit `/docs` for complete API reference

### Advanced Usage
- **API Integration**: Use the REST API for automation and scripts
- **Custom Schedules**: Create complex recurring blocking patterns
- **Device Management**: Fine-tune rules for specific family members
- **Monitoring**: Track rule effectiveness and usage patterns

## 🔮 **Future Roadmap**
- **Multi-Site Support**: Manage multiple UniFi sites from one interface
- **Usage Analytics**: Track blocked attempts and usage patterns  
- **Profile Management**: Different rules for different family members
- **Notification System**: Alerts when rules are triggered
- **Mobile App**: Native iOS/Android apps for on-the-go management

---

**Transform your home network into a family-friendly environment with professional-grade parental controls that actually work.** 