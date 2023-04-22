mod request;
mod schedulers;
mod storage_device;

use crate::request::Request;
use crate::request::RequestStatus;

use std::fs::File;
use crate::schedulers::Schedulers;
use crate::storage_device::HardDriveDisk;

fn main() {
    simulate(Schedulers::FirstComeFirstServed);
    simulate(Schedulers::ShortestSeekTimeFirst);
    simulate(Schedulers::Scan);
    simulate(Schedulers::CScan);
    wykres();
}

fn simulate(scheduler: Schedulers) {
    // Statistics
    let filename = format!("csv/{:?}.csv", scheduler);
    let writer = csv::Writer::from_writer(File::create(&filename).unwrap());

    // Storage device
    let mut hdd = HardDriveDisk::new(1000, writer);

    // Requests
    let request_generator = request::RequestGenerator::new(hdd.number_of_tracks, 50, 20000, 5000);
    let requests = request_generator.generate_requests();
    let mut normal_requests = requests.clone();
    let mut ready_requests = Vec::new();
    let mut done_requests = Vec::new();

    while done_requests.len() < requests.len() {
        show_progress(scheduler,done_requests.len(), requests.len(), &hdd);

        // add ready requests and remove them from normal requests
        let mut ready_requests_to_add:Vec<Request>= normal_requests.iter()
            .filter(|x| x.arrival_time <= hdd.time)
            .cloned()
            .collect();
        ready_requests_to_add.iter_mut()
            .for_each(|x| x.set_status(RequestStatus::Ready));
        ready_requests.append(&mut ready_requests_to_add);
        normal_requests.retain(|x| x.arrival_time > hdd.time);

        // if there are no ready requests, go to next time unit
        if ready_requests.is_empty() {
            hdd.time += 1;
            continue;
        }

        // schedule request
        let request = schedulers::get_next_request(scheduler, &mut ready_requests, &hdd);

        // go to request if there is one
        if let Some(mut request) = request {
            move_to_request(scheduler, &requests, &mut done_requests, &mut hdd, &request);
            ready_requests.remove(ready_requests.iter().position(|x| x == &request).unwrap());
            request.set_status(RequestStatus::Done);
            done_requests.push(request);
        }
        // idle
        else {
            if scheduler == Schedulers::Scan || scheduler == Schedulers::CScan {
                hdd.move_head(0, scheduler);
            }
            hdd.time += 1;
        }
    }
    show_progress(scheduler, done_requests.len(), requests.len(), &hdd);
    println!();
}

fn move_to_request(scheduler: Schedulers, requests: &Vec<Request>, done_requests: &mut Vec<Request>, hdd: &mut HardDriveDisk, request: &Request) {
    while hdd.current_track != request.track_number {
        hdd.move_head(request.track_number, scheduler);
        hdd.time += 1;
        hdd.seek_count += 1;
        show_progress(scheduler, done_requests.len(), requests.len(), hdd);
    }
}

fn show_progress(scheduler: Schedulers, done_requests: usize, total_requests: usize, hdd: &HardDriveDisk) {
    let scheduler_str = format!("{:?}", scheduler); // convert enum value to string
    print!("\r{:22} | time: {:<8} Progress: {}/{}, current_track: {}", scheduler_str, hdd.time, done_requests, total_requests, hdd.current_track);
}

fn wykres() {
    let output = std::process::Command::new("python")
        .arg("wykres.py")
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("narysowano wykres");
    } else {
        println!("blad przy rysowaniu wykresu");
    }
}
