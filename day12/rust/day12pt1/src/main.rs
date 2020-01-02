// Advent of Code Day 12 part 1

// I wasn't able to implement this one in the way I wanted to. Since I had a
// vector of moons, passing two elements of the vector into a function as
// mutable elements was making the borrow checker unhappy.
// Still, this solution runs very quickly anyway and the code is fairly
// straightforward. This was the first time I used the Regex crate, and its
// code comes from its documentation.

use regex::Regex;
use std::{env, fs, process};

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    pos_x: i32,
    pos_y: i32,
    pos_z: i32,
    v_x: i32,
    v_y: i32,
    v_z: i32,
}

impl Moon {
    fn apply_pull_from_other_moon(
        &mut self,
        other_moon_x: i32,
        other_moon_y: i32,
        other_moon_z: i32,
    ) {
        if self.pos_x < other_moon_x {
            self.v_x += 1;
        } else if self.pos_x > other_moon_x {
            self.v_x -= 1;
        }

        if self.pos_y < other_moon_y {
            self.v_y += 1;
        } else if self.pos_y > other_moon_y {
            self.v_y -= 1;
        }

        if self.pos_z < other_moon_z {
            self.v_z += 1;
        } else if self.pos_z > other_moon_z {
            self.v_z -= 1;
        }
    }

    fn update_velocity(&mut self) {
        self.pos_x += self.v_x;
        self.pos_y += self.v_y;
        self.pos_z += self.v_z;
    }

    fn total_energy(&self) -> i32 {
        (self.pos_x.abs() + self.pos_y.abs() + self.pos_z.abs())
            * (self.v_x.abs() + self.v_y.abs() + self.v_z.abs())
    }
}

fn process_arguments(args: &[String]) -> Result<Vec<Moon>, &str> {
    if args.len() != 2 {
        return Err("Please specify a text file for the moon data.");
    }
    let moon_format =
        Regex::new(r"<[xyz]=(-?[0-9]+),\s[xyz]=(-?[0-9]+),\s[xyz]=(-?[0-9]+)>").unwrap();
    let seed_file = fs::read_to_string(&args[1]).expect("The file path was not valid.");

    let mut moons: Vec<Moon> = Vec::new();

    for cap in moon_format.captures_iter(&seed_file) {
        let new_x = &cap[1]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");
        let new_y = &cap[2]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");
        let new_z = &cap[3]
            .parse::<i32>()
            .expect("A moon had an invalid starting value in its position.");

        moons.push(Moon {
            pos_x: *new_x,
            pos_y: *new_y,
            pos_z: *new_z,
            v_x: 0,
            v_y: 0,
            v_z: 0,
        });
    }
    Ok(moons)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut moons: Vec<Moon> = process_arguments(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    for _ in 0..1000 {

        // Calculating based on the array index isn't a great idea, but doing this
        // sort of double-traversal was turning into quite a mess when it came to
        // vector mutability and the borrow checker. I hope as I learn more about
        // Rust I'll be able to structure this in a better way!
        for current_moon in 0..=3 {
            for other_moon in 0..=3 {
                let other_moon_x = moons[other_moon].pos_x;
                let other_moon_y = moons[other_moon].pos_y;
                let other_moon_z = moons[other_moon].pos_z;

                // Technically, this check isn't necessary, but let's save some
                // cycles!
                if !(current_moon == other_moon) {
                    moons[current_moon].apply_pull_from_other_moon(
                        other_moon_x,
                        other_moon_y,
                        other_moon_z,
                    );
                }
            }
        }

        for moon in &mut moons {
            moon.update_velocity();
        }
    }
    let mut total_energy = 0;
    for moon in &moons {
        total_energy += moon.total_energy();
    }

    println!("The total energy in the system is: {}", total_energy);
}
