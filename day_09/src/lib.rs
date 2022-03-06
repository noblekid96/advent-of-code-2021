// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::cmp;

pub fn day9helper(height_map: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, low_points: &mut Vec<(usize,usize)>, x: usize, y: usize){

    visited[y][x] = true;

    let current_height = height_map[y][x];

    let mut is_low_point = true;

    if x > 0 {
        let is_lower = current_height < height_map[y][x-1];
        is_low_point = is_low_point && is_lower;
        if is_lower{
            visited[y][x-1] = true;
        }
    }
    if x < visited[0].len()-1{
        let is_lower = current_height < height_map[y][x+1];
        is_low_point = is_low_point && is_lower;
        if is_lower{
            visited[y][x+1] = true;
        }
    }
    if y > 0 {
        let is_lower = current_height < height_map[y-1][x];
        is_low_point = is_low_point && is_lower;
        if is_lower{
            visited[y-1][x] = true;
        }
    }
    if y < visited.len()-1{
        let is_lower = current_height < height_map[y+1][x];
        is_low_point = is_low_point && is_lower;
        if is_lower{
            visited[y+1][x] = true;
        }
    }

    if is_low_point == true {
        low_points.push((y, x));
    }

}

pub fn day9partb(height_map: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, basin_size: &mut i32, x: usize, y: usize){
    if height_map[y][x] == 9{
        return;
    }
    if visited[y][x] == true{
        return;
    }
    visited[y][x] = true;
    *basin_size += 1;

    if x > 0 {
        day9partb(height_map, visited, basin_size, x-1, y);
    }
    if x < visited[0].len()-1{
        day9partb(height_map, visited, basin_size, x+1, y);
    }
    if y > 0 {
        day9partb(height_map, visited, basin_size, x, y-1);
    }
    if y < visited.len()-1{
        day9partb(height_map, visited, basin_size, x, y+1);
    }
}

pub fn day9(input: &str, partb: bool) -> i32{
    let lines : Vec<&str> = input.split_terminator('\n').collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut low_points : Vec<(usize,usize)> = vec![];
    let mut visited = vec![vec![false; width]; height];

    let mut height_map: Vec<Vec<i32>> = vec![];

    for line in lines {
        let mut height_row = vec![];
        for chr in line.chars(){
            height_row.push(chr.to_digit(10).unwrap() as i32);
        }
        height_map.push(height_row);
    }

    for i in 0..height{
        for j in 0..width{
            if visited[i][j] == false {
                day9helper(&height_map, &mut visited, &mut low_points, j, i);
            }
        }
    }

    if partb == false {
        let mut sum : i32 = 0;
        for point in low_points{
            let height = height_map[point.0][point.1];
            sum += height + 1;
        }
        return sum;
    }
    visited = vec![vec![false; width]; height];
    let mut basin_sizes = vec![];
    for point in low_points {
        let mut basin_size = 0;
        day9partb(&height_map, &mut visited, &mut basin_size, point.1, point.0);
        println!("Size of basin for low point: ({},{}) is {}", point.0, point.1, basin_size);
        basin_sizes.push(basin_size);
    }


    basin_sizes.sort();
    println!("Basin sizes size: {:?}", basin_sizes);

    return basin_sizes[basin_sizes.len()-1] * basin_sizes[basin_sizes.len()-2]  * basin_sizes[basin_sizes.len()-3]
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_a_example() {
        println!("Running part 1 on example: 15");
        assert_eq!(super::day9(include_str!("example.txt"), false),15);
    }

    // #[test]
    // fn part_a_example_2() {
    //     println!("Running part 1 on example: 5934");
    //     assert_eq!(super::day7(include_str!("example2.txt"), false),5934);
    // }


    #[test]
    fn part_a() {
        let result = super::day9(include_str!("input.txt"), false);
        println!("Running part 1 on input: {result}");
    }

    #[test]
    fn part_b_example() {
        println!("Running part 2 on example: 1134");
        let result = super::day9(include_str!("example.txt"), true);
        assert_eq!(result,1134);
    }

    // #[test]
    // fn part_b_example2() {
    //     println!("Running part 2 on example2: 61229");
    //     let result = super::day9(include_str!("example2.txt"), true);
    //     assert_eq!(result,61229);
    // }


    #[test]
    fn part_b() {
        let result = super::day9(include_str!("input.txt"), true);
        println!("Running part 2 on input: {result}");
    }

}
