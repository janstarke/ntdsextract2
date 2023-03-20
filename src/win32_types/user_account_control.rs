use bitflags::bitflags;
use libesedb::Value;
use serde::Serialize;
use anyhow::{anyhow, Result};

use crate::{do_flat_serialization, esedb_utils::FromValue};

bitflags! {

    /// Source: https://docs.microsoft.com/en-us/windows/win32/adschema/a-useraccountcontrol
    #[derive(PartialEq, Eq)]
    pub (crate) struct UserAccountControl : u32 {

        /// The logon script is executed.
        const ADS_UF_SCRIPT = 0x0000_0001;

        /// The user account is disabled.
        const ADS_UF_ACCOUNTDISABLE = 0x0000_0002;

        /// The home directory is required.
        const ADS_UF_HOMEDIR_REQUIRED = 0x0000_0008;

        /// The account is currently locked out.
        const ADS_UF_LOCKOUT = 0x0000_0010;

        /// No password is required.
        const ADS_UF_PASSWD_NOTREQD = 0x0000_0020;

        /// The user cannot change the password.
        const ADS_UF_PASSWD_CANT_CHANGE = 0x0000_0040;

        /// The user can send an encrypted password.
        const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED = 0x0000_0080;

        /// This is an account for users whose primary account is in another
        /// domain. This account provides user access to this domain, but not
        /// to any domain that trusts this domain. Also known as a local user
        /// account.
        const ADS_UF_TEMP_DUPLICATE_ACCOUNT = 0x0000_0100;

        /// This is a default account type that represents a typical user.
        const ADS_UF_NORMAL_ACCOUNT = 0x0000_0200;

        /// This is a permit to trust account for a system domain that trusts
        /// other domains.
        const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT = 0x0000_0800;

        /// This is a computer account for a computer that is a member of this
        /// domain.
        const ADS_UF_WORKSTATION_TRUST_ACCOUNT = 0x0000_1000;

        /// This is a computer account for a system backup domain controller
        /// that is a member of this domain.
        const ADS_UF_SERVER_TRUST_ACCOUNT = 0x0000_2000;

        /// The password for this account will never expire.
        const ADS_UF_DONT_EXPIRE_PASSWD = 0x0001_0000;

        /// This is an MNS logon account.
        const ADS_UF_MNS_LOGON_ACCOUNT = 0x0002_0000;

        /// The user must log on using a smart card.
        const ADS_UF_SMARTCARD_REQUIRED = 0x0004_0000;

        /// The service account (user or computer account), under which a
        /// service runs, is trusted for Kerberos delegation. Any such service
        /// can impersonate a client requesting the service.
        const ADS_UF_TRUSTED_FOR_DELEGATION = 0x0008_0000;

        /// The security context of the user will not be delegated to a service
        /// even if the service account is set as trusted for Kerberos
        /// delegation.
        const ADS_UF_NOT_DELEGATED = 0x0010_0000;

        /// Restrict this principal to use only Data Encryption Standard (DES)
        /// encryption types for keys.
        const ADS_UF_USE_DES_KEY_ONLY = 0x0020_0000;

        /// This account does not require Kerberos pre-authentication for
        /// logon.
        const ADS_UF_DONT_REQUIRE_PREAUTH = 0x0040_0000;

        /// The user password has expired. This flag is created by the system
        /// using data from the Pwd-Last-Set attribute and the domain policy.
        const ADS_UF_PASSWORD_EXPIRED = 0x0080_0000;

        /// The account is enabled for delegation. This is a security-sensitive
        /// setting; accounts with this option enabled should be strictly
        /// controlled. This setting enables a service running under the 
        /// account to assume a client identity and authenticate as that user 
        /// to other remote servers on the network.
        const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION = 0x0100_0000;
    }
}

impl Serialize for UserAccountControl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut flags = Vec::new();
            if self.contains(Self::ADS_UF_SCRIPT) { flags.push("ADS_UF_SCRIPT") }
            if self.contains(Self::ADS_UF_ACCOUNTDISABLE) { flags.push("ADS_UF_ACCOUNTDISABLE") }
            if self.contains(Self::ADS_UF_HOMEDIR_REQUIRED) { flags.push("ADS_UF_HOMEDIR_REQUIRED") }
            if self.contains(Self::ADS_UF_LOCKOUT) { flags.push("ADS_UF_LOCKOUT") }
            if self.contains(Self::ADS_UF_PASSWD_NOTREQD) { flags.push("ADS_UF_PASSWD_NOTREQD") }
            if self.contains(Self::ADS_UF_PASSWD_CANT_CHANGE) { flags.push("ADS_UF_PASSWD_CANT_CHANGE") }
            if self.contains(Self::ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED) { flags.push("ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED") }
            if self.contains(Self::ADS_UF_TEMP_DUPLICATE_ACCOUNT) { flags.push("ADS_UF_TEMP_DUPLICATE_ACCOUNT") }
            if self.contains(Self::ADS_UF_NORMAL_ACCOUNT) { flags.push("ADS_UF_NORMAL_ACCOUNT") }
            if self.contains(Self::ADS_UF_INTERDOMAIN_TRUST_ACCOUNT) { flags.push("ADS_UF_INTERDOMAIN_TRUST_ACCOUNT") }
            if self.contains(Self::ADS_UF_WORKSTATION_TRUST_ACCOUNT) { flags.push("ADS_UF_WORKSTATION_TRUST_ACCOUNT") }
            if self.contains(Self::ADS_UF_SERVER_TRUST_ACCOUNT) { flags.push("ADS_UF_SCRIPT") }
            if self.contains(Self::ADS_UF_DONT_EXPIRE_PASSWD) { flags.push("ADS_UF_DONT_EXPIRE_PASSWD") }
            if self.contains(Self::ADS_UF_MNS_LOGON_ACCOUNT) { flags.push("ADS_UF_MNS_LOGON_ACCOUNT") }
            if self.contains(Self::ADS_UF_SMARTCARD_REQUIRED) { flags.push("ADS_UF_SMARTCARD_REQUIRED") }
            if self.contains(Self::ADS_UF_TRUSTED_FOR_DELEGATION) { flags.push("ADS_UF_TRUSTED_FOR_DELEGATION") }
            if self.contains(Self::ADS_UF_NOT_DELEGATED) { flags.push("ADS_UF_NOT_DELEGATED") }
            if self.contains(Self::ADS_UF_USE_DES_KEY_ONLY) { flags.push("ADS_UF_USE_DES_KEY_ONLY") }
            if self.contains(Self::ADS_UF_DONT_REQUIRE_PREAUTH) { flags.push("ADS_UF_DONT_REQUIRE_PREAUTH") }
            if self.contains(Self::ADS_UF_PASSWORD_EXPIRED) { flags.push("ADS_UF_PASSWORD_EXPIRED") }
            if self.contains(Self::ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION) { flags.push("ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION") }

            if do_flat_serialization() {
                flags.join("|").serialize(serializer)
            } else {
                flags.serialize(serializer)
            }
    }
}

impl FromValue for UserAccountControl {
  fn from_value_opt(value: &Value, attrib_name: &str) -> Result<Option<UserAccountControl>> {
      match value {
          Value::I32(val) => Ok(Some(<UserAccountControl>::from_bits_truncate(
              u32::from_ne_bytes(val.to_ne_bytes()),
          ))),
          Value::Null(()) => Ok(None),
          _ => Err(anyhow!(
              "invalid value detected: {:?} in field {}",
              value,
              attrib_name
          )),
      }
  }
}