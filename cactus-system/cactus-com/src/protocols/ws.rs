#[derive(Debug)]
pub struct WSProtocol;

impl TryFrom<&str> for WSProtocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(WSProtocol {})
        } else {
            anyhow::bail!("not http")
        }
    }
}