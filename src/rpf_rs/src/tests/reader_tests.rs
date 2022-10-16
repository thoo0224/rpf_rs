use std::io::SeekFrom;
use std::vec;

use crate::reader::Reader;
use crate::Result;

macro_rules! create_test_read_def {
    ($type_name:ident) => {
        paste::item! {
            #[test]
            fn [<test_read_ $type_name >]() -> Result<()> {
                let val = $type_name::MAX / 2;
                let buf = val.to_le_bytes();
                let mut reader = create_reader_of(&buf);
                let res = reader.[<read_ $type_name>]()?;
                assert_eq!(res, val);

                Ok(())
            }
        }
    };
}

fn create_reader_of<D>(data: D) -> Reader<D>
where
    D: AsRef<[u8]>,
{
    Reader::from(data)
}

#[test]
fn test_seek() -> Result<()> {
    let mut reader = create_reader_of(vec![0u8; 50]);
    assert_eq!(reader.tell(), 0);

    reader.seek(SeekFrom::Start(10))?;
    assert_eq!(reader.tell(), 10);

    reader.seek(SeekFrom::Current(10))?;
    assert_eq!(reader.tell(), 20);

    reader.seek(SeekFrom::End(10))?;
    assert_eq!(reader.tell(), reader.size() - 10);

    Ok(())
}

#[test]
fn test_tell() -> Result<()> {
    let mut reader = create_reader_of(vec![0u8; 50]);
    reader.seek(SeekFrom::Current(10))?;
    assert_eq!(reader.tell(), 10);

    Ok(())
}

create_test_read_def!(u8);
create_test_read_def!(i8);

create_test_read_def!(u16);
create_test_read_def!(i16);

create_test_read_def!(u32);
create_test_read_def!(i32);

create_test_read_def!(u64);
create_test_read_def!(i64);
