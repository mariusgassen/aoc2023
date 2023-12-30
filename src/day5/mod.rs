use std::{fmt, thread};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::Range;
use std::str::FromStr;
use std::string::ParseError;

use crate::aoc::Day;
use crate::aoc::tools::read_blocks;

pub struct Day5 {
    input: String,
}

impl Day5 {
    pub fn new(input: String) -> Day5 {
        Day5 {
            input
        }
    }
}

impl Day for Day5 {
    fn part1(&self) -> String {
        let almanac = Almanac::from_str(&self.input).unwrap();

        almanac.seeds.iter()
            .map(|seed| almanac.seed_to_location(seed))
            .min().expect("No seeds found")
            .to_string()
    }

    /// Very slow, longest worker needs about 30 minutes
    fn part2(&self) -> String {
        let almanac = Almanac::from_str(&self.input).unwrap();
        let ranges = almanac.seed_range_map.clone().get_source_ranges();

        thread::scope(|scope| {
            let handles: Vec<_> = ranges.iter()
                .map(|r|
                    scope.spawn(|| {
                        let timer = std::time::Instant::now();
                        println!("Starting thread");
                        let res = r.clone()
                            .map(|s| almanac.seed_to_location(&Seed(s)))
                            .min();
                        println!("Thread finished after {:?}", timer.elapsed());
                        res
                    })
                )
                .collect();

            handles.into_iter()
                .map(|handle| handle.join().unwrap())
                .min()
                .unwrap().expect("No seeds found")
                .to_string()
        })
    }
}

trait Id
    where Self: Sized {
    fn raw(&self) -> i64;

    fn new(id: i64) -> Self;

    fn add(&self, v: i64) -> Self {
        Self::new(self.raw() + v)
    }

    fn sub(&self, v: i64) -> Self {
        Self::new(self.raw() - v)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Seed(i64);

impl Id for Seed {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Seed(id)
    }
}

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Soil(i64);

impl Display for Soil {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Id for Soil {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Soil(id)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Fertilizer(i64);

impl Display for Fertilizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Id for Fertilizer {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Fertilizer(id)
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Water(i64);

impl Id for Water {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Water(id)
    }
}

impl Display for Water {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Light(i64);

impl Id for Light {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Light(id)
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Temperature(i64);

impl Id for Temperature {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Temperature(id)
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Humidity(i64);

impl Id for Humidity {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Humidity(id)
    }
}

impl Display for Humidity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Location(i64);

impl Id for Location {
    fn raw(&self) -> i64 {
        self.0
    }

    fn new(id: i64) -> Self {
        Location(id)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct RangeMap<S, D> {
    map: Vec<(S, D, i64)>,
}

impl<S, D> Clone for RangeMap<S, D>
    where S: Clone, D: Clone {
    fn clone(&self) -> Self {
        RangeMap {
            map: self.map.clone()
        }
    }
}

impl<S, D> RangeMap<S, D>
    where S: Eq + Hash + PartialOrd + Id, D: Id {
    fn new(block: &Vec<String>, fs: fn(i64) -> S, fd: fn(i64) -> D) -> RangeMap<S, D>
        where S: Eq + Hash {
        let mut map = Vec::new();
        for line in block {
            let mut iter = line.split_whitespace();
            let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
            let source_start = iter.next().unwrap().parse::<i64>().unwrap();
            let range = iter.next().unwrap().parse::<i64>().unwrap();
            let tuple = (fs(source_start), fd(dest_start), range);
            map.push(tuple);
        }

        RangeMap {
            map
        }
    }

    fn get(&self, source: &S) -> Option<D> {
        self.map.iter()
            // Find the correct range that contains the source
            .filter(|(s, _, r)| {
                let raw_s = s.raw();
                (raw_s..raw_s + r).contains(&source.raw())
            })
            // Calculate the destination
            .map(|(s, d, _)| {
                d.sub(s.raw()).add(source.raw())
            }).next()
    }
    fn get_source_ranges(&self) -> Vec<Range<i64>> {
        self.map.iter()
            .map(|(s, _, r)| {
                let raw_s = s.raw();
                raw_s..raw_s + r
            }).collect()
    }
}

struct Almanac {
    seeds: Vec<Seed>,
    seed_range_map: RangeMap<Seed, Seed>,
    seed_to_soil_map: RangeMap<Seed, Soil>,
    soil_to_fertilizer_map: RangeMap<Soil, Fertilizer>,
    fertilizer_to_water_map: RangeMap<Fertilizer, Water>,
    water_to_light_map: RangeMap<Water, Light>,
    light_to_temperature_map: RangeMap<Light, Temperature>,
    temperature_to_humidity_map: RangeMap<Temperature, Humidity>,
    humidity_to_location_map: RangeMap<Humidity, Location>,
}

impl Almanac {
    fn seed_to_soil(&self, seed: &Seed) -> Soil {
        self.seed_to_soil_map.get(&seed).unwrap_or(Soil(seed.0)).clone()
    }

    fn soil_to_fertilizer(&self, soil: &Soil) -> Fertilizer {
        self.soil_to_fertilizer_map.get(&soil).unwrap_or(Fertilizer(soil.0)).clone()
    }

    fn fertilizer_to_water(&self, fertilizer: &Fertilizer) -> Water {
        self.fertilizer_to_water_map.get(&fertilizer).unwrap_or(Water(fertilizer.0)).clone()
    }

    fn water_to_light(&self, water: &Water) -> Light {
        self.water_to_light_map.get(&water).unwrap_or(Light(water.0)).clone()
    }

    fn light_to_temperature(&self, light: &Light) -> Temperature {
        self.light_to_temperature_map.get(&light).unwrap_or(Temperature(light.0)).clone()
    }

    fn temperature_to_humidity(&self, temperature: &Temperature) -> Humidity {
        self.temperature_to_humidity_map.get(&temperature).unwrap_or(Humidity(temperature.0)).clone()
    }

    fn humidity_to_location(&self, humidity: &Humidity) -> Location {
        self.humidity_to_location_map.get(&humidity).unwrap_or(Location(humidity.0)).clone()
    }

    fn seed_to_location(&self, seed: &Seed) -> Location {
        let soil = self.seed_to_soil(seed);
        let fertilizer = self.soil_to_fertilizer(&soil);
        let water = self.fertilizer_to_water(&fertilizer);
        let light = self.water_to_light(&water);
        let temperature = self.light_to_temperature(&light);
        let humidity = self.temperature_to_humidity(&temperature);
        self.humidity_to_location(&humidity)
    }
}

impl FromStr for Almanac {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = read_blocks(s, Some("map"));
        let seeds = blocks[0][0].strip_prefix("seeds: ").unwrap()
            .split(" ")
            .map(|s| Seed(s.parse::<i64>().unwrap()))
            .collect::<Vec<Seed>>();

        // Make a vector of pairs from the vector of seeds
        //
        let seed_pairs = seeds.iter().step_by(2)
            .zip(seeds.iter().skip(1).step_by(2))
            .map(|(s1, s2)| (s1.clone(), s1.clone(), s2.raw()))
            .collect();

        let seed_range_map = RangeMap {
            map: seed_pairs,
        };

        let seed_to_soil_map = RangeMap::new(&blocks[1], Seed, Soil);
        let soil_to_fertilizer_map = RangeMap::new(&blocks[2], Soil, Fertilizer);
        let fertilizer_to_water_map = RangeMap::new(&blocks[3], Fertilizer, Water);
        let water_to_light_map = RangeMap::new(&blocks[4], Water, Light);
        let light_to_temperature_map = RangeMap::new(&blocks[5], Light, Temperature);
        let temperature_to_humidity_map = RangeMap::new(&blocks[6], Temperature, Humidity);
        let humidity_to_location_map = RangeMap::new(&blocks[7], Humidity, Location);

        Ok(Almanac {
            seeds,
            seed_range_map,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Day;

    const INPUT: &str = r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4    "#;

    fn day() -> super::Day5 {
        super::Day5::new(INPUT.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(day().run().0, "35");
    }

    #[test]
    fn test_part2() {
        assert_eq!(day().run().1, "46");
    }
}
