// -------------------------------------------
// 	Enums
// -------------------------------------------

/*
// Example 1:

enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sundary,
}
fn main() {
    let mut day = "Saturday".to_string();

    let week_day = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sundary".to_string(),
    ];
    day = week_day[1].clone();

    let day = WeekDay::Saturday;
}
*/

// Example 2:
enum TravelType {
    Car(f32),
    Train(f32),
    Aeroplane(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Aeroplane(miles) => miles * 5.0,
        };
        allowance
    }
}
fn main() {
    let participant = TravelType::Car(60.0);
    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance()
    );
}
