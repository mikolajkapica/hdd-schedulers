#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub id: u32,
    pub status: RequestStatus,
    pub track_number: u32,
    pub arrival_time: u32,
    pub waiting_time: u32,
    pub deadline_time: u32,
}

impl Request {
    fn new(id: u32, status: RequestStatus, track_number: u32, arrival_time: u32, deadline_time: u32) -> Request {
        Request {
            id,
            status,
            track_number,
            arrival_time,
            waiting_time: 0,
            deadline_time,
        }
    }
    pub fn set_status(&mut self, status: RequestStatus) {
        self.status = status;
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum RequestStatus {
    Normal,
    Ready,
    RealTime,
    RealTimeReady,
    Done,
    Dead,
}

pub struct RequestGenerator {
    number_of_tracks: u32,
    number_of_requests: u32,
    max_arrival_time: u32,
    deadline_time: u32,
    rt_chance: f32,
}

impl RequestGenerator {
    pub fn new(
        number_of_tracks: u32,
        number_of_requests: u32,
        max_arrival_time: u32,
        deadline_time: u32,
        rt_chance: f32,
    ) -> RequestGenerator {
        RequestGenerator {
            number_of_tracks,
            number_of_requests,
            max_arrival_time,
            deadline_time,
            rt_chance,
        }
    }

    pub fn generate_requests(&self) -> Vec<Request> {
        use rand::Rng;
        use rand::SeedableRng;

        let mut requests = Vec::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(412123420);
        let number_of_requests = self.number_of_requests;
        let deadline_time = self.deadline_time;

        for i in 0..number_of_requests {
            let track_number = rng.gen_range(0..self.number_of_tracks);
            let arrival_time = rng.gen_range(0..self.max_arrival_time);
            let real_time = rng.gen_range(0..100) < (self.rt_chance * 100 as f32) as u32;
            let status = if real_time {
                RequestStatus::RealTime
            } else {
                RequestStatus::Normal
            };
            requests.push(Request::new(
                i,
                status,
                track_number,
                arrival_time,
                arrival_time + deadline_time,
            ));
        }

        requests
    }
}
