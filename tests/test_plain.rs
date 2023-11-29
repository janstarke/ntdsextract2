use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write, Cursor},
    path::PathBuf, collections::HashMap,
};

use assert_cmd::Command;
use flate2::read::GzDecoder;
use libntdsextract2::ntds::Person;
use tempfile::NamedTempFile;


#[test]
fn test_plain() {
    let dstfile = read_test_data("ntds_plain.dit.gz");

    let mut cmd = Command::cargo_bin("ntdsextract2").unwrap();
    let result = cmd.arg(dstfile.path()).arg("user").ok();

    match &result {
        Ok(out) => {
            let mut users = HashMap::new();
            let reader = BufReader::new(Cursor::new(&out.stdout));
            let mut rdr = csv::Reader::from_reader(reader);
            for result in rdr.deserialize() {
                let record: Person = result.unwrap();
                users.insert(record.sam_account_name().as_ref().unwrap().clone(), record);
            }

            assert!(users.contains_key("Administrator"));
            assert!(! users.contains_key("InvalidUser"));

            let admin = users.get("Administrator").unwrap();
            assert!(! admin.is_deleted())
        }
        Err(why) => {
            println!("{why}");
        }
    }
    assert!(result.is_ok());
}

fn read_test_data(filename: &str) -> NamedTempFile {
    let dstfile = NamedTempFile::new().unwrap();

    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push(filename);
    let mut reader = BufReader::new(GzDecoder::new(File::open(data_path).unwrap()));

    let mut writer = BufWriter::new(File::create(dstfile.path()).unwrap());

    let mut length = 1;

    while length > 0 {
        let buffer = reader.fill_buf().unwrap();
        let _ = writer.write(buffer);
        length = buffer.len();
        reader.consume(length);
    }
    dstfile
}
