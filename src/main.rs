use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use chrono::{NaiveDate};
use serde_xml_rs::EventReader;

const PATH_TO_SEARCH: &str =
    "C:\\Users\\mdienel\\IdeaProjects\\google-fit-statistics\\data\\Google_Fit\\Aktivitäten";


fn main() {
    println!("Starting");
    let bike_files = get_files("Radfahren");
    let mut total_distance = 0f32;
    for file in bike_files {
        let distance_in_meters_for_file = get_distance_in_m_from_file(file);
        println!("Distance {}", distance_in_meters_for_file);
        total_distance += distance_in_meters_for_file;
    }
    println!("Total distance {}", total_distance);
}

fn get_distance_in_m_from_file(file_path: String) -> f32 {
    // let xml_content = fs::read_to_string(file_path).expect("Failed reading file");
    let file = File::open(&file_path).unwrap();
    let values: Vec<f32> = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.contains("DistanceMeters"))
        .map(|line| get_value_from_line(line))
        .collect();

    let mut max_value = 0f32;
    for x in values {
        if x > max_value {
            max_value = x;
        }
    }
    return max_value;
}

fn get_value_from_line(line: String) -> f32 {
    let string_value_opt = line.trim().strip_prefix("<DistanceMeters>")
        .map(|prefix_stripped| prefix_stripped.strip_suffix("</DistanceMeters>"))
        .flatten();
    if string_value_opt.is_some() {
        let string_value = string_value_opt.unwrap();
        let value = str::parse::<f32>(string_value);
        return value.unwrap();
    }
    return 0f32;
}

fn get_files(filetype: &str) -> Vec<String> {
    let paths: Vec<PathBuf> = fs::read_dir(Path::new(PATH_TO_SEARCH))
        .unwrap()
        .filter_map(|element| element.ok())
        .map(|dir_entry| dir_entry.path())
        .collect();
    let file_paths = paths
        .iter()
        .filter_map(|path| path.to_str())
        .filter(|file_path| file_path.contains(filetype))
        .filter(|file_path| has_valid_date(file_path))
        .map(|file_name| String::from(file_name))
        .collect();

    return file_paths;
}

fn has_valid_date(file_path: &str) -> bool {
    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2019, 04, 01).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
    let (year, month, day) = Path::new(file_path).file_name()
        .map(|file_name| get_year_month_day(file_name.to_str().unwrap()))
        .unwrap();
    let file_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    start_date < file_date && file_date < end_date
}

fn get_year_month_day(file_name: &str) -> (i32, u32, u32) {
    let year = str::parse::<i32>(&file_name[..4]).unwrap();
    let month = str::parse::<u32>(&file_name[5..7]).unwrap();
    let day = str::parse::<u32>(&file_name[8..10]).unwrap();
    (year, month, day)
}
