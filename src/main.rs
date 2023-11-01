use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Debug, Display};

const ENTITIES: usize = 3;
const ATTRIBUTES: usize = 3;

#[derive(Clone)]
enum Clue<'a> {
    Is(&'a AttributeValue<'a>, &'a AttributeValue<'a>),
    Left(&'a AttributeValue<'a>, &'a AttributeValue<'a>),
    Right(&'a AttributeValue<'a>, &'a AttributeValue<'a>),
    Neighbour(&'a AttributeValue<'a>, &'a AttributeValue<'a>),
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

struct AttributeValue<'a> {
    attribute: usize,
    value: &'a Value<'a>,
}

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Pos(usize),
    Str(&'a str),
}
impl Display for Value<'_> {
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

type Solution<'a> = Vec<Vec<&'a Value<'a>>>;
impl IsValid for Solution<'_> {
    fn is_valid(&self, clue: &Clue) -> bool {
        match clue {
            Clue::Is(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|v| *v == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|v| *v == b.value).unwrap()
                };

                a_pos == b_pos
            },
            Clue::Left(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|v| *v == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|v| *v == b.value).unwrap()
                };

                b_pos >= 1 && a_pos == b_pos - 1
            },
            Clue::Right(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|v| *v == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|v| *v == b.value).unwrap()
                };

                a_pos == b_pos + 1
            },
            Clue::Neighbour(a, b) => {
                let a_pos = match a.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(a.attribute - 1).unwrap().iter()
                        .position(|v| *v == a.value).unwrap()
                };
                let b_pos = match b.value {
                    Value::Pos(value) => *value,
                    Value::Str(_) => self
                        .get(b.attribute - 1).unwrap().iter()
                        .position(|v| *v == b.value).unwrap()
                };

                (b_pos >= 1 && a_pos == b_pos - 1) || a_pos == b_pos + 1
            },
        }
    }
}

fn main() {
    let attr_vals = get_attribute_values();

    let mut clues = gen_all_clues(&attr_vals);
    clues.shuffle(&mut thread_rng());

    let mut accepted_clues: Vec<Clue> = Vec::new();

    let mut possible_solutions = gen_possible_solutions(&attr_vals);

    for clue in clues.into_iter() {
        let solutions = get_solutions(possible_solutions.clone(), &clue);
        let n_solutions = solutions.len();

        if n_solutions == 0 || n_solutions == possible_solutions.len() {
            continue;
        }

        possible_solutions = solutions;
        accepted_clues.push(clue);

        if n_solutions == 1 {
            break;
        }
    }

    dbg!(possible_solutions, accepted_clues);
}

fn get_attribute_values() -> Vec<AttributeValue<'static>> {
    vec![
        AttributeValue { attribute: 0, value: &Value::Pos(0) },
        AttributeValue { attribute: 0, value: &Value::Pos(1) },
        AttributeValue { attribute: 0, value: &Value::Pos(2) },
        AttributeValue { attribute: 1, value: &Value::Str("red") },
        AttributeValue { attribute: 1, value: &Value::Str("green") },
        AttributeValue { attribute: 1, value: &Value::Str("blue") },
        AttributeValue { attribute: 2, value: &Value::Str("circle") },
        AttributeValue { attribute: 2, value: &Value::Str("square") },
        AttributeValue { attribute: 2, value: &Value::Str("triangle") },
    ]
}

fn gen_all_clues<'a>(attribute_values: &'a Vec<AttributeValue<'a>>) -> Vec<Clue<'a>> {
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
                if value_b.value != &Value::Pos(1) {
                    clues.push(Clue::Left(value_a, value_b));
                }

                if value_b.value != &Value::Pos(ENTITIES) {
                    clues.push(Clue::Right(value_a, value_b));
                }

                clues.push(Clue::Neighbour(value_a, value_b));
            }
        }
    }

    clues
}

fn gen_possible_solutions<'a>(
    attribute_values: &Vec<AttributeValue<'a>>,
) -> Vec<Solution<'a>> {
    let attr_1_values: Vec<&Value> = attribute_values.iter()
        .filter(|attr_val| attr_val.attribute == 1)
        .map(|a_v| a_v.value).collect();
    let attr_2_values: Vec<&Value> = attribute_values.iter()
        .filter(|attr_val| attr_val.attribute == 2)
        .map(|a_v| a_v.value).collect();

    let mut solutions = Vec::new();

    for perm_1 in attr_1_values.into_iter().permutations(ENTITIES) {
        for perm_2 in attr_2_values.clone().into_iter().permutations(ENTITIES) {
            solutions.push(vec![perm_1.clone(), perm_2])
        }
    }

    solutions
}

fn get_solutions<'a>(solutions: Vec<Solution<'a>>, clue: &Clue) -> Vec<Solution<'a>> {
    solutions.into_iter()
        .filter(|solution| solution.is_valid(clue))
        .collect()
}
