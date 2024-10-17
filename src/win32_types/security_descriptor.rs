use crate::value::FromValue;

pub struct SecurityDescriptor(sddl::SecurityDescriptor);

impl FromValue for SecurityDescriptor {
    fn from_value_opt(value: &crate::cache::Value) -> crate::ntds::Result<Option<Self>>
    where
        Self: Sized {
        match value {
            crate::cache::Value::Null(_) => Ok(None),
            crate::cache::Value::Binary(vec) | crate::cache::Value::LargeBinary(vec) => 
            {
                Ok(Some(Self(sddl::SecurityDescriptor::from_bytes(&vec[..])?)))
            }
            v => {
                log::error!("I don't know how to extract a security descriptor from {v}");
                Ok(None)
            }
        }
    }
}

impl TryFrom<&[u8]> for SecurityDescriptor {
    type Error = crate::ntds::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(sddl::SecurityDescriptor::try_from(value)?))
    }
}

impl AsRef<sddl::SecurityDescriptor> for SecurityDescriptor {
    fn as_ref(&self) -> &sddl::SecurityDescriptor {
        &self.0
    }
}