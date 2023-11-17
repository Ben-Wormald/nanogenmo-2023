use itertools::Itertools;
use petgraph::{graph::{Edge, NodeIndex, UnGraph}, data::FromElements, algo::min_spanning_tree};
use rand::seq::SliceRandom;
use rand_seeder::SipRng;
use std::{cmp::Ordering, collections::HashMap, env, fmt::{Debug, Display}};

const ENTITIES: usize = 104;
const ATTRIBUTES: usize = 104;
const DEFAULT_SEED: &str = "zebra";

#[derive(Clone, Debug, Eq, PartialEq)]
enum ClueType {
    Is,
    Left,
    Right,
}
impl PartialOrd for ClueType {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

#[derive(Clone)]
enum Clue<'a> {
    Is(&'a AttributeValue, &'a AttributeValue),
    Left(&'a AttributeValue, &'a AttributeValue),
    Right(&'a AttributeValue, &'a AttributeValue),
    // Neighbour(&'a AttributeValue, &'a AttributeValue),
}
impl Clue<'_> {
    fn from_edge<'a>(
        edge: &'a Edge<ClueType>,
        index_nodes: &'a HashMap<NodeIndex, &AttributeValue>,
    ) -> Clue<'a> {
        let (a, b, c) = (edge.source(), edge.target(), &edge.weight);
        let (a, b) = (index_nodes.get(&a).unwrap(), index_nodes.get(&b).unwrap());
        match c {
            ClueType::Is => Clue::Is(a, b),
            ClueType::Left => Clue::Left(a, b),
            ClueType::Right => Clue::Right(a, b),
        }
    }
}
impl Debug for Clue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clue::Is(a, b) => write!(f, "{} is {}", a.value, b.value),
            Clue::Left(a, b) => write!(f, "{} is left of {}", a.value, b.value),
            Clue::Right(a, b) => write!(f, "{} is right of {}", a.value, b.value),
            // Clue::Neighbour(a, b) => write!(f, "{} is next to {}", a.value, b.value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct AttributeValue {
    attribute: usize,
    value: Value,
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

type Solution<'a> = Vec<Vec<&'a AttributeValue>>;

fn main() {
    let seed = env::var("RNG_SEED").unwrap_or(DEFAULT_SEED.to_string());
    let mut rng: SipRng = rand_seeder::Seeder::from(seed).make_rng();

    let attr_vals = get_attribute_values();

    let random_solution = get_solution(&attr_vals, &mut rng);
    let mut possible_clues = gen_possible_clues(&random_solution);
    possible_clues.shuffle(&mut rng);

    let mut full_graph = UnGraph::<&AttributeValue, ClueType>::new_undirected();

    let mut node_indices: HashMap::<&AttributeValue, NodeIndex> = HashMap::new();
    let mut index_nodes: HashMap::<NodeIndex, &AttributeValue> = HashMap::new();

    for a_v in attr_vals.iter() {
        let node_index = full_graph.add_node(a_v);
        node_indices.insert(a_v, node_index);
        index_nodes.insert(node_index, a_v);
    }

    for clue in possible_clues.iter() {
        let (a, b, c) = match clue {
            Clue::Is(a, b) => (a, b, ClueType::Is),
            Clue::Left(a, b) => (a, b, ClueType::Left),
            Clue::Right(a, b) => (a, b, ClueType::Right),
        };
        let (a, b) = (node_indices.get(a).cloned().unwrap(), node_indices.get(b).cloned().unwrap());
        full_graph.add_edge(a, b, c);
    }
    
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&full_graph));

    let clues = mst.raw_edges().iter()
        .map(|edge| Clue::from_edge(edge, &index_nodes)).collect::<Vec<Clue>>();

    dbg!(&clues);
    dbg!(&clues.len());
}

fn get_attribute_values() -> Vec<AttributeValue> {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut attribute_values = vec!();

    for attribute in 0..ATTRIBUTES {
        for entity in 0..ENTITIES {
            let value = if attribute == 0 {
                Value::Pos(entity + 1)
            } else {
                let value_char = chars.get(entity % chars.len()).unwrap();
                Value::Str(format!("{}_{}", attribute, value_char))
            };

            attribute_values.push(AttributeValue {
                attribute,
                value,
            });
        }
    }

    attribute_values
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
                        clues.push(Clue::Left(
                            solution[attr_a][entity - 1],
                            solution[attr_b][entity],
                        ));
                        // clues.push(Clue::Neighbour(
                        //     solution[attr_a][entity - 1],
                        //     solution[attr_b][entity],
                        // ));
                    }

                    if entity < ENTITIES - 1 {
                        clues.push(Clue::Right(
                            solution[attr_a][entity + 1], 
                            solution[attr_b][entity],
                        ));
                        // clues.push(Clue::Neighbour(
                        //     solution[attr_a][entity + 1], 
                        //     solution[attr_b][entity],
                        // ));
                    }
                }
            }
        }
    }

    clues
}

fn get_solution<'a>(
    attribute_values: &'a [AttributeValue],
    mut rng: &mut SipRng,
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
