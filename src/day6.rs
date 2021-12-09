use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub struct LemonfishSchoolMeta {
    lemonfish: [usize; 9],
}

impl LemonfishSchoolMeta {
    fn tick(&mut self) {
        let new_fish = self.lemonfish[0];
        for i in 0..=7 {
            self.lemonfish[i] = self.lemonfish[i + 1];
        }
        self.lemonfish[8] = new_fish;
        self.lemonfish[6] += new_fish;
    }

    fn count(&self) -> usize {
        self.lemonfish.into_iter().sum()
    }
}

#[derive(Debug, Clone)]
pub struct LemonfishSchool {
    lemonfish: Vec<Lemonfish>,
}

impl LemonfishSchool {
    fn tick(&mut self) {
        let mut new_fish = vec![];
        self.lemonfish.iter_mut().for_each(|fish| {
            if fish.spawn_counter == 0 {
                fish.spawn_counter = 6;
                new_fish.push(Lemonfish { spawn_counter: 8 })
            } else {
                fish.spawn_counter -= 1;
            }
        });
        self.lemonfish.append(&mut new_fish);
    }

    fn into_meta(&self) -> LemonfishSchoolMeta {
        let mut result = LemonfishSchoolMeta { lemonfish: [0; 9] };
        self.lemonfish
            .clone()
            .into_iter()
            .for_each(|fish| result.lemonfish[fish.spawn_counter] += 1);
        result
    }
}

impl FromStr for LemonfishSchool {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LemonfishSchool {
            lemonfish: s
                .trim()
                .split(",")
                .map(|fish| fish.parse::<Lemonfish>())
                .collect::<Result<Vec<Lemonfish>, ParseIntError>>()?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Lemonfish {
    spawn_counter: usize,
}

impl FromStr for Lemonfish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lemonfish {
            spawn_counter: s.parse()?,
        })
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> LemonfishSchool {
    input.parse().unwrap()
}

#[aoc(day6, part1, naive)]
fn part1(school: &LemonfishSchool) -> usize {
    let mut our_school = school.clone();
    for _ in 0..80 {
        our_school.tick();
    }

    our_school.lemonfish.len()
}

#[aoc(day6, part1, meta)]
fn part1_meta(school: &LemonfishSchool) -> usize {
    let mut our_school = school.into_meta();
    for _ in 0..80 {
        our_school.tick();
    }

    our_school.count()
}

#[aoc(day6, part2)]
fn part2(school: &LemonfishSchool) -> usize {
    let mut our_school = school.into_meta();
    for _ in 0..256 {
        our_school.tick();
    }

    our_school.count()
}
