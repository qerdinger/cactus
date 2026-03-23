use cactus_foundation::std::version::Version;

#[derive(Debug)]
pub struct UserAgent {
    name: String,
    version: Option<Version>,
}

impl UserAgent {
    fn new(name: String, version: Option<Version>) -> Self {
        Self { name, version }
    }

    pub fn from_request(line: &str) -> Result<Self, anyhow::Error> {
        match line.trim().split_once("/") {
            Some((name, version)) => {
                Ok(Self::new(name.to_string(), Some(version.parse()?)))
            },
            None => anyhow::bail!("Cannot parse user agent"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &Option<Version> {
        &self.version
    }
}