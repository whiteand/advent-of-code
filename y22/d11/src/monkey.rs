use std::collections::HashMap;

use super::{condition::Condition, item::Item, operation::Expression};

#[derive(Debug)]
pub struct Monkey<'m> {
    pub items: Vec<Item>,
    pub operation: Expression<'m>,
    pub condition: Condition,
    pub inspected: u64,
}

impl Monkey<'_> {
    pub fn new<It: IntoIterator<Item = Item>>(
        items: It,
        operation: Expression,
        condition: Condition,
    ) -> Monkey {
        Monkey {
            items: items.into_iter().collect(),
            operation,
            condition,
            inspected: 0,
        }
    }

    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn inspect(&mut self, item: &mut Item, divider: u64, module: u64) {
        let vars = HashMap::from_iter([("old", item.worry_level)]);
        let increased = self.operation.calculate(&vars) % module;
        let after_inspection = increased / divider;
        item.worry_level = after_inspection % module;
        self.inspected += 1;
    }

    pub fn choose(&self, item: &Item) -> usize {
        self.condition.choose(item.worry_level)
    }
}
