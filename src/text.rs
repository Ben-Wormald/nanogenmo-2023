use std::fmt::Display;

use crate::*;

const OUTPUT_FILE: &str = "./il-nome-della-zebra.txt";

pub fn write_output(clues: Vec<Clue>, rng: &mut SipRng) {
    let hours = File::open(HOURS_LIST).unwrap();
    let hours = BufReader::new(hours).lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let n_hours = hours.len();
    let mut hours = hours.into_iter().cycle();

    let title = "Il nome della zebra\n\n~\n\n";

    let intro = "\
        After a long and arduous journey I arrived at the abbey at the beginning of November, in \
        the year of our Lord MMXXIII. Alas, my plans to browse the wealth of knowledge in the \
        famous library were halted, due to the murder of the abbot that very morning. With the \
        abbey in a state of disarray and confusion, I stumbled upon a series of clues that would \
        with patience, logic, and the will of God, lead me towards the identity of the murderer \
        among these brothers. That day of my arrival, one clue was given to me by the prior: that \
        the killer was seen fleeing into cell number seven of the abbey's dormitories. With this \
        knowledge, I began my investigation in the scriptorium the following day...\n\
    ";

    let outro = "\n\
        Upon learning this crucial fact, it all became clear to me: I had determined the identity \
        of the abbot's murderer.\n\
    ";

    let text_one = [
        "At the hour of",
        "Upon",
        "As we neared the end of",
        "During",
        "When it became",
        "As the hour turned to",
        "While the bells marked the beginning of",
    ];

    let text_two = [
        "it occurred to me that",
        "it was disclosed to me that",
        "a brother informed me that",
        "a brother let slip to me that",
        "it transpired that",
        "I overheard a muttering that",
        "it struck me that",
        "it became clear that",
        "I had deduced that",
        "it was revealed to me in a vision that",
        "a certain monk confessed to me that",
        "I came across a piece of parchment indicating that",
    ];

    let mut output = File::create(OUTPUT_FILE).unwrap();

    output.write_all(title.as_bytes()).unwrap();
    output.write_all(intro.as_bytes()).unwrap();

    for (index, clue) in clues.into_iter().enumerate() {
        if index % n_hours == 0 {
            let day = index / n_hours + 1;
            let day = roman::to(day as i32).unwrap();
            let text = format!("\n\nDay {}\n\n", day);

            output.write_all(text.as_bytes()).unwrap();
        }

        let text = format!(
            "{} {} {}",
            text_one.choose(rng).unwrap(),
            hours.next().unwrap(),
            text_two.choose(rng).unwrap(),
        );
        let clue = format!("{} {}.\n", text, clue.to_string(rng));

        output.write_all(clue.as_bytes()).unwrap();
    }

    output.write_all(outro.as_bytes()).unwrap();
}

impl Clue<'_> {
    fn to_string(&self, rng: &mut SipRng) -> String {
        let text_is = [
            "is",
            "is actually",
            "is, in fact,",
            "is almost certainly",
            "is without a doubt",
        ];

        let text_lr = [
            "resides in",
            "occupies",
            "lives in",
        ];

        match self {
            Clue::Is(a, b) => {
                let text = text_is.choose(rng).unwrap();
                format!("{} {} {}", a, text, b)
            },
            Clue::Left(a, b) => {
                let text = text_lr.choose(rng).unwrap();
                format!("{} {} the cell to the left of {}", a, text, b)
            },
            Clue::Right(a, b) => {
                let text = text_lr.choose(rng).unwrap();
                format!("{} {} the cell to the right of {}", a, text, b)
            },
        }
    }
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Attribute::Str(attribute) = &self.attribute {
            match attribute.as_str() {
                "name" => write!(f, "Brother {}", self.value),
                "age" => write!(f, "the monk who is {} years of age", self.value),
                "town" => write!(f, "the monk who hails from {}", self.value),
                "saint" => write!(f, "the monk whose patron saint is {}", self.value),
                _ => panic!("no text for attribute {}", attribute)
            }
        } else {
            write!(f, "the monk who occupies cell {}", self.value)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Pos(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
        }
    }
}
