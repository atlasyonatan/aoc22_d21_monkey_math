pub mod monkey;
pub mod operator;

use self::monkey::Monkey;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvaluationError<Parse> {
    Parse(#[from] Parse),
    MissingMonkey(String),
    UnknownOperator(String)
}

pub fn parse<'a, T: FromStr + 'a>(
    line: &str,
) -> Option<(
    String,
    Monkey<'a, T, Result<T, EvaluationError<<T as FromStr>::Err>>>,
)> {
    lazy_static! {
        static ref REG_LINE: Regex = Regex::new(r"^(\w+):\s*(.+)$").unwrap();
        static ref REG_EXPRESSION: Regex = Regex::new(r"^(\w+)\s*(\W+?)\s*(\w+)$").unwrap();
    }
    let captures = REG_LINE.captures(&line)?;
    let [name, expression] = [1, 2].map(|group| captures.get(group).unwrap().as_str().to_string());
    let function = match REG_EXPRESSION.captures(&expression) {
        None => Some(Monkey::new(move |_, _| Ok(expression.parse::<T>()?))),
        Some(captures) => {
            let left = captures.get(1)?.as_str().to_string();
            let operator = captures.get(2)?.as_str().to_string();
            let right = captures.get(3)?.as_str().to_string();
            Some(Monkey::new(move |monkeys, operators| {
                let operator = &operators
                    .get(&operator)
                    .ok_or(EvaluationError::UnknownOperator(operator.clone()))?;
                let left = monkeys
                    .get(&left)
                    .ok_or(EvaluationError::MissingMonkey(left.clone()))?
                    .evaluate(monkeys, operators)?;
                let right = monkeys
                    .get(&right)
                    .ok_or(EvaluationError::MissingMonkey(right.clone()))?
                    .evaluate(monkeys, operators)?;
                Ok(operator.evaluate(left, right))
            }))
        }
    }?;
    Some((name, function))
}
