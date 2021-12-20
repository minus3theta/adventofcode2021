use std::collections::BTreeSet;

use adventofcode2021::get_stdin;
use derive_more::{Add, From, Mul, Neg, Sub};
use itertools::{Either, Itertools};
use once_cell::sync::Lazy;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From, Add, Sub, Neg, Mul)]
struct Coord(i32, i32, i32);

type Scanner = Vec<Coord>;

type Basis = Vec<Coord>;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input)?);

    Ok(())
}

fn solve(input: &str) -> anyhow::Result<usize> {
    let mut scanners = parse(input)?;
    let mut all_beacon = BTreeSet::new();
    all_beacon.insert(scanners.pop().unwrap().into_iter().collect::<BTreeSet<_>>());
    while !scanners.is_empty() {
        let (found, not_found): (Vec<_>, Vec<_>) = scanners
            .into_iter()
            .map(|scanner| {
                let (count, shifted) = ALL_BASIS
                    .iter()
                    .map(|basis| {
                        let transformed = transform(basis, &scanner);
                        count_match(&all_beacon, &transformed)
                    })
                    .max()
                    .unwrap();
                (count, shifted)
            })
            .partition_map(|(count, transformed)| {
                if count >= 12 {
                    Either::Left(transformed)
                } else {
                    Either::Right(transformed)
                }
            });
        all_beacon.extend(found.into_iter().map(|sc| sc.into_iter().collect()));
        scanners = not_found;
    }
    let all_beacon: BTreeSet<_> = all_beacon.into_iter().flat_map(|b| b.into_iter()).collect();
    Ok(all_beacon.len())
}

fn count_match(fixed: &BTreeSet<BTreeSet<Coord>>, target: &[Coord]) -> (usize, Scanner) {
    fixed
        .iter()
        .map(|center| {
            center
                .iter()
                .cartesian_product(target.iter())
                .map(|(&c, &t)| {
                    let diff = c - t;
                    let shifted = target.iter().map(|&t| t + diff).collect_vec();
                    let overlap = shifted.iter().filter(|&t| center.contains(t)).count();
                    (overlap, shifted)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

static PERM: Lazy<Vec<Vec<usize>>> = Lazy::new(|| [0, 1, 2].into_iter().permutations(3).collect());

static FLIP_PATTERN: Lazy<Vec<Vec<bool>>> = Lazy::new(|| {
    (0..8)
        .map(|p| (0..3).map(|i| (p >> i) & 1 == 1).collect())
        .collect()
});

static STANDARD: Lazy<Basis> = Lazy::new(|| vec![Coord(1, 0, 0), Coord(0, 1, 0), Coord(0, 0, 1)]);

static ALL_BASIS: Lazy<Vec<Basis>> = Lazy::new(|| {
    PERM.iter()
        .cartesian_product(FLIP_PATTERN.iter())
        .filter_map(|(perm, flip)| {
            (inversion_parity(perm) == flip_parity(flip)).then(|| {
                perm.iter()
                    .zip(flip.iter())
                    .map(|(&i, &f)| if f { -STANDARD[i] } else { STANDARD[i] })
                    .collect()
            })
        })
        .collect()
});

fn inversion_parity(perm: &[usize]) -> bool {
    let inversion = perm
        .iter()
        .tuple_combinations()
        .filter(|&(x, y)| x > y)
        .count();
    inversion % 2 == 1
}

fn flip_parity(flip: &[bool]) -> bool {
    flip.iter().filter(|&&b| b).count() % 2 == 1
}

fn rotate(basis: &[Coord], coord: Coord) -> Coord {
    let Coord(x, y, z) = coord;
    basis[0] * x + basis[1] * y + basis[2] * z
}

fn transform(basis: &[Coord], scanner: &[Coord]) -> Scanner {
    scanner
        .iter()
        .map(|&beacon| rotate(basis, beacon))
        .collect()
}

fn parse(input: &str) -> anyhow::Result<Vec<Scanner>> {
    let mut iter = input.lines();
    let mut ret = vec![];
    while let Some(_) = iter.next() {
        let mut scanner = vec![];
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let coord = line
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple::<(i32, _, _)>()
                .unwrap();
            scanner.push(coord.into());
        }
        ret.push(scanner);
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_basis() {
        let basis = &*ALL_BASIS;
        assert_eq!(basis.len(), 24);
        assert_eq!(basis.iter().collect::<BTreeSet<_>>().len(), 24);
    }

    #[test]
    fn test_solve() {
        let input = r"--- scanner 0 ---
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
30,-46,-14";
        assert_eq!(solve(input).unwrap(), 79);
    }
}
