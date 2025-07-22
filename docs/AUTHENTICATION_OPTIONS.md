# UniFi Authentication Options for Parental Control App

## Current Implementation: Local Admin (Recommended)

Our app currently uses **local admin accounts** which is the **recommended approach** for UniFi API integrations as of 2024.

### Why Local Admin Accounts?

1. **No MFA Required**: Unlike cloud accounts, local accounts don't require Multi-Factor Authentication
2. **Reliable API Access**: Designed for programmatic access
3. **Dedicated Service Accounts**: Best practice for application integrations
4. **Full Network Control**: Access to all traffic rule APIs

### Setting Up Local Admin (User Instructions)

1. **Access UniFi Controller** at your local IP (e.g., https://192.168.1.1:8443)
2. **Navigate to**: Settings > System > Administration
3. **Click**: "Add New Admin"
4. **Configure**:
   - ✅ **Disable "Remote Access"** (crucial for local-only access)
   - Username: `unifi-parental-control` (or similar)
   - Email: `parental-control@yournetwork.local`
   - Role: **Site Administrator**
   - ✅ **Enable "Set Admin Password"**
   - Set a strong password
5. **Save** and use these credentials in the app

## Alternative Options (Research Notes)

### 1. UniFi Cloud SSO (Not Suitable)

**❌ Problems:**
- Requires MFA since July 2024 (mandatory for all cloud accounts)
- Designed for human users, not applications
- Complex OAuth flow for simple network operations
- Session management complications

**Why Ubiquiti Made This Change:**
> "Starting July 2024, all UniFi cloud accounts will be required to enable Multi-Factor Authentication (MFA) to enhance security measures"

### 2. Site Manager API (Future Consideration)

**API Key Based Authentication:**
```bash
curl -X GET 'https://api.ui.com/v1/hosts' \
  -H 'X-API-KEY: YOUR_API_KEY' \
  -H 'Accept: application/json'
```

**✅ Pros:**
- No username/password required
- Rate limit: 10,000 requests/minute
- Official Ubiquiti API
- Better for monitoring and reporting

**❌ Cons:**
- Currently **read-only**
- May not support traffic rule creation
- Cloud-dependent (requires internet)
- Focused on site management, not device control

**Current Limitations:**
> "The API key is currently read-only. When write endpoints become available, you'll be able to enable write access as needed"

### 3. SAML/SSO Integration (Network Level Only)

**Available for:**
- UniFi platform user authentication (login to UniFi interface)
- Captive portal guest access
- Admin access to UniFi controllers

**❌ Not Available for:**
- Direct API authentication
- Application-to-controller communication
- Programmatic traffic rule management

## Recommended Architecture

### Current (Recommended)
```
Parental App → Local Admin Auth → UniFi Controller → Network Rules
```

### Future Hybrid (When Available)
```
Parental App → {
  Site Manager API (monitoring/read)
  Local Admin Auth (traffic rules/write)
} → UniFi Network
```

## Implementation Status

- ✅ **Current**: Local admin authentication working
- 🔄 **Future**: Monitor Site Manager API for write capabilities
- ❌ **Not Planned**: Cloud SSO integration (due to MFA requirements)

## Security Considerations

1. **Dedicated Service Account**: Never use personal admin accounts
2. **Local-Only Access**: Disable remote access for API accounts
3. **Strong Passwords**: Use generated passwords for service accounts
4. **Network Isolation**: Consider firewall rules for API-only access
5. **Audit Logging**: Monitor account usage in UniFi logs

## User Experience Impact

**Current Approach:**
- Users create one local admin account
- Simple username/password in app
- Works entirely offline/local
- No dependency on Ubiquiti cloud services

**SSO Would Require:**
- Complex OAuth implementation
- MFA handling for every API call
- Internet dependency
- Potential session timeout issues
- Much more complex for users to set up

## Conclusion

The **local admin approach is actually the best practice** for API integrations. While SSO sounds appealing, it would significantly complicate the user experience and add technical challenges without meaningful benefits for this use case. 