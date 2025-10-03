use anyhow::Result;

pub trait Validation {
    fn validate(&self) -> Result<Self>
    where
        Self: Sized;
}
