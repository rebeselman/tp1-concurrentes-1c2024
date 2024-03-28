//! Hi! :), this is my implementation of the tp1
use config::Config;
use serde_json::{self, json};
use std::error::Error;
use std::process::Command;
pub mod config;

/// Function which runs the application
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    // descomprimir en la carpeta data usando el comando download_data.s
    let coso = Command::new("/bin/sh").arg("download_data.sh").output()?;
    println!("{:?}", coso);
    let json_data = json!({
            "padron": 108672,
            "sites": {
                "site1": {
                    "questions": "cantidad total de preguntas para ese sitio>",
                    "words": "cantidad total de palabras para ese sitio",
                    "tags": {
                        "tag1": {
                            "questions": "cantidad total de preguntas para ese tag para ese sitio",
                            "words": "cantidad total palabras para ese tag para ese sitio",
                        },

                        "tagN": {

                        },
                    },
                    "chatty_tags": [
                        "tag1", "tag2"
                    ]
                },
                "siteN" : {
                }
            },
            "tags": {
                "tag1": {
                    "questions": "cantidad total de preguntas para ese tag para todos los sitios",
                    "words": "cantidad total palabras para ese tag para todos los sitios",
                },
                "tagN": {

                },
            },
            "totals": {
                "chatty_sites": [
                    "site1", "site2"
                ],
                "chatty_tags": [
                    "tag1", "tag2"
                ]
            }
    });
    let formatted_json = serde_json::to_string_pretty(&json_data).map_err(|err| err.to_string())?;
    println!("{}", formatted_json);

    Ok(())
}
