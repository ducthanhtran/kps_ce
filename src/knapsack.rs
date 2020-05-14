use std::cell::Cell;
use std::cmp::{self, Ordering};


thread_local!(static ITEM_ID: Cell<usize> = Cell::new(0));


#[derive(Debug, Eq)]
pub struct Item { id: usize, value: usize, weight: usize }

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.value / other.weight).cmp(&(self.value / self.weight))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq  for Item {
    fn eq(&self, other: &Self) -> bool {
        (self.value / self.weight) == (other.value / other.weight)
    }
}

pub fn item(value: usize, weight: usize) -> Item {
    ITEM_ID.with(|thread_id| {
        let id = thread_id.get();
        thread_id.set(id + 1);
        Item { id, value, weight}
    })
}

#[derive(Debug)]
pub struct UnboundedKnapsack {
    items: Vec<Item>,
    capacity: usize
}

impl UnboundedKnapsack {
    pub fn new(mut items: Vec<Item>, capacity: usize) -> UnboundedKnapsack {
        items.sort();
        UnboundedKnapsack { items, capacity }
    }

    pub fn solve(&self) -> usize {
        let mut total_values = vec![0; self.capacity + 1];
        for i in 1..total_values.len() {
            for item in &self.items {
                if item.weight <= i {
                    total_values[i] = cmp::max(total_values[i],
                                               total_values[i-item.weight] + item.value);
                }
            }
        }
        *total_values.last().unwrap()
    }
}


#[derive(Debug, Eq)]
pub struct ItemWithSetup { item: Item, setup_cost: usize, setup_time: usize }

impl ItemWithSetup {
    pub fn new(value: usize, weight: usize, setup_cost: usize, setup_time: usize) -> ItemWithSetup {
        ItemWithSetup { item: item(value, weight), setup_cost, setup_time }
    }

    /// Data string format: "value weight setup_cost setup_time"
    pub fn from_string(data: String) -> ItemWithSetup {
        let values: Vec<usize> = data.split_whitespace().map(|d| d.parse::<usize>().unwrap()).collect();
        assert_eq!(values.len(), 4);
        ItemWithSetup::new(values[0], values[1], values[2], values[3])
    }
}

impl Ord for ItemWithSetup {
    fn cmp(&self, other: &Self) -> Ordering {
       self.item.cmp(&other.item)
    }
}

impl PartialOrd for ItemWithSetup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq  for ItemWithSetup {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

#[derive(Debug)]
pub struct KnapsackWithSetups {
    items: Vec<ItemWithSetup>,
    lower_bounds: Vec<i32>,
    capacity: usize
}

impl KnapsackWithSetups {
    pub fn new(items: Vec<ItemWithSetup>, capacity: usize) -> KnapsackWithSetups {
        let lower_bounds = items.iter()
                                          .map(|x| lower_bound(x, capacity as i32))
                                          .collect();
        KnapsackWithSetups { items, lower_bounds, capacity }
    }

    #[allow(unused_variables)]
    pub fn associated_ukp(&self, selected_items: Vec<bool>) -> Option<UnboundedKnapsack> {
        unimplemented!();
    }

    pub fn num_items(&self) -> usize {
        self.items.len()
    }
}

pub fn lower_bound(item_with_setup: &ItemWithSetup, capacity: i32) -> i32 {
    let num_items = ((item_with_setup.setup_cost / item_with_setup.item.value) + 1) as i32;
    let weight = item_with_setup.item.weight as i32;
    let reduced_capacity = capacity - weight * num_items;
    if reduced_capacity < 0 {
        -1
    } else {
        num_items
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id() {
        let items = vec![item(5, 5), item(30, 20)];
        assert_eq!(items[0].id, 0);
        assert_eq!(items[1].id, 1);
    }

    #[test]
    fn test_item_sort() {
        let mut items = vec![item(10, 10), item(30, 20)];
        items.sort();
        assert!(items == vec![item(30, 20), item(10, 10)]);
    }

    #[test]
    fn test_unbounded_knapsack_creation() {
        let capacity = 5000;
        let ukp = UnboundedKnapsack::new(vec![item(30, 20), item(10, 10)], capacity);

        assert_eq!(ukp.capacity, capacity);
        assert!(ukp.items == vec![item(30, 20), item(10, 10)]);

        let ukp2 = UnboundedKnapsack::new(vec![item(5, 5), item(10, 2), item(99, 1)], capacity);
        assert!(ukp2.items == vec![item(99, 1), item(10, 2), item(5, 5)]);
    }

    #[test]
    fn test_unbounded_knapsack_solving() {
        let ukp = UnboundedKnapsack::new(vec![item(3, 2), item(2, 1)], 4);
        assert_eq!(ukp.solve(), 8);

        let ukp2 = UnboundedKnapsack::new(vec![item(10, 5), item(30, 10), item(20, 15)], 100);
        assert_eq!(ukp2.solve(), 300);
    }

    #[test]
    fn test_item_with_setup_sort() {
        let mut items: Vec<ItemWithSetup> = vec![ItemWithSetup::new(10, 10, 99, 100),
                                                 ItemWithSetup::new(20, 5, 5, 1)];
        items.sort();
        assert!(items == vec![ItemWithSetup::new(20, 5, 0, 0),
                              ItemWithSetup::new(10, 10, 0, 0)]);
    }

    #[test]
    fn test_knapsack_with_setup_creation() {
        let capacity = 5000;
        let kps = KnapsackWithSetups::new(
            vec![ItemWithSetup::new(30, 20, 5, 7), ItemWithSetup::new(80, 1, 99, 100)],
            capacity
        );

        assert_eq!(kps.capacity, capacity);
        assert!(kps.items == vec![ItemWithSetup::new(30, 20, 5, 7), ItemWithSetup::new(80, 1, 99, 100)]);
    }
}