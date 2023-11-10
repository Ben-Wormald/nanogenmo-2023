use itertools::Itertools;
use rand::{seq::{SliceRandom, IteratorRandom}, thread_rng, rngs::ThreadRng};
use std::fmt::{Debug, Display};

const ENTITIES: usize = 5;
const ATTRIBUTES: usize = 5;
const PERMUTATION_SAMPLES: usize = 32;
const SOLUTION_SAMPLES: usize = 1024;

#[derive(Clone)]
enum Clue<'a> {
    Is(&'a AttributeValue, &'a AttributeValue),
    Left(&'a AttributeValue, &'a AttributeValue),
    Right(&'a AttributeValue, &'a AttributeValue),
    Neighbour(&'a AttributeValue, &'a AttributeValue),
}
impl Debug for Clue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clue::Is(a, b) => write!(f, "{} is {}", a.value, b.value),
            Clue::Left(a, b) => write!(f, "{} is left of {}", a.value, b.value),
            Clue::Right(a, b) => write!(f, "{} is right of {}", a.value, b.value),
            Clue::Neighbour(a, b) => write!(f, "{} is next to {}", a.value, b.value),
        }
    }
}

#[derive(Debug)]
struct AttributeValue {
    attribute: usize,
    value: Value,
}

#[derive(Debug, PartialEq)]
enum Value {
    Pos(usize),
    Str(String),
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Pos(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
        }
    }
}

trait IsValid {
    fn is_valid(&self, clue: &Clue) -> bool;
}

type Solution<'a> = Vec<Vec<&'a AttributeValue>>;
impl IsValid for Solution<'_> {
    fn is_valid(&self, clue: &Clue) -> bool {
        match clue {
            Clue::Is(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == b.value).unwrap()
                };

                a_pos == b_pos
            },
            Clue::Left(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == b.value).unwrap()
                };

                b_pos >= 1 && a_pos == b_pos - 1
            },
            Clue::Right(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == b.value).unwrap()
                };

                a_pos == b_pos + 1
            },
            Clue::Neighbour(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|a_v| a_v.value == b.value).unwrap()
                };

                (b_pos >= 1 && a_pos == b_pos - 1) || a_pos == b_pos + 1
            },
        }
    }
}

fn main() {
    let mut rng = &mut thread_rng();
    let attr_vals = get_attribute_values();

    let random_solution = get_solution(&attr_vals, &mut rng);
    dbg!(&random_solution);
    let mut possible_clues = gen_possible_clues(&random_solution);
    dbg!(&possible_clues);
    possible_clues.shuffle(&mut rng);

    let mut idx = 0;
    loop {
        let mut candidate_clues = possible_clues.clone();
        candidate_clues.remove(idx);

    }

    // let mut clues = gen_all_clues(&attr_vals);
    // clues.shuffle(&mut rng);
    
    // let mut accepted_clues: Vec<Clue> = Vec::new();
    
    // let possible_solutions = gen_possible_solutions(&attr_vals, &mut rng);
    
    // dbg!(attr_vals.len());
    // dbg!(clues.len());
    // dbg!(possible_solutions.len());

    // let mut possible_solutions = possible_solutions
    //     .choose_multiple(&mut rng, SOLUTION_SAMPLES).cloned().collect::<Vec<Solution>>();

    // for clue in clues.into_iter() {
    //     let solutions = get_solutions(possible_solutions.clone(), &clue);
    //     let n_solutions = solutions.len();

    //     if n_solutions == 0 || n_solutions == possible_solutions.len() {
    //         continue;
    //     }

    //     possible_solutions = solutions;
    //     accepted_clues.push(clue);

    //     if n_solutions == 1 {
    //         break;
    //     }
    // }

    // dbg!(possible_solutions, accepted_clues);
}

fn get_attribute_values() -> Vec<AttributeValue> {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut attribute_values = vec!();

    for attribute in 0..ATTRIBUTES {
        for entity in 0..ENTITIES {
            let value = if attribute == 0 {
                Value::Pos(entity + 1)
            } else {
                Value::Str(format!("{}_{}", attribute, chars.get(entity).unwrap()))
            };

            attribute_values.push(AttributeValue {
                attribute,
                value,
            });
        }
    }

    attribute_values
}

fn gen_all_clues<'a>(attribute_values: &'a Vec<AttributeValue>) -> Vec<Clue<'a>> {
    let mut clues = Vec::new();

    for value_a in attribute_values.iter() {
        for value_b in attribute_values.iter() {
            if value_a.attribute == value_b.attribute && value_a.value == value_b.value {
                continue;
            }

            if value_a.attribute != value_b.attribute {
                clues.push(Clue::Is(value_a, value_b));
            }

            if value_a.attribute != 0 {
                if value_b.value != Value::Pos(1) {
                    clues.push(Clue::Left(value_a, value_b));
                }

                if value_b.value != Value::Pos(ENTITIES) {
                    clues.push(Clue::Right(value_a, value_b));
                }

                clues.push(Clue::Neighbour(value_a, value_b));
            }
        }
    }

    clues
}

fn gen_possible_clues<'a>(solution: &'a Solution) -> Vec<Clue<'a>> {
    let mut clues = Vec::new();

    for attr_a in 0..ATTRIBUTES {
        for attr_b in 0..ATTRIBUTES {
            for entity in 0..ENTITIES {
                if attr_a != attr_b {
                    clues.push(Clue::Is(solution[attr_a][entity], solution[attr_b][entity]));
                }

                if attr_a != 0 {
                    if entity > 0 {
                        clues.push(Clue::Left(solution[attr_a][entity - 1], solution[attr_b][entity]));
                        clues.push(Clue::Neighbour(solution[attr_a][entity - 1], solution[attr_b][entity]));
                    }

                    if entity < ENTITIES - 1 {
                        clues.push(Clue::Right(solution[attr_a][entity + 1], solution[attr_b][entity]));
                        clues.push(Clue::Neighbour(solution[attr_a][entity + 1], solution[attr_b][entity]));
                    }
                }
            }
        }
    }

    clues
}

fn get_solution<'a>(
    attribute_values: &'a Vec<AttributeValue>,
    mut rng: &mut ThreadRng,
) -> Solution<'a> {
    attribute_values.iter()
        .chunks(ENTITIES).into_iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let mut chunk = chunk.collect::<Vec<&AttributeValue>>();
            if idx > 0 {
                chunk.shuffle(&mut rng);
            }
            chunk
        })
        .collect()
}

fn count_solutions(clues: &Vec<Clue>) -> usize {
    clues.iter().filter(|clue| matches!(clue, Clue::Is(_, _))).for_each(f)
    0
}

/*
[
    0: [
        colour: [r, g, b]
        shape: [c, t, s]
    ],
    1: [
        colour: [r, g, b]
        shape: [c, t, s]
    ],
    2: [
        colour: [r, g, b]
        shape: [c, t, s]
    ],
]
*/

// fn gen_possible_solutions_2<'a>(attribute_values: &'a Vec<AttributeValue>) -> Vec<Solution<'a>> {

// fn gen_possible_solutions<'a>(attribute_values: &'a Vec<AttributeValue>, rng: &mut ThreadRng) -> Vec<Solution<'a>> {
//     let value_sets: Vec<Vec<&Value>> = attribute_values.iter()
//         .chunks(ENTITIES).into_iter()
//         .map(|chunk| chunk.map(|a_v| &a_v.value).collect())
//         .collect();

//     let permutations = value_sets[1..].into_iter()
//         .map(|value_set| value_set.into_iter()
//             .map(|value| *value)
//             .permutations(ENTITIES)
//             .choose_multiple(rng, PERMUTATION_SAMPLES)
//             // .collect::<Vec<Vec<&Value>>>()
//         );

//     // dbg!(&permutations.len());

//     permutations.multi_cartesian_product().collect()
// }

fn get_solutions<'a>(solutions: Vec<Solution<'a>>, clue: &Clue) -> Vec<Solution<'a>> {
    solutions.into_iter()
        .filter(|solution| solution.is_valid(clue))
        .collect()
}
