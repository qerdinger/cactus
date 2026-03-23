use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: Option<u8>,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: Option<u8>) -> Self {
        Self { major, minor, patch }
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        
        if let [major, minor, patch] = parts.as_slice() {
            Ok(Self::new(major.parse()?, minor.parse()?, Some(patch.parse()?)))
        } else if let [major, minor] = parts.as_slice() {
            Ok(Self::new(major.parse()?, minor.parse()?, None))
        } else {
            anyhow::bail!("Cannot parse version string")
        }
    }

}