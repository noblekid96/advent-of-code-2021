// use std::collections::HashMap;
// use std::collections::HashSet;
use std::cmp;


pub fn griddle(input: &str, check_diagonals: bool) -> i32{
    let mut grid = vec![vec![0; 1000]; 1000];
    let mut lines : Vec<(Vec<i32>, Vec<i32>)> = vec![];
    let mut sum = 0;

    for input_line in input.split_terminator('\n'){
        // println!("{input_line}");
        let start_end : Vec<&str> = input_line.split(' ').collect();
        let start : &str = start_end[0];
        let end : &str = start_end[2];
        let start_coord = start.split(',').map(|numstring| numstring.to_string().parse().unwrap()).collect();
        let end_coord = end.split(',').map(|numstring| numstring.to_string().parse().unwrap()).collect();
        lines.push((start_coord,end_coord));
    }
    for it in lines.iter(){
        // println!("{:#?}", it);
        if it.0[0] == it.1[0] || it.0[1] == it.1[1] {
            // println!("Test");
            let (x1, x2, y1, y2) = (it.0[0], it.1[0], it.0[1], it.1[1]);
            // println!("{}{} {}{}", x1,x2,y1,y2);
            let lower_x = cmp::min(x1,x2);
            let upper_x = cmp::max(x1,x2);

            let lower_y = cmp::min(y1,y2);
            let upper_y = cmp::max(y1,y2);

            for x in lower_x..upper_x+1{
                for y in lower_y..upper_y+1{
                    // println!("{} {}",x,y);
                    grid[y as usize][x as usize] += 1;
                }
            }
        }

        if check_diagonals && (it.0[0] - it.1[0]).abs() == (it.0[1] - it.1[1]).abs(){
            let ( mut x1, x2, mut y1, y2) = (it.0[0], it.1[0], it.0[1], it.1[1]);
            let mut dx = 1;
            let mut dy = 1;

            if x2 < x1 {
                dx = -1;
            }

            if y2 < y1 {
                dy = -1;
            }

            while x1 != x2+dx && y1 != y2+dy {
                // println!("x1 {} y1 {}", x1,y1);
                grid[y1 as usize][x1 as usize] += 1;
                x1 += dx;
                y1 += dy;
            }
        }

    };


    for i in 0..1000{
        // println!("{:?}", grid[i]);
        for j in 0..1000{
            if grid[i][j] >= 2{
                sum += 1;
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 5");
        assert_eq!(super::griddle(include_str!("example.txt"), false),5);
    }

    #[test]
    fn part_a() {
        let result = super::griddle(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 12");
        let result = super::griddle(include_str!("example.txt"), true);
        assert_eq!(result,12);
    }

    #[test]
    fn part_b() {
        let result = super::griddle(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
    }

}
