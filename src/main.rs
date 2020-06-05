use std::env;

struct Mood<'a>{
    activity: &'a str,
    rating: &'a str,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let mood: Mood = Mood { activity: &args[1], rating: &args[2]};
    //let activity = &args[1];
    //let rating = &args[2];

    println!("You are doing {}", mood.activity);
    println!("And feel {}", mood.rating);
}
