use std::{
    io::{BufReader, Cursor},
    path::PathBuf, collections::HashMap,
};

use assert_cmd::Command;
use libntdsextract2::{ntds::Person, CsvSerialization};


#[test]
fn test_plain() {
    let dstfile = get_test_data("ntds_plain.dit");

    let mut cmd = Command::cargo_bin("ntdsextract2").unwrap();
    let result = cmd.arg(dstfile).arg("user").ok();

    match &result {
        Ok(out) => {
            let mut users = HashMap::new();
            let reader = BufReader::new(Cursor::new(&out.stdout));
            let mut rdr = csv::Reader::from_reader(reader);
            
            for result in rdr.deserialize() {
                let record: Person<CsvSerialization> = result.unwrap();
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

fn get_test_data(filename: &str) -> PathBuf {
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push(filename);

    data_path
}
