# 🛡️ Parental UniFi Quick Set - Features Overview

**A modern, beautiful parental control interface for UniFi networks**

## ✨ **Key Features**

### 🎨 **Beautiful, Modern Interface**
- **Mobile-First Design**: Fully responsive, works perfectly on phones, tablets, and desktop
- **Gradient Styling**: Beautiful purple-to-blue gradients with smooth animations
- **Card-Based Layout**: Clean, organized sections with hover effects and shadows
- **App Icons**: Visual identification with emojis (🎮 Fortnite, 📺 YouTube, etc.)
- **Loading States**: Smooth spinners and progress indicators

### ⚡ **Quick Actions**
- **🎮 Block Gaming**: Instantly block Fortnite, Roblox with one click
- **📺 Block Videos**: Block YouTube, TikTok, Netflix with one click  
- **⏰ Schedule Rules**: Quick access to homework time, bedtime schedules
- **✅ Unblock All**: Emergency unblock all rules with confirmation

### 🚫 **Flexible Blocking Options**
- **Permanent Blocks**: Always blocked until manually removed
- **Duration Blocks**: Block for specific hours (1-24 hours)
- **Time-Based Blocks**: Block until specific date/time
- **Recurring Schedules**:
  - 😴 Bedtime (8 PM - 7 AM)
  - 📚 Homework Time (3 PM - 6 PM)  
  - 📅 School Days Only
  - 🎉 Weekends Only
  - ⚙️ Custom Schedules

### 📱 **Supported Apps & Services**
- **Gaming**: 🎮 Fortnite, 🟩 Roblox, 🎮 Twitch, 🎮 Discord, Minecraft
- **Video**: 📺 YouTube, 🎬 Netflix, 🎵 TikTok
- **Social**: 📷 Instagram, 👻 Snapchat
- **Extensible**: Easy to add more apps via configuration

### 🎯 **Device Targeting**
- **All Devices**: Apply rules to entire network
- **Specific Devices**: Target individual phones, tablets, computers
- **Auto-Detection**: Automatically discovers and lists network devices
- **Device Names**: Shows friendly names (iPhone, MacBook, etc.)

### 💾 **Smart State Management**
- **Persistent Rules**: Rules survive browser refresh and app restarts
- **LocalStorage Backup**: Client-side persistence for reliability
- **Real-Time Sync**: Live updates between UniFi controller and interface
- **Status Tracking**: Active/disabled states with visual indicators

### 🔧 **Rule Management**
- **Visual Rule List**: See all active rules with app icons and schedules
- **Quick Controls**: Pause/Resume rules without deleting
- **Rule Details**: Shows schedule type, affected devices, creation time
- **Bulk Actions**: Unblock all rules with single click
- **Smart Deletion**: Removes rules from both UI and UniFi controller

## 🏗️ **Technical Excellence**

### 🦀 **Modern Rust Backend**
- **Axum 0.7**: Latest async web framework with excellent performance
- **Type Safety**: Structured request/response types with Serde
- **Error Handling**: Comprehensive error responses with user-friendly messages
- **Session Management**: Cookie-based authentication with UniFi controllers
- **Logging**: Detailed logging for debugging and monitoring

### 🔐 **Security & Authentication**
- **Local Admin Accounts**: Recommended approach for API access
- **HTTPS Support**: SSL certificate validation (can disable for self-signed)
- **Session Cookies**: Secure session management
- **No API Keys**: Uses standard UniFi username/password authentication

### 📡 **UniFi Integration**
- **Local Network API**: Direct communication with UniFi controller
- **Firewall Rules**: Creates proper firewall rules for app blocking
- **Device Discovery**: Automatically finds connected devices
- **Rule Synchronization**: Keeps UI and controller in sync
- **Graceful Handling**: Handles network errors and controller restarts

### 🌐 **Cross-Platform Deployment**
- **Native Binary**: Single-file executable, no dependencies
- **Docker Support**: Multi-stage builds with Alpine Linux
- **Docker Compose**: Easy local development and testing
- **Fly.io Ready**: Cloud deployment configuration included
- **GitHub Actions**: Automated CI/CD pipeline

## 👥 **Parent-Friendly Design**

### 🎯 **Intuitive User Experience**
- **One-Click Blocking**: No complex configuration needed
- **Visual Feedback**: Clear status messages and confirmations
- **Mobile Optimized**: Easy to use on phones while away from home
- **Quick Presets**: Common scenarios (bedtime, homework) built-in
- **Emergency Access**: Quick unblock for urgent situations

### 📋 **Clear Information Display**
- **Rule Descriptions**: Plain English explanations
- **Time Information**: Shows when rules expire or when they're active
- **Device Lists**: Clear indication of which devices are affected
- **Status Badges**: Color-coded active/disabled indicators

### 🛠️ **Easy Setup**
- **Setup Guide**: Comprehensive documentation for UniFi configuration
- **Auto-Discovery**: Automatically detects controller URL
- **Validation**: Clear error messages for setup issues
- **Testing Tools**: Built-in connection testing

## 🚀 **Getting Started**

1. **Setup UniFi**: Follow [docs/UNIFI_SETUP_GUIDE.md](docs/UNIFI_SETUP_GUIDE.md)
2. **Run Application**: `./target/release/parental-unifi-quick-set`
3. **Open Browser**: Navigate to `http://localhost:3000`
4. **Connect**: Enter your UniFi controller details
5. **Start Blocking**: Use quick actions or create custom rules

## 📈 **Future Enhancements**

- **Content Categories**: Block by content type (social media, gaming, etc.)
- **Time Limits**: Daily screen time limits with automatic blocking
- **User Profiles**: Different rules for different family members
- **Reporting**: Usage statistics and blocked attempt reports
- **Mobile App**: Dedicated iOS/Android apps
- **Voice Integration**: Alexa/Google Home control
- **Geofencing**: Location-based rule activation

## 🎯 **Use Cases**

### 👨‍👩‍👧‍👦 **Family Scenarios**
- **Homework Time**: Block distracting apps during study hours
- **Bedtime Routine**: Ensure devices are offline for better sleep
- **Chore Completion**: Temporary blocks until tasks are done
- **Screen Time Limits**: Healthy digital boundaries
- **Emergency Situations**: Quick unblock for important communications

### 🏠 **Household Management**
- **Guest Network**: Separate controls for visitors
- **Work Hours**: Maintain productivity during remote work
- **Family Events**: Block distractions during family time
- **Educational Focus**: Allow educational apps while blocking entertainment

This modern interface transforms complex network administration into simple, parent-friendly controls that actually get used. 