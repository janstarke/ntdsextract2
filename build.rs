use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use lazy_regex::regex_captures;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use convert_case::Casing;
use convert_case::Case;
use std::fmt::Display;
use core::fmt::Formatter;

struct AppId {
    id_name: String,
    numeric_id: u32,
    ntds_name: String
}
impl AppId {
    fn new (
        id_name: String,
        numeric_id: u32,
        ntds_name: String) -> Self {
        Self {
            id_name,
            numeric_id,
            ntds_name
        }
    }
    fn try_from(line: &str) -> Option<Self> {
        if let Some((_, id_name, numeric_id, ntds_name)) = regex_captures!(r#"#define  (ATT_[A-Z0-9_]+)\s+0x([0-9a-fA-F]+)\s+//\s*(\w+)"#, line) {
            let numeric_id = if numeric_id.len() % 2 == 1 {
                format!("0{numeric_id}")
            } else {
                numeric_id.to_string()
            };

            match hex::decode(&numeric_id) {
                Ok(mut numeric_id) => {
                    while numeric_id.len() < 4 {
                        numeric_id.insert(0, 0);
                    }
                    Some(Self{
                        id_name: id_name.to_case(Case::UpperCamel),
                        numeric_id: BigEndian::read_u32(&numeric_id),
                        ntds_name: ntds_name.to_string(),
                    })
                }
                Err(why) => panic!("invalid numeric id: '{numeric_id}': {why}")
            }
        } else {
            None
        }
    }
}

impl Display for AppId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, r#"    #[strum(serialize = "{}")]"#, self.ntds_name)?;
        writeln!(f, r#"    {} = 0x{:x},"#, self.id_name, self.numeric_id)?;
        Ok(())
    }
}

fn main() {

    let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let reader = io::BufReader::new(fs::File::open("misc/attids.h").unwrap());
    let mut out_file = io::BufWriter::new(fs::File::create(Path::new(&out_dir).join("src").join("ntds").join("attribute_id.rs")).unwrap());

    writeln!(out_file, "use strum::{{EnumString, IntoStaticStr}};\n").unwrap();
    writeln!(out_file, "#[derive(IntoStaticStr, EnumString, Debug, Eq, PartialEq, Hash, Clone)]").unwrap();
    writeln!(out_file, "#[strum(use_phf)]").unwrap();
    writeln!(out_file, "pub enum NtdsAttributeId {{").unwrap();

    for line in reader.lines() {
        if let Some(app_id) = AppId::try_from(&line.unwrap()) {
            write!(out_file, "{app_id}").unwrap();
        }
    }

    write!(out_file, "{}", AppId::new("DsRecordId".to_owned(), 0xffffff01, "DNT_col".to_owned())).unwrap();
    write!(out_file, "{}", AppId::new("DsParentRecordId".to_owned(), 0xffffff02, "PDNT_col".to_owned())).unwrap();
    write!(out_file, "{}", AppId::new("DsRecordTime".to_owned(), 0xffffff03, "time_col".to_owned())).unwrap();
    write!(out_file, "{}", AppId::new("DsAncestors".to_owned(), 0xffffff04, "Ancestors_col".to_owned())).unwrap();

    writeln!(out_file, "}}").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=misc/attids.h");
}