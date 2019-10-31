use std::vec;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
struct Point {
    poz_x: i32,
    poz_y: i32
}

#[derive(Debug)]
struct Pair {
    first_point: Point,
    second_point: Point
}

fn main() {
    let mut points_list: Vec<Point> = vec![];

    let mut rdr = csv::Reader::from_path("points.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);

        let point = Point {
            poz_x: record.get(0).unwrap().parse().unwrap(),
            poz_y: record.get(1).unwrap().parse().unwrap()
        };
        println!("{:?}", point);
        points_list.push(point);
    }

    points_list.sort();
    println!("{:?}", points_list);

    // build lists with all the points per level
    let mut lines: HashMap<i32, Vec<Point>> = HashMap::new();
    for index in 0..points_list.len() {
        let point = points_list.index(index);
        if !lines.contains_key(&point.poz_x) {
            lines.insert(point.poz_x, vec![*point]);
        } else {
            let list = lines.get_mut(&point.poz_x).unwrap();
            list.push(*point);
        }
    }
    println!("{:?}", lines);

    // build all the pairs per level
    let mut pairs_level: HashMap<i32, Vec<Pair>> = HashMap::new();
    let mut levels: Vec<i32> = Vec::new();
    for line in lines {
        let level = line.0;
        let points = line.1;
        levels.push(level);
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let pair = Pair {
                    first_point: *points.get(i).unwrap(),
                    second_point: *points.get(j).unwrap()
                };

                if !pairs_level.contains_key(&level) {
                    pairs_level.insert(level, vec![pair]);
                } else {
                    let list = pairs_level.get_mut(&level).unwrap();
                    list.push(pair);
                }
            }
        }
    }
    println!("{:?}", pairs_level);
    println!("{:?}", levels);

    // count rectangles
    let mut count: i32 = 0;
    for i in 0..levels.len() {
        for j in (i + 1)..levels.len() {
            let first_line = pairs_level.get(levels.get(i).unwrap()).unwrap();
            let second_line = pairs_level.get(levels.get(j).unwrap()).unwrap();

            for pair_1 in first_line {
                for pair_2 in second_line {
                    if (pair_1.first_point.poz_y == pair_2.first_point.poz_y) && (pair_1.second_point.poz_y == pair_2.second_point.poz_y) {
                        count = count + 1;
                        println!("{:?} {:?}", pair_1, pair_2);
                    }
                }
            }
        }
    }

    println!("rectangles found: {}", count);
}
