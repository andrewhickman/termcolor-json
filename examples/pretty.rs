use termcolor::{ColorChoice, StandardStream};

pub fn main() -> serde_json::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Auto);

    termcolor_json::to_writer(
        &mut stdout.lock(),
        &serde_json::json!({
            "string": "value",
            "number": 123,
            "bool": true,
            "null": null,
        }),
    )?;

    Ok(())
}
