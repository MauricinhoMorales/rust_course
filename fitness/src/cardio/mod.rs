const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_program() {
    println!("The cardio trainer is {PERSONAL_TRAINER}")
}

#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

#[derive(Debug)]
pub struct Exercise {
    day: String,
    minutes: u32,
    tool: CardioTool,
}

impl Exercise {
    pub fn new(day: String, minutes: u32, tool: CardioTool) -> Exercise {
        Exercise { day, minutes, tool }
    }
}
