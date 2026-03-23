/*
Priority:

1st: BR
2nd: Zstd
3rd: Gzip
 */
use std::collections::HashSet;

#[derive(Debug)]
pub enum HttpEncoding {
    Br,
    Zstd,
    Gzip,
}

impl TryFrom<&str> for HttpEncoding {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let encodings = value
            .split(",")
            .map(str::trim)
            .collect::<HashSet<&str>>();

        if encodings.contains("br") {
            return Ok(Self::Br)
        } else if encodings.contains("zstd") {
            return Ok(Self::Zstd)
        } else if encodings.contains("gzip") {
            return Ok(Self::Gzip)
        }

        anyhow::bail!(format!("HTTP encoding [{}] not supported", value))
    }
}