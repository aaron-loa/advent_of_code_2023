use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Operator {
    GreaterThan,
    LessThan,
    NoOperator,
}
impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::GreaterThan => ">".to_string(),
            Operator::LessThan => "<".to_string(),
            Operator::NoOperator => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Category {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReturnType {
    Next(String),
    Finish(Category),
}

impl FromStr for ReturnType {
    type Err = ();
    fn from_str(input: &str) -> Result<ReturnType, Self::Err> {
        if input.len() == 1 {
            match input {
                "A" => Ok(ReturnType::Finish(Category::Accepted)),
                "R" => Ok(ReturnType::Finish(Category::Rejected)),
                _ => Err(()),
            }
        } else {
            Ok(ReturnType::Next(input.to_string()))
        }
    }
}
#[derive(Debug)]
struct Condition {
    operator: Operator,
    left_side: String,
    right_side: usize,
    return_type: ReturnType,
}

#[derive(Debug)]
struct Rule {
    name: String,
    conditions: Vec<Condition>,
}

impl Rule {
    fn check(&self, x: usize, m: usize, a: usize, s: usize) -> ReturnType {
        for condition in &self.conditions {
            let left_side = match condition.left_side.as_str() {
                "x" => Some(x),
                "m" => Some(m),
                "a" => Some(a),
                "s" => Some(s),
                _ => None
            };
            if left_side.is_none() {
                return condition.return_type.clone();
            }
            let left_side = left_side.unwrap();
            let right_side = condition.right_side;
            let operator = &condition.operator;
            match operator {
                Operator::GreaterThan => {
                    if left_side > right_side {
                        return condition.return_type.clone();
                    }
                }
                Operator::LessThan => {
                    if left_side < right_side {
                        return condition.return_type.clone();
                    }
                }
                Operator::NoOperator => {
                    return condition.return_type.clone();
                }
            }
        }
        return self.conditions[self.conditions.len() - 1].return_type.clone();
    }
}

fn part1() {
    let input = include_str!("./input");
    let (rules, values) = input.split_once("\n\n").unwrap();
    let mut rule_map = HashMap::new();

    rules.lines().for_each(|rule| {
        let (name, conditions) = rule.split_once("{").unwrap();
        let mut rule = Rule {
            name: name.trim().to_string(),
            conditions: vec![],
        };
        conditions[..conditions.len() - 1]
            .split(",")
            .for_each(|to_parse| {
                let operator;
                if to_parse.contains(">") {
                    operator = Operator::GreaterThan;
                } else if to_parse.contains("<") {
                    operator = Operator::LessThan;
                } else {
                    operator = Operator::NoOperator;
                }
                let (left_side, right_side) = to_parse.split_once(&operator.to_string()).unwrap();

                match right_side.split_once(":") {
                    Some((value, category)) => {
                        rule.conditions.push(Condition {
                            operator,
                            left_side: left_side.trim().to_string(),
                            right_side: value.trim().parse::<usize>().unwrap(),
                            return_type: ReturnType::from_str(category.trim()).unwrap(),
                        });
                    }
                    None => {
                        rule.conditions.push(Condition {
                            operator,
                            left_side: left_side.trim().to_string(),
                            right_side: 0,
                            return_type: ReturnType::from_str(right_side).unwrap(),
                        });
                    }
                }
            });

        rule_map.insert(rule.name.clone(), rule);
    });
    let values = values
        .lines()
        .map(|value| {
            let values = value[1..value.len() - 1]
                .split(",")
                .map(|to_parse| {
                    let (_, right) = to_parse.split_once("=").unwrap();
                    return right.parse::<usize>().unwrap();
                })
                .collect::<Vec<usize>>();
            return values;
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for value in values {
        let (x, m, a, s) = (value[0], value[1], value[2], value[3]);
        let mut current_rule = rule_map.get("in").unwrap();
        loop {
            let res = current_rule.check(x, m, a, s);
            match res {
                ReturnType::Next(next) => {
                    current_rule = rule_map.get(&next).unwrap();
                }
                ReturnType::Finish(category) => {
                    match category {
                        Category::Accepted => {
                            sum += x + m + a + s;
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
    }
    println!("{}", sum);
}

fn main() {
    let start = std::time::Instant::now();
    part1();
    let end = std::time::Instant::now();
    println!("{:?}", end - start);
}
