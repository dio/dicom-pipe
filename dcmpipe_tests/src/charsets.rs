use crate::{parse_file, parse_all_values};
use dcmpipe_dict::dict::dicom_elements as tags;
use dcmpipe_lib::core::charset::CSRef;
use dcmpipe_lib::core::dcmelement::DicomElement;
use dcmpipe_lib::core::dcmobject::{DicomNode, DicomRoot};
use encoding::all;
use std::io::Error;

/// This DICOMDIR has sequences with nested elements that change charsets
#[test]
fn test_parse_nested_charset_values() -> Result<(), Error> {
    let dcmroot: DicomRoot = parse_file(
        "./fixtures/dclunie/charsettests/DICOMDIR",
        true,
    )?;

    parse_all_values(&dcmroot)
}

#[test]
fn test_scs_arab() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSARAB",
        all::ISO_8859_6,
        "ISO_IR 127",
        "قباني^لنزار",
    )
}

#[test]
fn test_scs_french() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSFREN",
        all::WINDOWS_1252,
        "ISO_IR 100",
        "Buc^Jérôme",
    )
}

#[test]
fn test_scs_german() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSGERM",
        all::WINDOWS_1252,
        "ISO_IR 100",
        "Äneas^Rüdiger",
    )
}

#[test]
fn test_scs_greek() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSGREEK",
        all::ISO_8859_7,
        "ISO_IR 126",
        "Διονυσιος",
    )
}

#[test]
fn test_scs_h31() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSH31",
        all::ISO_2022_JP,
        "ISO 2022 IR 87",
        "Yamada^Tarou=山田^太郎=やまだ^たろう",
    )
}

#[test]
fn test_scs_h32() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSH32",
        all::ISO_2022_JP,
        "ISO 2022 IR 13",
        "",
    )
}

#[test]
fn test_scs_hebrew() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSHBRW",
        all::ISO_8859_8,
        "ISO_IR 138",
        "שרון^דבורה",
    )
}

#[test]
fn test_scs_i12() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSI2",
        all::WINDOWS_1252,
        "ISO 2022 IR 149",
        "Hong^Gildong=\u{1b}$)Cûó^\u{1b}$)CÑÎÔ×=\u{1b}$)CÈ«^\u{1b}$)C±æµ¿",
    )
}

#[test]
fn test_scs_russian() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSRUSS",
        all::ISO_8859_5,
        "ISO_IR 144",
        "Люкceмбypг",
    )
}

#[test]
fn test_scs_x1() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSX1",
        all::UTF_8,
        "ISO_IR 192",
        "Wang^XiaoDong=王^小東=",
    )
}

#[test]
fn test_scs_x2() -> Result<(), Error> {
    test_scs_file(
        true,
        "./fixtures/dclunie/charsettests/SCSX2",
        all::GB18030,
        "GB18030",
        "Wang^XiaoDong=王^小东=",
    )
}

fn test_scs_file(with_std: bool, path: &str, cs: CSRef, scs: &str, pn: &str) -> Result<(), Error> {
    let dcmroot: DicomRoot = parse_file(path, with_std)?;

    assert_eq!(dcmroot.get_cs().name(), cs.name());

    let scs_elem: &DicomElement = dcmroot
        .get_child(tags::SpecificCharacterSet.tag)
        .expect("Should have SCS")
        .as_element();

    // value can be multiple and sometimes contains empties -- match logic from the parser
    let scs_val: String = scs_elem
        .parse_strings()?
        .into_iter()
        .filter(|cs_entry: &String| !cs_entry.is_empty())
        .nth(0)
        .expect("Should have at least one value for SCS");
    assert_eq!(scs_val, scs);

    let pn_elem: &DicomElement = dcmroot
        .get_child(tags::PatientsName.tag)
        .expect("Should have PN")
        .as_element();

    let pn_val: String = pn_elem.parse_string()?;
    assert_eq!(pn_val, pn);

    Ok(())
}
