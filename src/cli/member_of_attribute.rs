use strum::Display;

#[derive(clap::ValueEnum, Clone, Copy, Display, Hash, Eq, PartialEq)]
pub enum MemberOfAttribute {
    /// show the Security ID (SID)
    #[strum(serialize = "sid")]
    Sid,

    /// show the relative distinguished name (RDN) value
    #[strum(serialize = "rdn")]
    Rdn,

    /// show the distinguished name (DN)
    #[strum(serialize = "dn")]
    Dn
}