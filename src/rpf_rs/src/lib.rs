use std::error::Error;

use reader::Reader;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub mod archive;
pub mod prelude;
pub mod reader;
pub mod native;

#[cfg(test)]
mod tests;

pub(crate) trait Deserializable<D>
where
    D: AsRef<[u8]>,
{
    fn deserialize(reader: &mut Reader<D>) -> Result<Self>
    where
        Self: Sized;
}
