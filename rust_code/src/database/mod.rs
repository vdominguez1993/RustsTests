use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::time::SystemTime;

const DB_PATH: &str = "../db/temperature.db";
const CPU_TEMP_TABLE: &str = "cpu_temp";
const TIME_COLUMN: &str = "time";
const VALUE_COLUMN: &str = "value";

const THERMAL_ZONE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn update_data() {
    check_db_existence().expect("Some error happened with the DB");
    add_data().expect("Error adding data");
}

struct DataTemp {
    time: i32,
    temp: f64,
}

pub fn get_last_entries(num_entries: i32) -> Result<String> {
    let conn = Connection::open(DB_PATH)?;
    let query: String = match num_entries {
        x if x > 0 => format!(
            "SELECT {time}, {} FROM {} ORDER BY {time} DESC LIMIT {}",
            VALUE_COLUMN,
            CPU_TEMP_TABLE,
            num_entries,
            time = TIME_COLUMN
        ),
        _ => format!(
            "SELECT {time}, {} FROM {} ORDER BY {time} DESC",
            VALUE_COLUMN,
            CPU_TEMP_TABLE,
            time = TIME_COLUMN
        ),
    };

    let mut stmt = conn.prepare(&query)?;
    let data_iter = stmt.query_map(params![], |row| {
        Ok(DataTemp {
            time: row.get(0)?,
            temp: row.get(1)?,
        })
    })?;

    let mut entries = String::from("{\"Data\": [");

    for data in data_iter {
        let data = data?;
        entries.push_str(&format!(
            "\r\n{{\"Time\":{}, \"Temp\": {}}},",
            data.time, data.temp
        ));
    }
    entries.pop();
    entries.push_str("\r\n]}");

    Ok(entries)
}

fn check_db_existence() -> Result<()> {
    // Check if file exists
    let is_file = match fs::metadata(DB_PATH) {
        // Return if it is a file
        Ok(file_handler) => file_handler.is_file(),
        // False if file does not exist or can't be opened
        Err(_) => false,
    };

    if is_file == false {
        // Create the db and its table

        let conn = Connection::open(DB_PATH)?;
        let command = format!(
            "create table if not exists {} (
            id integer primary key,
            {} integer not null,
            {} real not null
        )",
            CPU_TEMP_TABLE, TIME_COLUMN, VALUE_COLUMN
        );

        conn.execute(&command, NO_PARAMS)?;
    }

    Ok(())
}

fn add_data() -> Result<()> {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let temperature = get_temperature();
    let conn = Connection::open(DB_PATH)?;
    let command = format!(
        "INSERT INTO {} ({}, {}) values (?1, ?2)",
        CPU_TEMP_TABLE, TIME_COLUMN, VALUE_COLUMN
    );

    conn.execute(&command, &[&time.to_string(), &temperature.to_string()])?;

    Ok(())
}

fn get_temperature() -> f32 {
    let value_in_file = fs::read_to_string(THERMAL_ZONE_FILE).expect("Error reading temperature");
    let integer_value: u32 = value_in_file.trim().parse().expect("Error parsing temp");

    (integer_value / 1000) as f32
}
