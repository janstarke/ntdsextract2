use serde::Serializer;
use serde::ser::SerializeSeq;

use crate::do_flat_serialization;
use crate::win32_types::ToRfc3339;

pub (crate) fn to_ts<T, S>(ts: &Option<T>, s: S) -> Result<S::Ok, S::Error> where S: Serializer, T: ToRfc3339 {
    match ts {
        Some(ts) =>
            s.serialize_str(&ts.to_rfc3339()),
        None => s.serialize_str("")
    }
}


pub (crate) fn serialize_object_list<S>(ol: &[String], s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if do_flat_serialization() {
        s.serialize_str(&ol.join(","))
    } else {
        let mut seq = s.serialize_seq(None)?;
        for o in ol.iter() {
            seq.serialize_element(o)?;
        }
        seq.end()
    }
}