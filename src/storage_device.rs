use std::fs::File;
use crate::schedulers::Schedulers;

pub struct HardDriveDisk {
    pub number_of_tracks: u32,
    pub current_track: u32,
    pub seek_count: u32,
    pub time: u32,
    pub scan_right: bool,
    writer: csv::Writer<File>,
}

impl HardDriveDisk {
    pub fn new(number_of_tracks: u32, writer: csv::Writer<File>) -> HardDriveDisk {
        HardDriveDisk {
            number_of_tracks,
            current_track: number_of_tracks/2,
            seek_count: 0,
            time: 0,
            scan_right: true,
            writer,
        }
    }
    pub fn move_head(&mut self, target_track_number: u32, scheduler: Schedulers) {
        self.seek_count += 1;

        match scheduler {
            Schedulers::Scan => {
                if self.scan_right {
                    self.current_track += 1;
                } else {
                    self.current_track -= 1;
                }

                if self.current_track == self.number_of_tracks - 1 {
                    self.scan_right = false;
                } else if self.current_track == 0 {
                    self.scan_right = true;
                }
            }
            Schedulers::CScan => {
                if self.current_track == self.number_of_tracks - 1 {
                    self.current_track = 0;
                } else {
                    self.current_track += 1;
                }
            }
            _ => {
                if self.current_track < target_track_number {
                    self.current_track += 1;
                } else {
                    self.current_track -= 1;
                }
            }
        }

        self.write_to_file();
        }
    pub fn write_to_file(&mut self) {
        self.writer.serialize((self.time, self.current_track)).unwrap();
    }
}