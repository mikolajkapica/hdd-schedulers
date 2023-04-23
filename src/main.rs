mod request;
mod schedulers;
mod storage_device;

use crate::request::Request;
use crate::request::RequestStatus;
use std::fs::File;
use crate::schedulers::Schedulers;
use crate::storage_device::HardDriveDisk;
use std::io::repeat;

const NUMBER_OF_TRACKS: u32 = 500;
const NUMBER_OF_REQUESTS: u32 = 100;
const MAX_ARRIVAL_TIME: u32 = 25_000;
const DEADLINE_TIME: u32 = 15_000;
const RT_CHANCE: f32 = 0.5;


fn main() {
    simulate(Schedulers::FirstComeFirstServed);
    simulate(Schedulers::ShortestSeekTimeFirst);
    simulate(Schedulers::Scan);
    simulate(Schedulers::CScan);
    simulate_with_rt(Schedulers::FirstComeFirstServed,Schedulers::EarliestDeadlineFirst);
    simulate_with_rt(Schedulers::FirstComeFirstServed,Schedulers::FeasibleDeadlineScan);
    simulate_with_rt(Schedulers::ShortestSeekTimeFirst,Schedulers::EarliestDeadlineFirst);
    simulate_with_rt(Schedulers::ShortestSeekTimeFirst,Schedulers::FeasibleDeadlineScan);
    simulate_with_rt(Schedulers::Scan,Schedulers::EarliestDeadlineFirst);
    simulate_with_rt(Schedulers::Scan,Schedulers::FeasibleDeadlineScan);
    simulate_with_rt(Schedulers::CScan,Schedulers::EarliestDeadlineFirst);
    simulate_with_rt(Schedulers::CScan,Schedulers::FeasibleDeadlineScan);
    wykres();
}

fn simulate_with_rt(scheduler: Schedulers, rt_scheduler: Schedulers) {
    // Statistics
    let filename = format!("csv/{:?}-{:?}.csv", scheduler, rt_scheduler);
    let writer = csv::Writer::from_writer(File::create(&filename).unwrap());

    // Storage device
    let mut hdd = HardDriveDisk::new(NUMBER_OF_TRACKS, writer);

    // Requests
    let request_generator = request::RequestGenerator::new(hdd.number_of_tracks, NUMBER_OF_REQUESTS, MAX_ARRIVAL_TIME, DEADLINE_TIME, RT_CHANCE);
    let requests = request_generator.generate_requests();
    let mut normal_requests:Vec<Request> = requests.iter().filter(|x| x.status == RequestStatus::Normal).cloned().collect();
    let mut rt_requests:Vec<Request> = requests.iter().filter(|x| x.status == RequestStatus::RealTime).cloned().collect();
    let mut ready_requests = Vec::new();
    let mut ready_rt_requests:Vec<Request> = Vec::new();
    let mut done_requests = Vec::new();
    let mut dead_requests = Vec::new();
    while done_requests.len() + dead_requests.len() < requests.len() {
        show_progress_with_rt(rt_scheduler, scheduler,done_requests.len(), requests.len(), &hdd);

        // add ready requests and remove them from normal requests
        let mut ready_requests_to_add:Vec<Request> = normal_requests.iter()
            .filter(|x| x.arrival_time <= hdd.time)
            .cloned()
            .collect();
        ready_requests_to_add.iter_mut()
            .for_each(|x| x.set_status(RequestStatus::Ready));
        ready_requests.append(&mut ready_requests_to_add);
        normal_requests.retain(|x| x.arrival_time > hdd.time);

        // add dead requests and remove them from rt requests
        let mut dead_requests_to_add:Vec<Request> = rt_requests.iter()
            .filter(|x| x.deadline_time <= hdd.time)
            .cloned()
            .collect();
        dead_requests_to_add.iter_mut()
            .for_each(|x| x.set_status(RequestStatus::Dead));
        let mut ready_dead_requests:Vec<Request> = ready_rt_requests.iter()
            .filter(|x| x.deadline_time <= hdd.time)
            .cloned()
            .collect();
        dead_requests.append(&mut ready_dead_requests);
        dead_requests.append(&mut dead_requests_to_add);
        rt_requests.retain(|x| x.deadline_time > hdd.time);
        ready_rt_requests.retain(|x: &Request| x.deadline_time > hdd.time);


        // add ready rt requests and remove them from rt requests
        let mut ready_rt_requests_to_add:Vec<Request> = rt_requests.iter()
            .filter(|x| x.arrival_time <= hdd.time)
            .cloned()
            .collect();
        ready_rt_requests_to_add.iter_mut()
            .for_each(|x| x.set_status(RequestStatus::RealTimeReady));
        ready_rt_requests.append(&mut ready_rt_requests_to_add);
        rt_requests.retain(|x| x.arrival_time > hdd.time);

        // if there are no ready requests, go to next time unit
        if ready_requests.is_empty() && ready_rt_requests.is_empty() {
            hdd.time += 1;
            continue;
        }

        // schedule realtime request
        let rt_request = schedulers::get_next_request(rt_scheduler, &mut ready_rt_requests, &hdd);

        // go to request if there is one
        if let Some(mut rt_request) = rt_request {
            // delete rt request from ready_rt_requests
            ready_rt_requests.retain(|x| x != &rt_request);
            if rt_scheduler == Schedulers::FeasibleDeadlineScan {
                while hdd.current_track != rt_request.track_number {
                    // increment waiting time of all ready requests
                    ready_requests.iter_mut()
                        .for_each(|x| x.waiting_time += 1);
                    step_to_request(rt_scheduler, &mut hdd, &rt_request);
                    show_progress_with_rt(rt_scheduler, scheduler,done_requests.len(), requests.len(), &hdd);
                    let mut indexes_to_remove = Vec::new();
                    for i in 0..ready_requests.len() {
                        if ready_requests[i].track_number == hdd.current_track {
                            ready_requests[i].set_status(RequestStatus::Done);
                            done_requests.push(ready_requests[i].clone());
                            indexes_to_remove.push(i);
                        }
                    }
                    for i in 0..indexes_to_remove.len() {
                        ready_requests.remove(indexes_to_remove[i] - i);
                    }
                }
            } else {
                while hdd.current_track != rt_request.track_number {
                    // increment waiting time of all ready requests
                    ready_requests.iter_mut()
                        .for_each(|x| x.waiting_time += 1);
                    step_to_request(rt_scheduler, &mut hdd, &rt_request);
                    show_progress_with_rt(rt_scheduler, scheduler, done_requests.len(), requests.len(), &hdd);
                }
            }
            if rt_request.deadline_time >= hdd.time {
                rt_request.set_status(RequestStatus::Done);
                done_requests.push(rt_request);
            } else {
                rt_request.set_status(RequestStatus::Dead);
                dead_requests.push(rt_request);
            }
            continue;
        }

        // schedule request
        let request = schedulers::get_next_request(scheduler, &mut ready_requests, &hdd);

        // go to request if there is one
        if let Some(mut request) = request {
            // delete request from ready_requests
            ready_requests.retain(|x| x != &request);

            while hdd.current_track != request.track_number {
                // increment waiting time of all ready requests
                ready_requests.iter_mut()
                    .for_each(|x| x.waiting_time += 1);
                step_to_request(scheduler, &mut hdd, &request);
                show_progress_with_rt(rt_scheduler, scheduler, done_requests.len(), requests.len(), &hdd);
            }
            request.set_status(RequestStatus::Done);
            done_requests.push(request);
        }
        // idle
        else {
            // increment waiting time of all ready requests
            ready_requests.iter_mut()
                .for_each(|x| x.waiting_time += 1);
            if scheduler == Schedulers::Scan || scheduler == Schedulers::CScan {
                hdd.move_head(0, scheduler);
            }
            hdd.time += 1;
        }
    }
    show_progress_with_rt(rt_scheduler, scheduler, done_requests.len(), requests.len(), &hdd);
    // print average waiting time with 2 decimal places
    println!(" \nAverage waiting time of done requests: {:.0}", average_waiting_time(&done_requests));
    let underscores = "_".repeat(150);
    println!("{}", underscores);
}

#[allow(dead_code)]
fn simulate(scheduler: Schedulers) {
    // Statistics
    let filename = format!("csv/{:?}.csv", scheduler);
    let writer = csv::Writer::from_writer(File::create(&filename).unwrap());

    // Storage device
    let mut hdd = HardDriveDisk::new(NUMBER_OF_TRACKS, writer);

    // Requests
    let request_generator = request::RequestGenerator::new(hdd.number_of_tracks, NUMBER_OF_REQUESTS, MAX_ARRIVAL_TIME, DEADLINE_TIME, RT_CHANCE);
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
            // delete request from ready_requests
            ready_requests.retain(|x| x != &request);
            while hdd.current_track != request.track_number {
                // increment waiting time of all ready requests
                ready_requests.iter_mut()
                    .for_each(|x| x.waiting_time += 1);
                step_to_request(scheduler, &mut hdd, &request);
                show_progress(scheduler, done_requests.len(), requests.len(), &hdd);
            }
            request.set_status(RequestStatus::Done);
            done_requests.push(request);
        }
        // idle
        else {
            // increment waiting time of all ready requests
            ready_requests.iter_mut()
                .for_each(|x| x.waiting_time += 1);
            if scheduler == Schedulers::Scan || scheduler == Schedulers::CScan {
                hdd.move_head(0, scheduler);
            }
            hdd.time += 1;
        }
    }
    show_progress(scheduler, done_requests.len(), requests.len(), &hdd);
    println!(" \nAverage waiting time of done requests: {:.0}", average_waiting_time(&done_requests));
    let underscores = "_".repeat(100);
    println!("{}", underscores);
}
fn step_to_request(scheduler: Schedulers, hdd: &mut HardDriveDisk, request: &Request) {
    hdd.move_head(request.track_number, scheduler);
    hdd.time += 1;
}
fn show_progress(scheduler: Schedulers, done_requests: usize, total_requests: usize, hdd: &HardDriveDisk) {
    let scheduler_str = format!("{:?}", scheduler); // convert enum value to string
    print!("\r{:45} | time: {:<8} Progress: {}/{}, current_track: {}, seek_count: {} ", scheduler_str, hdd.time, done_requests, total_requests, hdd.current_track, hdd.seek_count);
}
fn show_progress_with_rt(rt_scheduler: Schedulers, scheduler: Schedulers, done_requests: usize, total_requests: usize, hdd: &HardDriveDisk) {
    let scheduler_str = format!("{:?}-{:?}", scheduler, rt_scheduler); // convert enum value to string
    print!("\r{:45} | time: {:<8} Progress: {}/{}, current_track: {}, seek_count: {} ", scheduler_str, hdd.time, done_requests, total_requests, hdd.current_track, hdd.seek_count);
}
fn average_waiting_time(done_requests: &Vec<Request>) -> f64 {
    let mut sum = 0;
    for request in done_requests {
        sum += request.waiting_time;
    }
    sum as f64 / done_requests.len() as f64
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
