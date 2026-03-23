use cactus_foundation::std::version::Version;

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent#syntax
#[derive(Debug)]
pub struct UserAgent {
    name: String,
    version: Option<Version>,
    system_info: Option<String>,
    platform: Option<String>,
    platform_details: Option<String>,
    extensions: Vec<String>,
}

impl UserAgent {
    pub fn new(
        name: String,
        version: Option<Version>,
        system_info: Option<String>,
        platform: Option<String>,
        platform_details: Option<String>,
        extensions: Vec<String>,
    ) -> Self {
        Self {
            name,
            version,
            system_info,
            platform,
            platform_details,
            extensions,
        }
    }

    pub fn from_request(line: &str) -> Result<Self, anyhow::Error> {
        let line = line.trim();

        let (main_part, comment_part) = match line.split_once('(') {
            Some((before, after)) => {
                let comment = after.split(')').next().unwrap_or("").trim();
                (before.trim(), Some(comment.to_string()))
            }
            None => (line, None),
        };

        let mut tokens = main_part.split_whitespace();

        let first = tokens
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty User-Agent"))?;

        let (name, version) = match first.split_once('/') {
            Some((n, v)) => (n.to_string(), Some(v.parse()?)),
            None => (first.to_string(), None),
        };

        let extensions = tokens.map(|s| s.to_string()).collect::<Vec<_>>();

        let (system_info, platform, platform_details) =
            Self::parse_comment(comment_part.as_deref());

        Ok(Self::new(
            name,
            version,
            system_info,
            platform,
            platform_details,
            extensions,
        ))
    }

    fn parse_comment(
        comment: Option<&str>,
    ) -> (Option<String>, Option<String>, Option<String>) {
        if let Some(comment) = comment {
            let parts: Vec<_> = comment.split(';').map(|s| s.trim()).collect();

            let system_info = parts.get(0).map(|s| s.to_string());
            let platform = parts.get(1).map(|s| s.to_string());
            let platform_details = parts.get(2).map(|s| s.to_string());

            (system_info, platform, platform_details)
        } else {
            (None, None, None)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    pub fn system_info(&self) -> Option<&str> {
        self.system_info.as_deref()
    }

    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }

    pub fn platform_details(&self) -> Option<&str> {
        self.platform_details.as_deref()
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}