use crate::reader::Reader;
use crate::{Deserializable, Result};

pub const RPF_MAGIC: u32 = 0x52504637;

#[derive(Debug)]
pub struct Header {
    pub magic: u32,
    pub entry_count: u32,
    pub names_length: u32,
    pub decryption_type: u32,
}

impl<D> Deserializable<D> for Header
where
    D: AsRef<[u8]>,
{
    fn deserialize(reader: &mut Reader<D>) -> Result<Self> {
        Ok(Self {
            magic: reader.read_u32()?,
            entry_count: reader.read_u32()?,
            names_length: reader.read_u32()?,
            decryption_type: reader.read_u32()?,
        })
    }
}

impl Header {
    pub fn validate(&self) -> Result<()> {
        if self.magic != RPF_MAGIC {
            panic!("Invalid RPF magic. {:#x} != {:#x}", self.magic, RPF_MAGIC)
        }

        Ok(())
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.magic == other.magic &&
        self.entry_count == other.entry_count &&
        self.names_length == other.names_length &&
        self.decryption_type == other.decryption_type
    }
}

pub struct Archive<D>
where
    D: AsRef<[u8]>,
{
    pub name: String,
    pub reader: Reader<D>,
}

impl<D> Archive<D>
where
    D: AsRef<[u8]>,
{
    pub fn new(name: Option<String>, data: D) -> Self {
        Self {
            name: name.unwrap_or_else(|| "archive.rpf".to_owned()),
            reader: Reader::from(data),
        }
    }

    pub fn read_header(&mut self) -> Result<Header> {
        Header::deserialize(&mut self.reader)
    }
}

impl<D> From<D> for Archive<D>
where
    D: AsRef<[u8]>,
{
    fn from(data: D) -> Self {
        Self::new(None, data)
    }
}
