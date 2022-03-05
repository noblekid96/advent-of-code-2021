// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::cmp;
use std::collections::VecDeque;


pub fn day6(input: &str, num_days: i32) -> i128{
    println!("{}",input);
    let mut dp : [i128; 512] = [0; 512];
    for timer_str in input.split(','){
        let timer_day : usize = timer_str.trim().to_string().parse().unwrap();
        dp[timer_day] += 1;
    }

    println!("{:?}",dp);

    let mut temp_queue: VecDeque<i32> = VecDeque::new();
    for day in 1..num_days+1 {
        let day : usize = day as usize;
        dp[day+6] += dp[day-1];
        dp[day+8] += dp[day-1];
        // println!{"{:?}",dp}
    }

    let mut num_lanterfishes: i128 = 0;
    for i in num_days..num_days+9{
        num_lanterfishes += dp[i as usize];
    }


    return num_lanterfishes as i128;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 26");
        assert_eq!(super::day6(include_str!("example.txt"), 18),26);
    }

    #[test]
    fn part_a_example_2() {
        println!("Running part 1 on example: 5934");
        assert_eq!(super::day6(include_str!("example.txt"), 80),5934);
    }


    #[test]
    fn part_a() {
        let result = super::day6(include_str!("input.txt"), 80);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 26984457539");
        let result = super::day6(include_str!("example.txt"), 256);
        assert_eq!(result,26984457539);
    }

    #[test]
    fn part_b() {
        let result = super::day6(include_str!("input.txt"), 256);
        println!("Running part 2 on input: {result}");
    }

}
