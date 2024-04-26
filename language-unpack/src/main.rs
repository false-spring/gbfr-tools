use anyhow::Context;
use clap::{Parser, Subcommand};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};
use xxhash32_lib::xxhash32_custom;

#[derive(Debug, Deserialize)]
struct LanguageRowColumn {
    id_hash_: String,
    text_: String,
}

#[derive(Debug, Deserialize)]
struct LanguageRow {
    column_: LanguageRowColumn,
}

#[derive(Debug, Deserialize)]
struct LanguageFile {
    rows_: Vec<LanguageRow>,
}

impl LanguageFile {
    pub fn open(file_path: &str) -> anyhow::Result<Self> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let lang_file = rmp_serde::from_read(&mut reader)?;
        Ok(lang_file)
    }

    pub fn to_hashmap(&self) -> std::collections::HashMap<String, String> {
        let mut hashmap = std::collections::HashMap::new();

        for row in &self.rows_ {
            hashmap.insert(row.column_.id_hash_.clone(), row.column_.text_.clone());
        }

        hashmap
    }
}

const LANGUAGES: [&str; 10] = ["bp", "cs", "ct", "en", "es", "fr", "ge", "it", "jp", "ko"];

struct Characters;

impl Characters {
    pub fn extract() -> anyhow::Result<()> {
        let characters = [
            ("Pl0000", "TXT_PL0000"),
            ("Pl0100", "TXT_PL0100"),
            ("Pl0200", "TXT_PL0200"),
            ("Pl0300", "TXT_PL0300"),
            ("Pl0400", "TXT_PL0400"),
            ("Pl0500", "TXT_PL0500"),
            ("Pl0600", "TXT_PL0600"),
            ("Pl0700", "TXT_PL0700"),
            ("Pl0800", "TXT_PL0800"),
            ("Pl0900", "TXT_PL0900"),
            ("Pl1000", "TXT_PL1000"),
            ("Pl1100", "TXT_PL1100"),
            ("Pl1200", "TXT_PL1200"),
            ("Pl1300", "TXT_PL1300"),
            ("Pl1400", "TXT_PL1400"),
            ("Pl1500", "TXT_PL1500"),
            ("Pl1600", "TXT_PL1600"),
            ("Pl1700", "TXT_PL1700"),
            ("Pl1800", "TXT_PL1800"),
            ("Pl1900", "TXT_PL1900"),
            ("Pl2000", "TXT_PL2000"),
            ("Pl2100", "TXT_PL2100"),
            ("Pl2200", "TXT_PL2200"),
            ("Pl2300", "TXT_PL2300"),
            ("Pl2400", "TXT_PL2400"),
        ];

        for language in LANGUAGES {
            let mut output = Map::new();
            let lang_file_path = format!("text/{}/text_chara.msg", language);
            let language_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = language_file.to_hashmap();

            for (id, key) in characters {
                let default = String::new();
                let text = hashmap.get(key).unwrap_or(&default);
                output.insert(id.to_string(), Value::String(text.to_string()));
            }

            let mut output_file =
                File::create(format!("data/{}/characters.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Overmastery;

impl Overmastery {
    fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement =
            db.prepare("SELECT Key, Unk16 FROM limit_bonus_param WHERE Unk16 IS NOT NULL")?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();
            let lang_file_path = format!("text/{}/text_limit_bonus.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    output.insert(
                        key.to_string().to_lowercase(),
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file =
                File::create(format!("data/{}/overmasteries.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Weapon;

impl Weapon {
    pub fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement =
            db.prepare("SELECT Key, Name FROM weapon WHERE Name IS NOT NULL AND Key IS NOT NULL")?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();

            let lang_file_path = format!("text/{}/text.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    let hash = format!("{:08x}", xxhash32_custom(key.as_bytes()));

                    output.insert(
                        hash,
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/weapons.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Sigils;

impl Sigils {
    fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement =
            db.prepare("SELECT Key, Name FROM gem WHERE Name IS NOT NULL AND Key IS NOT NULL")?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();

            let lang_file_path = format!("text/{}/text.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    let hash = format!("{:08x}", xxhash32_custom(key.as_bytes()));

                    output.insert(
                        hash,
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/sigils.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Traits;

impl Traits {
    fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement =
            db.prepare("SELECT Key, Name FROM skill WHERE Name IS NOT NULL AND Key IS NOT NULL")?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();

            let lang_file_path = format!("text/{}/text.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    let hash = format!("{:08x}", xxhash32_custom(key.as_bytes()));

                    output.insert(
                        hash,
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/traits.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Items;

impl Items {
    fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement = db.prepare(
            "SELECT Key, ItemName FROM item WHERE ItemName IS NOT NULL AND Key IS NOT NULL",
        )?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();

            let lang_file_path = format!("text/{}/text.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;
            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    let hash = format!("{:08x}", xxhash32_custom(key.as_bytes()));

                    output.insert(
                        hash,
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/items.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Quests;

impl Quests {
    fn extract() -> anyhow::Result<()> {
        for language in LANGUAGES {
            let mut output = Map::new();
            let lang_file_path = format!("text/{}/text_stage.msg", language);
            let language_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;

            for row in &language_file.rows_ {
                if !row.column_.id_hash_.starts_with("TXT_QR") {
                    continue;
                }

                let quest_id = row.column_.id_hash_.split("_").nth(2);

                if let Some(quest_id) = quest_id {
                    output.insert(
                        quest_id.to_string(),
                        json!({
                            "key": row.column_.id_hash_,
                            "text": row.column_.text_,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/quests.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

struct Enemies;

impl Enemies {
    fn extract(db: &Connection) -> anyhow::Result<()> {
        let mut statement = db.prepare(
            "SELECT KeyMaybe, VariantName1 FROM enemy WHERE KeyMaybe IS NOT NULL AND VariantName1 IS NOT NULL",
        )?;

        for language in LANGUAGES {
            let rows = statement.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut output = Map::new();

            let lang_file_path = format!("text/{}/text_chara.msg", language);
            let lang_file = LanguageFile::open(&lang_file_path).context(format!(
                "Could not open language file at path: {}",
                &lang_file_path
            ))?;

            let hashmap = lang_file.to_hashmap();

            for row in rows {
                let (key, translation_id) = row.unwrap();
                let text = hashmap.get(&translation_id);

                if let Some(text) = text {
                    if text.is_empty() {
                        continue;
                    }

                    // Convert enemy name to titlecase, EM7700 -> Em7700
                    let mut enemy_name = key.clone();

                    enemy_name = enemy_name
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == 0 || !c.is_ascii_alphanumeric() {
                                c.to_ascii_uppercase()
                            } else {
                                c.to_ascii_lowercase()
                            }
                        })
                        .collect();

                    let hash = format!("{:08x}", xxhash32_custom(enemy_name.as_bytes()));

                    output.insert(
                        hash,
                        json!({
                            "key": key,
                            "text": text,
                        }),
                    );
                }
            }

            let mut output_file = File::create(format!("data/{}/enemies.json", language)).unwrap();

            output_file.write(&serde_json::to_string_pretty(&output)?.as_bytes())?;
        }

        Ok(())
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Print { file: PathBuf },
    ExtractAll {},
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match &args.command {
        Commands::Print { file } => {
            let language_file = LanguageFile::open(file.to_str().unwrap())?;

            for row in language_file.rows_ {
                println!("{:?}", row);
            }
        }
        Commands::ExtractAll {} => {
            let default_path = "system_table.sqlite";
            let db = Connection::open(&default_path).context(format!(
                "Could not open sqlite db at path: {}",
                &default_path
            ))?;

            // Create output data directory if it doesn't exist.
            for langauge in LANGUAGES {
                std::fs::create_dir_all(format!("data/{}", langauge)).unwrap();
            }

            Characters::extract()?;
            Overmastery::extract(&db)?;
            Weapon::extract(&db)?;
            Sigils::extract(&db)?;
            Traits::extract(&db)?;
            Items::extract(&db)?;
            Quests::extract()?;
            Enemies::extract(&db)?;
        }
    }

    Ok(())
}
