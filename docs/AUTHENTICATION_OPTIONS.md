# UniFi Authentication Options for Parental Control App

## Current Implementation: Local Admin (Recommended)

Our app currently uses **local admin accounts** which is the **recommended approach** for UniFi API integrations as of 2024.

### Why Local Admin Accounts?

1. **No MFA Required**: Unlike cloud accounts, local accounts don't require Multi-Factor Authentication
2. **Reliable API Access**: Designed for programmatic access
3. **Dedicated Service Accounts**: Best practice for application integrations
4. **Full Network Control**: Access to all traffic rule APIs

## Research: Site Manager API Alternative (Not Suitable)

We investigated using the **UniFi Site Manager API** with SSO authentication as an alternative approach, but found it **unsuitable for parental control applications**:

### ❌ **Why Site Manager API Won't Work:**

1. **Read-Only Limitation**: 
   - Current v1.0 API is explicitly read-only
   - Cannot create, update, or delete traffic rules
   - No write capabilities for firewall management

2. **Missing Core Features**:
   - No traffic rule endpoints
   - No device blocking/unblocking capabilities  
   - No parental control management functions
   - Limited to basic device info and metrics

3. **Available Endpoints** (v1.0):
   - List Hosts/Sites/Devices (read-only)
   - ISP Metrics (read-only)
   - SD-WAN Configs (read-only)
   - **No traffic management functionality**

4. **Future Uncertainty**: 
   - Documentation promises write capabilities "when available"
   - No timeline provided for traffic rule management
   - Current application needs immediate functionality

### 📊 **API Comparison Summary**

| Capability | Local Network API | Site Manager API |
|------------|------------------|------------------|
| Create Traffic Rules | ✅ Full Support | ❌ Not Available |
| Block Applications | ✅ Full Support | ❌ Not Available |
| Device Management | ✅ Full Control | ❌ Read-Only Info |
| Real-time Changes | ✅ Immediate | ❌ No Write Access |
| Works Offline | ✅ Yes | ❌ Cloud Dependent |

### Setting Up Local Admin (User Instructions)

1. **Access UniFi Controller** at your local IP (e.g., https://192.168.1.1:8443)
2. **Navigate to**: Settings > System > Administration
3. **Click**: "Add New Admin"
4. **Configure**:
   - ✅ **Disable "Remote Access"** (this makes it local-only)
   - ✅ **Set strong password**
   - ✅ **Choose appropriate permissions** (Site Administrator for full access)
   - ✅ **Document credentials securely**

### Best Practices

1. **Dedicated Service Account**: Create a specific admin account just for this application
2. **Strong Password**: Use a complex, unique password
3. **Minimal Permissions**: Grant only necessary access levels
4. **Regular Rotation**: Change passwords periodically
5. **Monitor Usage**: Review admin activity logs regularly

## Alternative Authentication Methods (Not Recommended)

### ❌ **UniFi Cloud Accounts (Problems)**
- **MFA Requirement**: Since July 2024, all cloud accounts require Multi-Factor Authentication
- **API Incompatibility**: MFA breaks programmatic API access
- **Internet Dependency**: Requires connection to Ubiquiti cloud services
- **Complexity**: OAuth flows add unnecessary complexity

### ❌ **Site Manager API (Current Limitations)**
- **Read-Only Access**: Cannot modify traffic rules or device settings
- **Missing Features**: No parental control or traffic management capabilities
- **Future Dependency**: Write access promised but not available
- **Cloud Dependency**: Requires internet connection and Ubiquiti account

## Conclusion

**Local admin accounts remain the best and only viable option** for UniFi parental control applications. The Site Manager API, while offering SSO integration, lacks the fundamental write capabilities needed for traffic rule management and device control.

Our current implementation following local admin best practices provides:
- ✅ Full functionality access
- ✅ Reliable, offline operation  
- ✅ Immediate real-time control
- ✅ Industry-standard security practices
- ✅ No external dependencies 