use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Whatever {
    name: String,
    lat: f32,
    lon: f32,
    country: String,
    state: String
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;

    let data = r#"
        [
        {
          "name": "Toronto",
          "local_names": {
            "ur": "پرانا ٹورانٹو",
            "pl": "Toronto",
            "ps": "ټورنټو",
            "es": "Toronto",
            "pt": "Toronto",
            "ar": "تورونتو القديمة",
            "oc": "Toronto",
            "el": "Παλαιό Τορόντο",
            "fa": "تورنتو",
            "en": "Toronto",
            "gr": "Τορόντον",
            "fr": "Toronto",
            "ug": "تورونتو",
            "hy": "Տորոնտո",
            "de": "Toronto",
            "pa": "ਟੋਰਾਂਟੋ",
            "ca": "Toronto"
          },
          "lat": 43.6534817,
          "lon": -79.3839347,
          "country": "CA",
          "state": "Ontario"
        }
      ]"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Vec<Whatever> = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p[0].name, p[0].country);

    Ok(())
}

fn main() {
    typed_example();
}