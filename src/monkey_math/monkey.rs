use super::operator::Operator;
use std::collections::HashMap;

pub struct Monkey<'a, T, R>(
    Box<dyn Fn(&HashMap<String, Monkey<'a, T, R>>, &HashMap<String, Operator<T>>) -> R + 'a>,
);

impl<'a, T, R> Monkey<'a, T, R> {
    pub fn new(
        f: impl Fn(&HashMap<String, Monkey<'a, T, R>>, &HashMap<String, Operator<T>>) -> R + 'a,
    ) -> Self {
        Self(Box::new(f))
    }

    pub fn evaluate(
        &self,
        monkeys: &HashMap<String, Monkey<'a, T, R>>,
        operators: &HashMap<String, Operator<T>>,
    ) -> R {
        self.0(monkeys, operators)
    }
}
