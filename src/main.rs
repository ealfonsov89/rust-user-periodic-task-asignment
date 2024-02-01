use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

const WEEK_IN_SECONDS: u64 = 604800;
fn main() {
    
    let (users, week_interval, week_offset) = get_program_arg();

    let user = get_next_user(week_offset, week_interval, users); 
    println!("{:?}", user);
}

fn get_program_arg() -> (Vec<String>, u64, u64) {
    let args: Vec<String> = env::args().collect();

    let mut users = vec![String::from("Armando"), String::from("Tomás"), String::from("Ernesto"), String::from("Alexis"), String::from("JuanMa")];
    let mut week_interval = 2u64;
    let mut week_offset = 1u64;

    for i in 1..args.len() {
        if args[i] == "--week-interval" && i + 1 < args.len() {
            week_interval = args[i + 1].parse().unwrap();
        } else if args[i] == "--week-offset" && i + 1 < args.len() {
            week_offset = args[i + 1].parse().unwrap();
        } else if args[i] == "--users" && i + 1 < args.len() {
            users = args[i + 1].split(",").map(|name| String::from(name)).collect();
        }
    }

    return (users, week_interval, week_offset);
}

fn get_next_user(week_offset: u64, week_interval: u64, users: Vec<String>) -> String {
    let total_weeks = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / WEEK_IN_SECONDS) + week_offset;
    let weeks_intervals: u32 = (total_weeks / week_interval).try_into().unwrap();
    let user_length: u32 = users.len().try_into().unwrap();
    let user_index: usize = (weeks_intervals % user_length - 1).try_into().unwrap();
    return users[user_index].clone();
   
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn get_user_0() {
        let users = vec![String::from("Armando"), String::from("Tomás"), String::from("Ernesto"), String::from("Alexis"), String::from("JuanMa")];
        let week_interval = 2u64;
        let week_offset = 1u64;
        assert_eq!(get_next_user(week_offset, week_interval, users), "Armando");
    }

    #[test]
    fn get_user_3() {
        let users = vec![String::from("Armando"), String::from("Tomás"), String::from("Ernesto"), String::from("Alexis"), String::from("JuanMa")];
        let week_interval = 2u64;
        let week_offset = 4u64;
        assert_eq!(get_next_user(week_offset, week_interval, users), "Ernesto");
    }
}