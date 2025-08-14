pub struct FanService {
    pub fans: Vec<Fan>,
    pub pwms: Vec<Pwm>,
}

impl FanService {
    pub fn new(fans: Vec<Fan>, pwms: Vec<Pwm>) -> Self {
        Self {fans, pwms }
    }
}
