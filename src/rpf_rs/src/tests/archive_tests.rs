use crate::{prelude::Archive, Result};
use crate::archive::{Header, RPF_MAGIC};

fn read_test_file<'a>() -> Result<&'a [u8]> {
    let data = include_bytes!("../../resources/dlc.rpf");
    Ok(data)
}

fn create_test_archive<'a>() -> Result<Archive<&'a [u8]>> {
    let data = read_test_file()?;
    Ok(Archive::new(Some("test.rpf".to_owned()), data))
}

#[test]
fn test_read_header() -> Result<()> {
    let mut archive = create_test_archive()?;
    let header = archive.read_header()?;
    header.validate()?;

    assert_eq!(header, Header {
        magic: RPF_MAGIC,
        entry_count: 15,
        names_length: 160,
        decryption_type: 1313165391
    });

    Ok(())
}
