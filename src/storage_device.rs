use std::fs::File;

struct HDD {
    number_of_tracks: i32,
    current_track: i32,
    seek_count: i32,
    time: i32,
    scan_right: bool,
    writer: csv::Writer<File>,
}