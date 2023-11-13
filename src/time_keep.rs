use instant::Instant;

pub struct TimeKeep{
    last_tick: Instant
}

impl TimeKeep{
    pub fn new() -> Self{
        Self {
            last_tick: Instant::now()
         }
    }

    pub fn update_and_get_dt(&mut self) -> f32{
        let new_tick = Instant::now();
        let dt = new_tick - self.last_tick;
        self.last_tick = new_tick;
        dt.as_secs_f32()
    }
}