pub mod mall;

pub use crate::mall::*;

use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .iter()
        .flat_map(|(_, floor)| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.clone(), store.clone()))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, &Employee)> {
    let mut highest = 0.0;
    let mut result: Vec<(&str, &Employee)> = vec![];

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, emp) in &store.employees {
                if emp.salary > highest {
                    highest = emp.salary;
                    result = vec![(name, emp)];
                } else if (emp.salary - highest).abs() < f64::EPSILON {
                    result.push((name, emp));
                }
            }
        }
    }
    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let guard_count = mall.guards.len();
    let employee_count = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum::<usize>();

    guard_count + employee_count
}

pub fn check_for_securities(mall: &mut Mall, mut candidates: HashMap<String, Guard>) {
    let total_size: u64 = mall.floors.values().map(|f| f.size_limit).sum();

    let needed_guards = (total_size as f64 / 200.0).ceil() as usize;
    let current = mall.guards.len();

    if needed_guards > current {
        let to_hire = needed_guards - current;
        for (name, guard) in candidates.drain().take(to_hire) {
            mall.hire_guard(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let hours = emp.working_hours.1 - emp.working_hours.0;
                if hours >= 10 {
                    emp.raise(emp.salary * 0.1);
                } else {
                    emp.cut(emp.salary * 0.1);
                }
            }
        }
    }
}
