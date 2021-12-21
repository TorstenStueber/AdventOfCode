use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, Neg, Sub},
    str::FromStr,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for &Coordinate {
    type Output = Coordinate;

    fn neg(self) -> Self::Output {
        Coordinate {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        Ok(Coordinate {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        })
    }
}

type Input = Vec<Vec<Coordinate>>;

fn parse_input(input: String) -> Input {
    let mut result = vec![];
    let mut beacons = vec![];
    let mut skip_line = true;

    for line in input.trim().lines() {
        if skip_line {
            skip_line = false;
            continue;
        }

        let line = line.trim();
        if line.is_empty() {
            result.push(beacons);
            beacons = vec![];
            skip_line = true;
        } else {
            beacons.push(line.parse().unwrap());
        }
    }

    result.push(beacons);
    result
}

/// rotate the coordinates of a set of beacons
/// rotation is in 0..24 and is one of the 24 right-angle 3d rotations
fn rotate(beacons: &Vec<Coordinate>, rotation: u8, result: &mut Vec<Coordinate>) {
    result.clear();

    beacons.iter().for_each(|Coordinate { x, y, z }| {
        let (x, y, z) = match rotation % 6 {
            0 => (*x, *y, *z),
            1 => (*y, *z, *x),
            2 => (*z, *x, *y),
            3 => (-*x, *z, *y),
            4 => (*z, -*y, *x),
            5 => (*y, *x, -*z),
            _ => unreachable!(),
        };
        let (x, y, z) = match rotation / 6 {
            0 => (x, y, z),
            1 => (x, -y, -z),
            2 => (-x, y, -z),
            3 => (-x, -y, z),
            _ => unreachable!(),
        };

        result.push(Coordinate { x, y, z });
    })
}

/// return whether a coordinate is in the range of a scanner
fn beacon_is_in_scanner_range(beacon: &Coordinate) -> bool {
    (-1000..=1000).contains(&beacon.x)
        && (-1000..=1000).contains(&beacon.y)
        && (-1000..=1000).contains(&beacon.z)
}

/// return true if the beacon coordinates are consistent given a
/// specific translation of the beacons
/// there need to be at least 12 overlapping coordinates
fn are_scanners_matching(
    scanner1: &Vec<Coordinate>,
    scanner2: &Vec<Coordinate>,
    scanner2_minus_scanner1: &Coordinate,
) -> bool {
    let mut overlapping_beacons_minus_scanner1 = HashSet::new();

    for coordinate_minus_scanner1 in scanner1 {
        let coordinate_minus_scanner2 = coordinate_minus_scanner1 - scanner2_minus_scanner1;

        if beacon_is_in_scanner_range(&coordinate_minus_scanner2) {
            overlapping_beacons_minus_scanner1.insert(coordinate_minus_scanner1);
        }
    }

    if overlapping_beacons_minus_scanner1.len() < 12 {
        return false;
    }

    for coordinate_minus_scanner2 in scanner2 {
        let coordinate_minus_scanner1 = coordinate_minus_scanner2 + scanner2_minus_scanner1;

        if beacon_is_in_scanner_range(&coordinate_minus_scanner1) {
            if !overlapping_beacons_minus_scanner1.remove(&coordinate_minus_scanner1) {
                return false;
            }
        }
    }

    overlapping_beacons_minus_scanner1.len() == 0
}

/// Match a scanner `maatching_scanner` to another scanner `base_scanner`
/// The base scanner is fixed in place and the matching scanner is rotated and translated
/// in all possible ways to find a match
/// If a match is found, then the return the respective rotated coordinates of the matching
/// scanner and its translation
fn match_scanner_to_scanner(
    base_scanner: &Vec<Coordinate>,
    matching_scanner: &Vec<Coordinate>,
    base_distance_structure: &DistanceStructure,
    matching_distance_strucure: &DistanceStructure,
) -> Option<(Vec<Coordinate>, Coordinate)> {
    let mut translation_counts = HashMap::new();
    let mut rotated_matching_scanner = vec![];

    let potentially_matching_beacons =
        preselect_matching_beacons(base_distance_structure, matching_distance_strucure);

    for rotation in 0..24 {
        rotate(matching_scanner, rotation, &mut rotated_matching_scanner);

        translation_counts.clear();

        for (base_index, scanner_index) in potentially_matching_beacons.iter() {
            let coordinate1 = &base_scanner[*base_index];
            let coordinate2 = &rotated_matching_scanner[*scanner_index];

            let translation = coordinate1 - coordinate2;

            *translation_counts.entry(translation).or_insert(0) += 1;
        }

        for (translation, count) in translation_counts.iter() {
            if *count < 12 {
                continue;
            }

            if are_scanners_matching(base_scanner, &rotated_matching_scanner, &translation) {
                return Some((rotated_matching_scanner, translation.clone()));
            }
        }
    }

    None
}

/// Use distance structures of two scanners to preselect all potential overlapping
/// pairs of beacons (given by their respective indexes).
/// "Beacons are matching" means the if the scanners are aligned at these two beacons,
/// then the scanners have at least 12 matching beacons
///
/// The main idea of this function is that beacons can only be matching if they have at least
/// 12 respective neightbors at the same Manhatten distance
/// The distance structures help to do this very efficiently
fn preselect_matching_beacons(
    distance_structure1: &DistanceStructure,
    distance_structure2: &DistanceStructure,
) -> Vec<(usize, usize)> {
    let mut matching_counter = HashMap::new();

    let mut iter1 = distance_structure1.iter();
    let mut iter2 = distance_structure2.iter();

    let mut element1 = iter1.next();
    let mut element2 = iter2.next();

    while let (Some((distance1, indexes1)), Some((distance2, indexes2))) = (element1, element2) {
        if distance1 < distance2 {
            element1 = iter1.next();
        } else if distance1 > distance2 {
            element2 = iter2.next();
        } else {
            for index1 in indexes1.iter() {
                for index2 in indexes2.iter() {
                    *matching_counter.entry((*index1, *index2)).or_insert(0) += 1;
                }
            }
            element1 = iter1.next();
            element2 = iter2.next();
        }
    }

    let mut result = vec![];
    for ((index1, index2), counts) in matching_counter {
        if counts >= 11 {
            result.push((index1, index2));
        }
    }

    result
}

type DistanceStructure = Vec<(i32, Vec<usize>)>;

/// build the distance structure of all beacons of a single scanner
/// the distance structure is a special arrangements of all manhatten distances between
/// all beacons of the scanner
/// the distance structure is defined as follows: for all pairs of distinct beacons compute
/// the manhatten distance d; then for every pair (i, j) of beacons whose distance is d
/// (where i, j are the indexes of the beacons in `scanner_beacons`), create the
/// vector v that contains all such indexes i
/// then add the entry (d, v) to the resulting distance structure
/// the resulting vector is then sorted by d
fn build_distance_structure(scanner_beacons: &Vec<Coordinate>) -> DistanceStructure {
    let mut distances_and_indexes = vec![];

    for (index1, beacon1) in scanner_beacons.iter().enumerate() {
        for (index2, beacon2) in scanner_beacons.iter().enumerate() {
            if index1 >= index2 {
                continue;
            }

            let difference = beacon1 - beacon2;
            let distance = difference.x.abs() + difference.y.abs() + difference.z.abs();

            distances_and_indexes.push((distance, index1));
            distances_and_indexes.push((distance, index2));
        }
    }

    distances_and_indexes.sort_by(|a, b| a.0.cmp(&b.0));

    let mut distance_structure = vec![];
    let mut iter = distances_and_indexes.iter();

    if let Some((mut last_distance, beacon_index)) = iter.next() {
        let mut index_vec = vec![*beacon_index];

        for (current_distance, beacon_index) in iter {
            if *current_distance != last_distance {
                distance_structure.push((last_distance, index_vec));
                index_vec = vec![];
                last_distance = *current_distance;
            }
            index_vec.push(*beacon_index);
        }
        distance_structure.push((last_distance, index_vec));
        distance_structure
    } else {
        vec![]
    }
}

/// build distance structures for all scanners
fn build_distance_structures(input: &Input) -> Vec<DistanceStructure> {
    input.iter().map(build_distance_structure).collect()
}

/// Determine where the scanners are actually positioned
/// The resulting vector has the same order or as the scanners
/// in the input. Each entry in the resulting vectors are the correctly
/// rotated beacon coordinates of the scanner and its correct translation
/// in relation to the first scanner.
fn determine_scanner_positions(input: &Input) -> Vec<(Vec<Coordinate>, Coordinate)> {
    let distance_structures = build_distance_structures(input);
    let mut non_matching_scanners = HashSet::new();

    let mut result = HashMap::new();
    result.insert(0, (input[0].clone(), Coordinate { x: 0, y: 0, z: 0 }));

    while result.len() < input.len() {
        for (scanner_index, scanner) in input.iter().enumerate() {
            if result.contains_key(&scanner_index) {
                continue;
            }

            for (base_index, (base_scanner, base_center)) in result.iter() {
                if non_matching_scanners.contains(&(*base_index, scanner_index)) {
                    continue;
                }

                let base_distance_structure = &distance_structures[*base_index];
                let scanner_distance_structure = &distance_structures[scanner_index];

                if let Some((rotated_scanner, relative_translation)) = match_scanner_to_scanner(
                    base_scanner,
                    scanner,
                    base_distance_structure,
                    scanner_distance_structure,
                ) {
                    let absolute_translation = base_center + &relative_translation;
                    result.insert(scanner_index, (rotated_scanner, absolute_translation));
                    break;
                } else {
                    non_matching_scanners.insert((*base_index, scanner_index));
                }
            }
        }
    }

    (0..input.len())
        .map(|index| result.get(&index).unwrap().clone())
        .collect()
}

fn task1(input: &Input) -> usize {
    let scanner_positions = determine_scanner_positions(input);

    let mut beacons = HashSet::new();
    for (scanner, center) in scanner_positions {
        for Coordinate { x, y, z } in scanner {
            beacons.insert(Coordinate {
                x: x + center.x,
                y: y + center.y,
                z: z + center.z,
            });
        }
    }

    beacons.len()
}

fn task2(input: &Input) -> i32 {
    let scanner_positions = determine_scanner_positions(input);
    let mut max_distance = 0;

    for (_, center1) in scanner_positions.iter() {
        for (_, center2) in scanner_positions.iter() {
            let difference = center1 - center2;
            let distance = difference.x.abs() + difference.y.abs() + difference.z.abs();
            max_distance = max_distance.max(distance);
        }
    }

    max_distance
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input(String::from(
        "
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        ",
    ));

    assert_eq!(task1(&input), 79);
    assert_eq!(task2(&input), 3621);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 483);
    assert_eq!(task2(&input), 14804);
}
