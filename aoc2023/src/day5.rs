use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
struct ConvertEntry {
    dest: u64,
    src: u64,
    len: u64,
}

#[derive(Debug)]
struct ConvertMap {
    seed_to_soil: Vec<ConvertEntry>,
    soil_to_fertilizer: Vec<ConvertEntry>,
    fertilizer_to_water: Vec<ConvertEntry>,
    water_to_light: Vec<ConvertEntry>,
    light_to_temperature: Vec<ConvertEntry>,
    temperature_to_humidity: Vec<ConvertEntry>,
    humidity_to_location: Vec<ConvertEntry>,
}

fn parse_to_convert_map(lines: &[String]) -> Vec<ConvertEntry> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let dest = parts.next().unwrap().parse::<u64>().unwrap();
            let src = parts.next().unwrap().parse::<u64>().unwrap();
            let len = parts.next().unwrap().parse::<u64>().unwrap();
            ConvertEntry { dest, src, len }
        })
        .collect::<Vec<_>>()
}

fn find_map(convert_map: &[ConvertEntry], src: u64) -> u64 {
    convert_map
        .iter()
        .find(|entry| entry.src <= src && src < entry.src + entry.len)
        .map_or(src, |entry| entry.dest + src - entry.src)
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    len: u64,
}

fn fill_convert_map(convert_map: Vec<ConvertEntry>) -> Vec<ConvertEntry> {
    let mut filled = vec![];
    let mut src = 0;
    for entry in convert_map {
        if src < entry.src {
            filled.push(ConvertEntry {
                dest: src,
                src,
                len: entry.src - src,
            });
        }
        filled.push(entry);
        src = entry.src + entry.len;
    }
    filled.push(ConvertEntry {
        dest: src,
        src,
        len: u64::MAX - src,
    });
    filled
}

fn find_map_for_range(convert_map: &[ConvertEntry], src: Range) -> Vec<Range> {
    let start_entry = convert_map
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.src <= src.start && src.start < entry.src + entry.len)
        .unwrap();

    let mut out_ranges = vec![];
    let range_start = src.start;
    let range_end = src.start + src.len;

    for entry in &convert_map[start_entry.0..] {
        let entry_start = entry.src;
        let entry_end = entry.src + entry.len;
        let out_range_start = range_start.max(entry_start);
        let out_range_end = range_end.min(entry_end);
        let out_range_dest = entry.dest + out_range_start - entry_start;
        out_ranges.push(Range {
            start: out_range_dest,
            len: out_range_end - out_range_start,
        });
        if range_end <= entry_end {
            break;
        }
    }
    out_ranges
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> String {
    let lines = input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();
    let lines = lines
        .split(std::string::String::is_empty)
        .collect::<Vec<_>>();

    let seeds = lines[0][0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut convert_map = ConvertMap {
        seed_to_soil: parse_to_convert_map(&lines[1][1..]),
        soil_to_fertilizer: parse_to_convert_map(&lines[2][1..]),
        fertilizer_to_water: parse_to_convert_map(&lines[3][1..]),
        water_to_light: parse_to_convert_map(&lines[4][1..]),
        light_to_temperature: parse_to_convert_map(&lines[5][1..]),
        temperature_to_humidity: parse_to_convert_map(&lines[6][1..]),
        humidity_to_location: parse_to_convert_map(&lines[7][1..]),
    };

    convert_map.seed_to_soil.sort_by_key(|e| e.src);
    convert_map.soil_to_fertilizer.sort_by_key(|e| e.src);
    convert_map.fertilizer_to_water.sort_by_key(|e| e.src);
    convert_map.water_to_light.sort_by_key(|e| e.src);
    convert_map.light_to_temperature.sort_by_key(|e| e.src);
    convert_map.temperature_to_humidity.sort_by_key(|e| e.src);
    convert_map.humidity_to_location.sort_by_key(|e| e.src);

    let min_location = seeds
        .iter()
        .copied()
        .map(|seed| find_map(&convert_map.seed_to_soil, seed))
        .map(|soil| find_map(&convert_map.soil_to_fertilizer, soil))
        .map(|fertilizer| find_map(&convert_map.fertilizer_to_water, fertilizer))
        .map(|water| find_map(&convert_map.water_to_light, water))
        .map(|light| find_map(&convert_map.light_to_temperature, light))
        .map(|temperature| find_map(&convert_map.temperature_to_humidity, temperature))
        .map(|humidity| find_map(&convert_map.humidity_to_location, humidity))
        .min()
        .unwrap();

    format!("{min_location}")
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> String {
    let lines = input
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();
    let lines = lines
        .split(std::string::String::is_empty)
        .collect::<Vec<_>>();

    let seeds = lines[0][0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut convert_map = ConvertMap {
        seed_to_soil: parse_to_convert_map(&lines[1][1..]),
        soil_to_fertilizer: parse_to_convert_map(&lines[2][1..]),
        fertilizer_to_water: parse_to_convert_map(&lines[3][1..]),
        water_to_light: parse_to_convert_map(&lines[4][1..]),
        light_to_temperature: parse_to_convert_map(&lines[5][1..]),
        temperature_to_humidity: parse_to_convert_map(&lines[6][1..]),
        humidity_to_location: parse_to_convert_map(&lines[7][1..]),
    };

    convert_map.seed_to_soil.sort_by_key(|e| e.src);
    convert_map.soil_to_fertilizer.sort_by_key(|e| e.src);
    convert_map.fertilizer_to_water.sort_by_key(|e| e.src);
    convert_map.water_to_light.sort_by_key(|e| e.src);
    convert_map.light_to_temperature.sort_by_key(|e| e.src);
    convert_map.temperature_to_humidity.sort_by_key(|e| e.src);
    convert_map.humidity_to_location.sort_by_key(|e| e.src);

    convert_map.seed_to_soil = fill_convert_map(convert_map.seed_to_soil);
    convert_map.soil_to_fertilizer = fill_convert_map(convert_map.soil_to_fertilizer);
    convert_map.fertilizer_to_water = fill_convert_map(convert_map.fertilizer_to_water);
    convert_map.water_to_light = fill_convert_map(convert_map.water_to_light);
    convert_map.light_to_temperature = fill_convert_map(convert_map.light_to_temperature);
    convert_map.temperature_to_humidity = fill_convert_map(convert_map.temperature_to_humidity);
    convert_map.humidity_to_location = fill_convert_map(convert_map.humidity_to_location);

    let mut seed_ranges = seeds
        .chunks(2)
        .map(|range| Range {
            start: range[0],
            len: range[1],
        })
        .collect::<Vec<_>>();
    seed_ranges.sort_by_key(|r| r.start);

    let mut soil_ranges = seed_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.seed_to_soil, *range))
        .collect::<Vec<_>>();
    soil_ranges.sort_by_key(|r| r.start);

    let mut fertilizer_ranges = soil_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.soil_to_fertilizer, *range))
        .collect::<Vec<_>>();
    fertilizer_ranges.sort_by_key(|r| r.start);

    let mut water_ranges = fertilizer_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.fertilizer_to_water, *range))
        .collect::<Vec<_>>();
    water_ranges.sort_by_key(|r| r.start);

    let mut light_ranges = water_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.water_to_light, *range))
        .collect::<Vec<_>>();
    light_ranges.sort_by_key(|r| r.start);

    let mut temperature_ranges = light_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.light_to_temperature, *range))
        .collect::<Vec<_>>();
    temperature_ranges.sort_by_key(|r| r.start);

    let mut humidity_ranges = temperature_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.temperature_to_humidity, *range))
        .collect::<Vec<_>>();
    humidity_ranges.sort_by_key(|r| r.start);

    let mut location_ranges = humidity_ranges
        .iter()
        .flat_map(|range| find_map_for_range(&convert_map.humidity_to_location, *range))
        .collect::<Vec<_>>();
    location_ranges.sort_by_key(|r| r.start);

    let min_location = location_ranges[0].start;

    format!("{min_location}")
}
