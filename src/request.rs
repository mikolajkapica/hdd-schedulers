struct Request {
    status: RequestStatus,
    track_number: i32,
    arrival_time: i32,
    deadline_time: i32,
}

impl Request {
    fn new(status: RequestStatus, track_number: i32, arrival_time: i32, deadline_time: i32) -> Request {
        Request {
            status,
            track_number,
            arrival_time,
            deadline_time,
        }
    }
}

enum RequestStatus {
    Normal,
    Ready,
    RealTime,
    RealTimeReady,
    Done,
}

pub struct RequestGenerator {
    number_of_tracks: i32,
    number_of_requests: i32,
    max_arrival_time: i32,
    deadline_time: i32,
}

impl RequestGenerator {
    pub fn new(
        number_of_tracks: i32,
        number_of_requests: i32,
        max_arrival_time: i32,
        deadline_time: i32,
    ) -> RequestGenerator {
        RequestGenerator {
            number_of_tracks,
            number_of_requests,
            max_arrival_time,
            deadline_time,
        }
    }

    pub fn generate_requests(&self) -> Vec<Request> {
        use rand::Rng;
        use rand::SeedableRng;

        let mut requests = Vec::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(420);
        let number_of_requests = self.number_of_requests;
        let deadline_time = self.deadline_time;

        for _ in 0..number_of_requests {
            let track_number = rng.gen_range(0..self.number_of_tracks);
            let arrival_time = rng.gen_range(0..self.max_arrival_time);
            let real_time = rng.gen_range(0..100) < 5;
            let status = if real_time {
                RequestStatus::RealTime
            } else {
                RequestStatus::Normal
            };
            requests.push(Request::new(
                status,
                track_number,
                arrival_time,
                arrival_time + deadline_time,
            ));
        }

        requests
    }
}
