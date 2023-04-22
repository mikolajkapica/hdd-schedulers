mod schedulers;

pub enum Schedulers {
    FCFS,
    SSTF,
    SCAN,
    CSCAN,
    EDF,
    FDSCAN,
}

pub fn get_next_request(scheduling_algorithm: Schedulers, requests: &Vec<Request>, hdd: &HDD) -> Option<Request> {
    match scheduling_algorithm {
        Schedulers::FCFS => fcfs(requests),
        Schedulers::SSTF => sstf(requests, hdd),
        Schedulers::SCAN => scan(requests, hdd),
        Schedulers::CSCAN => cscan(requests, hdd),
        Schedulers::EDF => edf(requests),
        Schedulers::FDSCAN => fdscan(requests, hdd),
    }
}

fn fcfs(requests: &Vec<Request>) -> Option<Request> {
    todo!("Implement FCFS");
}

fn sstf(requests: &Vec<Request>, hdd: &HDD) -> Option<Request> {
    todo!("Implement SSTF");
}

fn scan(requests: &Vec<Request>, hdd: &HDD) -> Option<Request> {
    todo!("Implement SCAN");
}

fn cscan(requests: &Vec<Request>, hdd: &HDD) -> Option<Request> {
    todo!("Implement CSCAN");
}

fn edf(requests: &Vec<Request>) -> Option<Request> {
    todo!("Implement EDF");
}

fn fdscan(requests: &Vec<Request>, hdd: &HDD) -> Option<Request> {
    todo!("Implement FDSCAN");
}
