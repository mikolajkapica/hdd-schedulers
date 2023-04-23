use crate::request::Request;
use crate::storage_device::HardDriveDisk;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Schedulers {
    FirstComeFirstServed,
    ShortestSeekTimeFirst,
    Scan,
    CScan,
    EarliestDeadlineFirst,
    FeasibleDeadlineScan,
}

pub fn get_next_request(scheduler: Schedulers, requests: &mut [Request], hdd: &HardDriveDisk) -> Option<Request> {
    match scheduler {
        Schedulers::FirstComeFirstServed => fcfs(requests),
        Schedulers::ShortestSeekTimeFirst => sstf(requests, hdd),
        Schedulers::Scan => scan(requests, hdd),
        Schedulers::CScan => cscan(requests, hdd),
        Schedulers::EarliestDeadlineFirst => edf(requests),
        Schedulers::FeasibleDeadlineScan => fdscan(requests, hdd),
    }
}

fn fcfs(requests: &mut [Request]) -> Option<Request> {
    requests.sort_by_key(|request| request.arrival_time);
    requests.first().cloned()
}

fn sstf(requests: &mut [Request], hdd: &HardDriveDisk) -> Option<Request> {
    requests.iter()
        .min_by_key(|request| (request.track_number as i32 - hdd.current_track as i32).abs())
        .cloned()
}

fn scan(requests: &[Request], hdd: &HardDriveDisk) -> Option<Request> {
    if hdd.scan_right {
        // filter requests which have track number greater than current track
        requests.iter()
            .filter(|r| r.track_number >= hdd.current_track)
            .min_by_key(|r| r.track_number).cloned()
    } else {
        // filter requests which have track number less than current track
        requests.iter()
            .filter(|r| r.track_number <= hdd.current_track)
            .max_by_key(|r| r.track_number).cloned()
    }
}

fn cscan(requests: &mut [Request], hdd: &HardDriveDisk) -> Option<Request> {
    requests.iter()
        .filter(|r| r.track_number >= hdd.current_track)
        .min_by_key(|r| r.track_number).cloned()
}

fn edf(requests: &[Request]) -> Option<Request> {
    requests.iter()
        .min_by_key(|r| r.deadline_time).cloned()
}

fn fdscan(requests: &[Request], hdd: &HardDriveDisk) -> Option<Request> {
    requests.iter()
        .filter(|r| r.deadline_time >= hdd.time + (r.track_number as i32 - hdd.current_track as i32).unsigned_abs())
        .min_by_key(|r| r.deadline_time).cloned()
}
