# UniFi Setup Guide: Local Admin Account & API Access

**Complete step-by-step guide for configuring UniFi Network Application for parental control applications**

## 📋 **Prerequisites**

Before starting, ensure you have:
- ✅ UniFi Network Application running (version 8.0+ recommended)
- ✅ Administrator access to your UniFi controller
- ✅ Network connectivity to your UniFi controller
- ✅ A strong password generator (recommended)

## 🎯 **Overview**

This guide covers:
1. **Creating a Local Admin Account** with Site Administrator permissions
2. **Enabling API Access** in UniFi Network Application  
3. **Testing the Configuration** to ensure everything works
4. **Security Best Practices** for ongoing management

---

## 📖 **Part 1: Create Local Admin Account**

### **Step 1: Access UniFi Network Application**

#### **For Software-Based Controllers:**
1. Open your web browser
2. Navigate to your UniFi controller URL:
   - `https://[controller-ip]:8443` (e.g., `https://192.168.1.1:8443`)
   - Replace `[controller-ip]` with your actual controller IP

#### **For UniFi OS Consoles (UDM/UDR/Cloud Key):**
1. Open your web browser  
2. Navigate to your console URL:
   - `https://[console-ip]` (e.g., `https://192.168.1.1`)
   - **Note**: UniFi OS consoles do NOT use port `:8443`

### **Step 2: Log In as Administrator**

1. **Accept SSL Certificate Warning** (if present)
   - Click "Advanced" → "Proceed to [IP] (unsafe)"
   - This is normal for self-signed certificates

2. **Enter your current admin credentials**
   - Use your existing administrator account

### **Step 3: Navigate to Admin Management**

#### **For Software-Based Controllers:**
1. Click **"Settings"** in the left sidebar
2. Scroll down and click **"System"**  
3. Click the **"Administration"** tab
4. Click **"Add New Admin"** button

#### **For UniFi OS Consoles:**
1. From UniFi OS home page, click **"Network"** application
2. Click **"Settings"** in the left sidebar
3. Scroll down and click **"System"**
4. Click the **"Administration"** tab  
5. Click **"Add New Admin"** button

**Alternative for UniFi OS:**
1. From UniFi OS home page, click **"Admins"**
2. Click the **"+"** (plus) button to add a new admin

### **Step 4: Configure Local Admin Account**

Fill out the **Add New Admin** form with these **exact settings**:

#### **🔧 Required Settings:**

| Field | Value | Notes |
|-------|-------|-------|
| **Email** | `parental-control@yournetwork.local` | Use a descriptive email |
| **Username** | `parental-control-app` | Choose a clear, descriptive name |
| **Role** | `Site Administrator` | **Critical**: Do NOT use "View Only" |
| **Site Access** | Select your site name | Usually "Default" for single-site setups |
| **Remote Access** | ❌ **DISABLED** | **Critical**: Must be turned OFF |
| **Set Admin Password** | ✅ **ENABLED** | **Critical**: Must be turned ON |
| **Password** | `[generate strong password]` | Use a password manager |

#### **🔐 Password Requirements:**
- Minimum 12 characters
- Mix of uppercase, lowercase, numbers, symbols
- Example: `UniFi$App2024!Secure`
- **Save this password securely!**

#### **⚠️ Critical Settings:**

```
✅ DO:
- Disable "Remote Access" 
- Enable "Set Admin Password"
- Choose "Site Administrator" role
- Use a strong, unique password

❌ DON'T:
- Enable "Remote Access" (security risk)
- Choose "View Only" (won't work for API)
- Use a weak password
- Use your personal admin credentials
```

### **Step 5: Save the Admin Account**

1. **Double-check all settings** against the table above
2. Click **"Invite"** or **"Save"** button
3. **Record the credentials immediately**:
   ```
   Username: parental-control-app
   Password: [your generated password]
   Controller URL: https://[your-controller-ip]:8443
   ```

### **Step 6: Test the New Account**

1. **Open a new browser tab/window**
2. **Navigate to your UniFi controller**
3. **Log in with the new credentials**:
   - Username: `parental-control-app`
   - Password: `[your password]`
4. **If prompted to update password**: 
   - Enter a new strong password
   - **Record the updated password**

✅ **Success**: You should see the UniFi Network Application interface

---

## 🔌 **Part 2: Enable API Access**

### **Understanding UniFi API Access**

**Good News**: The **Local Network API is enabled by default** in modern UniFi Network Application versions (8.0+). No additional configuration is typically required.

### **Verify API Access is Working**

#### **Method 1: Test API Endpoint**

Open a new browser tab and navigate to:
```
https://[controller-ip]:8443/api/self
```

**Expected Results:**
- **Success**: JSON response with site information
- **Failure**: "Login Required" error or connection refused

#### **Method 2: cURL Test (Advanced Users)**

```bash
# Test basic connectivity
curl -k https://[controller-ip]:8443/api/self

# Expected response:
# {"data":[],"meta":{"msg":"api.err.LoginRequired","rc":"error"}}
```

✅ **This error is expected** - it confirms the API endpoint is accessible

### **API Configuration Notes**

#### **✅ What's Already Enabled:**
- **Local Network API**: Available at `https://[controller]:8443/api/`
- **JSON API Endpoints**: All standard endpoints accessible
- **Authentication**: Username/password authentication works
- **HTTPS Access**: SSL/TLS encryption enabled

#### **❌ What You DON'T Need:**
- **Site Manager API Key**: Only for cloud-based Site Manager API
- **Special API Settings**: Local API works out-of-the-box
- **Port Configuration**: Standard HTTPS port (443/8443) is used
- **Additional Certificates**: Self-signed certificates work fine

### **Troubleshooting API Access**

#### **Problem: "Connection Refused"**
```bash
# Check if controller is accessible
ping [controller-ip]

# Check if port is open
telnet [controller-ip] 8443
```

**Solutions:**
1. Verify controller IP address
2. Check firewall settings
3. Ensure UniFi service is running

#### **Problem: SSL Certificate Errors**
**Solution**: Use `-k` flag in cURL or ignore SSL warnings in browsers
```bash
curl -k https://[controller-ip]:8443/api/self
```

#### **Problem: API Returns "Forbidden"**
**Cause**: Account doesn't have sufficient permissions
**Solution**: Verify the account has "Site Administrator" role

---

## 🧪 **Part 3: Test Configuration**

### **Test 1: API Authentication**

Use this cURL command to test login:

```bash
# Replace with your actual values
curl -k -X POST \
  -H "Content-Type: application/json" \
  -d '{"username":"parental-control-app","password":"YOUR_PASSWORD"}' \
  https://[controller-ip]:8443/api/login

# Expected: HTTP 200 with set-cookie header
```

### **Test 2: List Sites** 

```bash
# After successful login, test listing sites
curl -k -X GET \
  -b cookie.txt \
  https://[controller-ip]:8443/api/self/sites

# Expected: JSON array with site information
```

### **Test 3: Application Integration**

1. **Start your parental control application**
2. **Configure with new credentials**:
   - Controller URL: `https://[controller-ip]:8443`
   - Username: `parental-control-app`  
   - Password: `[your password]`
3. **Test basic functionality**:
   - Login to UniFi
   - List devices
   - Create a test traffic rule

---

## 🔒 **Part 4: Security Best Practices**

### **Account Management**

#### **✅ Do:**
- **Dedicated Purpose**: Use this account only for the parental control application
- **Strong Passwords**: Use a password manager to generate and store credentials
- **Regular Rotation**: Change passwords every 90 days
- **Monitor Usage**: Review admin activity logs monthly
- **Document Access**: Keep a secure record of which applications use this account

#### **❌ Don't:**
- **Share Credentials**: Never use this account for other purposes
- **Enable Remote Access**: Keep "Remote Access" disabled
- **Use Personal Accounts**: Don't use your main admin account for applications
- **Store Plaintext**: Don't store passwords in application code or config files

### **Network Security**

#### **Firewall Recommendations:**
```bash
# Allow access only from specific IP ranges
# Example firewall rule (adjust for your setup):
allow from 192.168.1.0/24 to any port 8443
deny from any to any port 8443
```

#### **Network Isolation:**
- Consider placing the application server on a management VLAN
- Restrict access to UniFi controller from other network segments
- Monitor network traffic to/from the controller

### **Monitoring & Auditing**

#### **Regular Checks:**
1. **Monthly**: Review admin activity logs
2. **Quarterly**: Rotate passwords
3. **Semi-Annually**: Review account permissions
4. **Annually**: Full security audit

#### **Activity Monitoring:**
- Check **Settings → System → Logs** for admin activity
- Look for unusual login times or locations
- Monitor API call frequency and patterns

---

## 📞 **Part 5: Troubleshooting**

### **Common Issues & Solutions**

#### **Issue: "Invalid Credentials"**
**Causes:**
- Incorrect username/password
- Account locked or disabled
- Case sensitivity issues

**Solutions:**
1. Double-check credentials (case-sensitive)
2. Reset password through main admin account
3. Verify account is not disabled

#### **Issue: "Insufficient Permissions"**
**Cause**: Account doesn't have Site Administrator role
**Solution**: 
1. Log in as main administrator
2. Edit the service account
3. Change role to "Site Administrator"
4. Save changes

#### **Issue: "SSL Certificate Error"**
**Cause**: Self-signed certificate warnings
**Solutions:**
- **For applications**: Configure to ignore SSL verification
- **For browsers**: Accept certificate warnings
- **For cURL**: Use `-k` flag

#### **Issue: "Connection Timeout"**
**Causes:**
- Network connectivity issues
- Firewall blocking access
- UniFi service not running

**Solutions:**
1. Test basic connectivity: `ping [controller-ip]`
2. Check port accessibility: `telnet [controller-ip] 8443`
3. Verify UniFi service status
4. Review firewall rules

### **Getting Help**

#### **Log Collection:**
Before seeking support, collect:
1. **Application logs** showing the error
2. **UniFi controller logs** from Settings → System → Logs
3. **Network connectivity tests** (ping, telnet)
4. **Account configuration screenshots**

#### **Community Support:**
- **Ubiquiti Community Forums**: community.ui.com
- **GitHub Issues**: For application-specific problems
- **Reddit**: r/Ubiquiti for community help

---

## ✅ **Verification Checklist**

Before concluding setup, verify:

### **Account Configuration:**
- [ ] Local admin account created successfully
- [ ] Account has "Site Administrator" role
- [ ] "Remote Access" is disabled
- [ ] Strong password is set and recorded securely
- [ ] Account can log into UniFi controller

### **API Access:**
- [ ] API endpoints respond (even with "Login Required")
- [ ] Authentication works via API
- [ ] Application can connect to controller
- [ ] Basic API operations succeed

### **Security:**
- [ ] Dedicated service account (not personal account)
- [ ] Strong, unique password generated
- [ ] Credentials stored securely (password manager)
- [ ] Remote access disabled
- [ ] Network access restricted (if applicable)

### **Testing:**
- [ ] Application successfully connects
- [ ] Basic parental control functions work
- [ ] No SSL/certificate issues
- [ ] Performance acceptable

---

## 📚 **Additional Resources**

### **Official Documentation:**
- **UniFi Network API**: developer.ui.com (Site Manager API - different from Local API)
- **UniFi Network Application**: help.ui.com
- **Security Best Practices**: community.ui.com

### **Community Resources:**
- **Art of WiFi Blog**: artofwifi.net (excellent UniFi API resources)
- **UniFi API Browser**: GitHub community tools
- **Ubiquiti Community**: Active support forums

### **Related Guides:**
- **AUTHENTICATION_OPTIONS.md**: Detailed explanation of auth methods
- **Application README**: Your parental control app documentation
- **Network Security**: General UniFi security hardening guides

---

## 🏁 **Conclusion**

You have successfully:
1. ✅ **Created a local admin account** with appropriate permissions
2. ✅ **Verified API access** is working correctly  
3. ✅ **Implemented security best practices** for ongoing management
4. ✅ **Tested the configuration** end-to-end

Your **Parental UniFi Quick Set** application should now be able to:
- Authenticate with your UniFi controller
- Create traffic rules to block applications  
- Manage device access and schedules
- Operate reliably without cloud dependencies

**Next Steps:**
- Configure your application with the new credentials
- Test parental control features
- Set up monitoring and maintenance schedules
- Enjoy secure, reliable parental controls! 🎉 