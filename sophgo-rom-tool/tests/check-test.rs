use sophgo_rom_tool::Error;

#[test]
fn check_success_raw_blob() {
    let content = vec![
        0x6F, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
    ];
    let ans = sophgo_rom_tool::check(&content).unwrap();
    assert_eq!(
        ans.refill_header,
        Some(sophgo_rom_tool::HeaderInfo {
            blcp_image_checksum: 0xCAFE0000,
            bl2_image_checksum: 0xCAFE8761,
            bl2_image_size: 512
        })
    );
    assert_eq!(ans.resize_image_full_length, 4608);
}

#[test]
fn check_error_head_length() {
    let content = vec![0x6F, 0x00, 0x00];
    let ans = sophgo_rom_tool::check(&content);
    assert_eq!(ans, Err(Error::HeadLength { wrong_length: 3 }))
}

#[test]
fn check_error_magic_number() {
    let content = vec![0x44, 0x33, 0x22, 0x11];
    let ans = sophgo_rom_tool::check(&content);
    assert_eq!(
        ans,
        Err(Error::MagicNumber {
            wrong_magic: 0x11223344
        })
    )
}

#[test]
fn check_error_raw_blob_head_length() {
    let content = vec![
        0x6F, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6F, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let ans = sophgo_rom_tool::check(&content);
    assert_eq!(ans, Err(Error::HeadLength { wrong_length: 20 }))
}

#[test]
fn check_error_raw_blob_magic() {
    let content = vec![
        0x6F, 0x00, 0x00, 0x02, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x11, 0x45, 0x14, 0x19, 0x19,
        0x81, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0, 0x1,
    ];
    let ans = sophgo_rom_tool::check(&content);
    assert_eq!(
        ans,
        Err(Error::RawBlobMagic {
            wrong_magic: [
                0x6F, 0x00, 0x00, 0x02, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x11, 0x45, 0x14, 0x19,
                0x19, 0x81, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x11, 0x45, 0x14, 0x19, 0x19,
                0x81, 0x0, 0x1
            ]
        })
    )
}