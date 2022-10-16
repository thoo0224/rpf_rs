use crate::{prelude::Archive, Result};

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
    let _archive = create_test_archive()?;

    Ok(())
}
