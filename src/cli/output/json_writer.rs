use super::Writer;

#[derive(Default)]
pub struct JsonWriter;

impl Writer for JsonWriter {
    fn write_typenames<I>(&self, names: I) -> anyhow::Result<()>
    where
        I: Iterator<Item = String>,
    {
        anyhow::Result::from_iter(names.map(|name| {
            serde_json::to_string_pretty(&name)
                .map_err(|why| anyhow::anyhow!(why))
                .map(|json_string| {
                    println!("{json_string}");
                })
        }))
    }
}
