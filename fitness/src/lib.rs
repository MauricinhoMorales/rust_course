mod cardio;
mod weightlifting;

mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}")
    }
}

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExcercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExcercise,
}

impl GymWorkout {
    pub fn new() -> GymWorkout {
        cardio::ask_about_program();
        weightlifting::ask_about_program();
        diet::ask_about_program();
        GymWorkout {
            cardio: CardioExercise::new(String::from("Monday"), 10, CardioTool::Bike),
            weightlifting: WeightliftingExcercise::new(String::from("Squats"), 20),
        }
    }
}
