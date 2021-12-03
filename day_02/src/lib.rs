pub fn part_a(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                horizontal += amount;
            }
            "down" => {
                depth += amount;
            }
            "up" => {
                depth -= amount;
            }
            _ => panic!(),
        }
    }
    horizontal*depth
}

pub fn part_b(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => panic!(),
        }
    }
    horizontal*depth
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
        println!("Running part 1 on example: 150");
        assert_eq!(super::part_a(include_str!("example.txt")),150);
    }

    #[test]
    fn part_a() {
        let result = super::part_a(include_str!("input.txt"));
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 900");
        assert_eq!(super::part_b(include_str!("example.txt")),900);
    }

    #[test]
    fn part_b() {
        let result = super::part_b(include_str!("input.txt"));
        println!("Running part 2 on input: {result}");
    }

}
