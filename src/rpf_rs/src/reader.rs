use std::io::SeekFrom;

use crate::Result;

pub struct Reader<D>
where
    D: AsRef<[u8]>,
{
    data: D,
    pos: i64,
    len: i64,
}

impl<D> Reader<D>
where
    D: AsRef<[u8]>,
{
    pub fn new(data: D) -> Self {
        let len = data.as_ref().len();
        Self {
            data,
            pos: 0,
            len: len as i64,
        }
    }
}

impl<D> From<D> for Reader<D>
where
    D: AsRef<[u8]>,
{
    fn from(data: D) -> Self {
        Self::new(data)
    }
}

macro_rules! create_read_def {
    ($type:tt) => {
        paste::item! {
            #[allow(dead_code)]
            #[inline]
            pub fn [<read_ $type>](&mut self) -> Result<$type> {
                let size = $type::BITS as usize / 8;
                self.can_read(size)?;

                let buf = self.get_slice(size);
                Ok($type::from_le_bytes(buf[0..size].try_into().unwrap()))
            }
        }
    };
}

#[allow(dead_code)]
impl<D> Reader<D>
where
    D: AsRef<[u8]>,
{
    #[inline]
    pub fn seek(&mut self, from: SeekFrom) -> Result<()> {
        match from {
            SeekFrom::Start(i) => self.pos = i as i64,
            SeekFrom::Current(i) => self.pos += i,
            SeekFrom::End(i) => self.pos = self.len - i,
        }

        Ok(())
    }

    #[inline]
    pub fn tell(&self) -> i64 {
        self.pos
    }

    #[inline]
    pub fn size(&self) -> i64 {
        self.len
    }

    #[inline]
    pub fn get_slice(&mut self, len: usize) -> &[u8] {
        &self.data.as_ref()[self.pos as usize..len]
    }

    create_read_def!(i8);
    create_read_def!(u8);

    create_read_def!(u16);
    create_read_def!(i16);

    create_read_def!(u32);
    create_read_def!(i32);

    create_read_def!(u64);
    create_read_def!(i64);

    #[inline]
    fn can_read(&self, count: usize) -> Result<()> {
        if self.pos as usize + count > self.len as usize {
            panic!(
                "Tried to read too many bytes! Pos={}, Len={}, Count={}",
                self.pos, self.len, count
            );
        }

        Ok(())
    }
}
