pub fn part_a(input: &str) -> i64 {
    let depths: Vec<i64> = input.trim().split('\n').map( |l| l.parse::<i64>().unwrap()).collect();
    let mut increases = 0;
    for i in 1..depths.len(){
        if depths[i] > depths[i-1]{
            increases+=1;
        }
    }
    increases
}

pub fn part_b(input: &str) -> i64 {
    let depths: Vec<i64> = input.trim().split('\n').map( |l| l.parse::<i64>().unwrap()).collect();
    let mut increases = 0;
    for i in 0..depths.len()-3{
        if depths[i+3] > depths[i]{
            increases+=1;
        }
    }
    increases
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 7");
        assert_eq!(super::part_a(include_str!("example.txt")),7);
    }

    #[test]
    fn part_a() {
        let result = super::part_a(include_str!("input.txt"));
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 5");
        assert_eq!(super::part_b(include_str!("example.txt")),5);
    }

    #[test]
    fn part_b() {
        let result = super::part_b(include_str!("input.txt"));
        println!("Running part 2 on input: {result}");
    }

}
