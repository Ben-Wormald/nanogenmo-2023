use itertools::Itertools;
use petgraph::{graph::{Edge, NodeIndex, UnGraph}, data::FromElements, algo::min_spanning_tree};
use rand::seq::SliceRandom;
use rand_seeder::SipRng;
use std::{
    cmp::Ordering,
    collections::HashMap,
    env,
    fmt::Debug,
    fs::File,
    io::{BufReader, BufRead, Write},
};

mod text;

use text::write_output;

const ENTITIES: usize = 350;
const DEFAULT_SEED: &str = "zebra";
const ATTRIBUTE_LIST: &str = "./data/attributes.txt";
const HOURS_LIST: &str = "./data/hours.txt";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Attribute {
    Pos,
    Str(String),
}

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
            Clue::Is(a, b) => write!(f, "{:?} is {:?}", a.value, b.value),
            Clue::Left(a, b) => write!(f, "{:?} is left of {:?}", a.value, b.value),
            Clue::Right(a, b) => write!(f, "{:?} is right of {:?}", a.value, b.value),
            // Clue::Neighbour(a, b) => write!(f, "{:?} is next to {:?}", a.value, b.value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct AttributeValue {
    attribute: Attribute,
    value: Value,
}

#[derive(Eq, PartialEq, Hash)]
enum Value {
    Pos(usize),
    Str(String),
}
impl Debug for Value {
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

    let attributes = get_attributes();
    let attr_vals = get_attribute_values(&attributes, &mut rng);

    let random_solution = get_solution(&attr_vals, &mut rng);
    let mut possible_clues = gen_possible_clues(&random_solution, attributes.len());
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

    write_output(clues, &mut rng);
}

fn get_attributes() -> Vec<Attribute> {
    let attribute_list = File::open(ATTRIBUTE_LIST).unwrap();
    let attribute_list = BufReader::new(attribute_list).lines();

    let mut attributes = Vec::<Attribute>::new();

    for attribute in attribute_list.flatten() {
        attributes.push(match attribute.as_str() {
            "POS" => Attribute::Pos,
            _ => Attribute::Str(attribute),
        });
    }

    attributes
}

fn get_attribute_values(attributes: &[Attribute], mut rng: &mut SipRng) -> Vec<AttributeValue> {
    let mut attribute_values = vec!();

    for attribute in attributes.iter() {
        let values = match attribute {
            Attribute::Pos => vec![],
            Attribute::Str(attribute) => {
                let values = format!("./data/{attribute}.txt");
                let values = File::open(values).unwrap();
                let mut values = BufReader::new(values)
                    .lines().map_while(Result::ok).collect::<Vec<String>>();
                values.shuffle(&mut rng);
                values
            },
        };

        for entity in 0..ENTITIES {
            let value = match attribute {
                Attribute::Pos => Value::Pos(entity + 1),
                Attribute::Str(_) => Value::Str(values.get(entity).unwrap().to_string()),
            };
            
            attribute_values.push(AttributeValue {
                attribute: attribute.clone(),
                value,
            });
        }
    }

    attribute_values
}

fn gen_possible_clues<'a>(solution: &'a Solution, n_attributes: usize) -> Vec<Clue<'a>> {
    let mut clues = Vec::new();

    for attr_a in 0..n_attributes {
        for attr_b in 0..n_attributes {
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
