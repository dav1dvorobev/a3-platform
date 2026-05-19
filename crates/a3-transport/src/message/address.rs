#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Address {
    pub name: String,
    pub second_level_domain: String,
    pub top_level_domain: String,
}

impl Address {
    pub fn from_str(address: &str) -> crate::Result<Self> {
        let address: Vec<&str> = address.split(&['@', '.']).collect();
        if address.len() != 3 {
            return Err(crate::Error::AddressError(
                "invalid address format, expect \"name@second-level-domain.top-level-domain\"",
            ));
        }
        Ok(Self {
            name: address[0].to_owned(),
            second_level_domain: address[1].to_owned(),
            top_level_domain: address[2].to_owned(),
        })
    }
}

impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address = <String as serde::Deserialize>::deserialize(deserializer)?;
        Address::from_str(address.as_str()).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@{}.{}",
            self.name, self.second_level_domain, self.top_level_domain
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_address_correctly() {
        let address = Address::from_str("search@email.local").unwrap();
        assert_eq!(address.name.as_str(), "search");
        assert_eq!(address.second_level_domain.as_str(), "email");
        assert_eq!(address.top_level_domain.as_str(), "local");
    }

    #[test]
    fn serializes_address_correctly() {
        let address = Address {
            name: "search".to_string(),
            second_level_domain: "email".to_string(),
            top_level_domain: "local".to_string(),
        };
        assert_eq!(address.to_string().as_str(), "search@email.local");
    }
}
