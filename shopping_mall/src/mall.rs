use std::collections::HashMap;

#[inline]
fn coerce_map<V>(m: HashMap<impl Into<String>, V>) -> HashMap<String, V> {
    m.into_iter().map(|(k, v)| (k.into(), v)).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: HashMap<String, Guard>,
    pub floors: HashMap<String, Floor>,
}

impl Mall {
    pub fn new(
        name: impl Into<String>,
        guards: HashMap<impl Into<String>, Guard>,
        floors: HashMap<impl Into<String>, Floor>,
    ) -> Self {
        Self {
            name: name.into(),
            guards: coerce_map(guards),
            floors: coerce_map(floors),
        }
    }

    pub fn change_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    pub fn hire_guard(&mut self, name: impl Into<String>, guard: Guard) {
        self.guards.insert(name.into(), guard);
    }

    pub fn fire_guard(&mut self, name: impl Into<String>) {
        self.guards.remove(&name.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guard {
    pub age: u32,
    pub years_experience: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Floor {
    pub stores: HashMap<String, Store>,
    pub size_limit: u64,
}

impl Floor {
    pub fn new(stores: HashMap<impl Into<String>, Store>, size_limit: u64) -> Self {
        Self {
            stores: coerce_map(stores),
            size_limit,
        }
    }

    pub fn replace_store(&mut self, store: impl Into<String>, with: Store) {
        self.stores.entry(store.into()).and_modify(|v| *v = with);
    }

    pub fn add_store(&mut self, name: impl Into<String>, store: Store) -> Result<(), ()> {
        let has_space = self.size_limit
            >= self.stores.values().map(|s| s.square_meters).sum::<u64>() + store.square_meters;

        if has_space {
            self.stores.insert(name.into(), store);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove_store(&mut self, name: impl Into<String>) {
        self.stores.remove(&name.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub employees: HashMap<String, Employee>,
    pub square_meters: u64,
}

impl Store {
    pub fn new(employees: HashMap<impl Into<String>, Employee>, square_meters: u64) -> Self {
        Self {
            employees: coerce_map(employees),
            square_meters,
        }
    }

    pub fn hire_employee(&mut self, name: impl Into<String>, employee: Employee) {
        self.employees.insert(name.into(), employee);
    }

    pub fn fire_employee(&mut self, name: impl Into<String>) {
        self.employees.remove(&name.into());
    }

    pub fn expand(&mut self, by: u64) {
        self.square_meters += by;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Employee {
    pub age: u32,
    // The employee works from `working_hours.0` to `working_hours.1`
    pub working_hours: (u32, u32),
    pub salary: f64,
}

impl Employee {
    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn change_workload(&mut self, from: u32, to: u32) {
        self.working_hours = (from, to);
    }

    pub fn raise(&mut self, amount: f64) {
        self.salary += amount;
    }

    pub fn cut(&mut self, amount: f64) {
        self.salary -= amount;
    }
}

// pub fn mall() -> Mall {
//     Mall::new(
//         "La Vie Funchal",
//         [
//             (
//                 "John Oliver",
//                 Guard {
//                     age: 34,
//                     years_experience: 7,
//                 },
//             ),
//             (
//                 "Logan West",
//                 Guard {
//                     age: 23,
//                     years_experience: 2,
//                 },
//             ),
//             (
//                 "Bob Schumacher",
//                 Guard {
//                     age: 53,
//                     years_experience: 15,
//                 },
//             ),
//         ]
//         .into(),
//         [
//             (
//                 "Ground Floor",
//                 Floor::new(
//                     [
//                         (
//                             "Footzo",
//                             Store::new(
//                                 [
//                                     (
//                                         "Finbar Haines",
//                                         Employee {
//                                             age: 36,
//                                             working_hours: (9, 14),
//                                             salary: 650.88,
//                                         },
//                                     ),
//                                     (
//                                         "Roksanna Rocha",
//                                         Employee {
//                                             age: 45,
//                                             working_hours: (13, 22),
//                                             salary: 772.,
//                                         },
//                                     ),
//                                     (
//                                         "Sienna-Rose Penn",
//                                         Employee {
//                                             age: 26,
//                                             working_hours: (9, 22),
//                                             salary: 1000.43,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 50,
//                             ),
//                         ),
//                         (
//                             "Swashion",
//                             Store::new(
//                                 [
//                                     (
//                                         "Abdallah Stafford",
//                                         Employee {
//                                             age: 54,
//                                             working_hours: (8, 22),
//                                             salary: 1234.21,
//                                         },
//                                     ),
//                                     (
//                                         "Marian Snyder",
//                                         Employee {
//                                             age: 21,
//                                             working_hours: (8, 14),
//                                             salary: 831.9,
//                                         },
//                                     ),
//                                     (
//                                         "Amanda Mclean",
//                                         Employee {
//                                             age: 29,
//                                             working_hours: (13, 22),
//                                             salary: 1222.12,
//                                         },
//                                     ),
//                                     (
//                                         "Faizaan Castro",
//                                         Employee {
//                                             age: 32,
//                                             working_hours: (11, 18),
//                                             salary: 1106.43,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 43,
//                             ),
//                         ),
//                     ]
//                     .into(),
//                     300,
//                 ),
//             ),
//             (
//                 "Food Floor",
//                 Floor::new(
//                     [
//                         (
//                             "PizBite",
//                             Store::new(
//                                 [
//                                     (
//                                         "Juniper Cannon",
//                                         Employee {
//                                             age: 21,
//                                             working_hours: (16, 23),
//                                             salary: 804.35,
//                                         },
//                                     ),
//                                     (
//                                         "Alena Simon",
//                                         Employee {
//                                             age: 28,
//                                             working_hours: (9, 15),
//                                             salary: 973.54,
//                                         },
//                                     ),
//                                     (
//                                         "Yasemin Collins",
//                                         Employee {
//                                             age: 29,
//                                             working_hours: (9, 19),
//                                             salary: 986.33,
//                                         },
//                                     ),
//                                     (
//                                         "Areeb Roberson",
//                                         Employee {
//                                             age: 54,
//                                             working_hours: (9, 22),
//                                             salary: 957.82,
//                                         },
//                                     ),
//                                     (
//                                         "Rocco Amin",
//                                         Employee {
//                                             age: 44,
//                                             working_hours: (13, 23),
//                                             salary: 689.21,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 60,
//                             ),
//                         ),
//                         (
//                             "Chillout Grill",
//                             Store::new(
//                                 [
//                                     (
//                                         "Rhian Crowther",
//                                         Employee {
//                                             age: 45,
//                                             working_hours: (9, 15),
//                                             salary: 841.18,
//                                         },
//                                     ),
//                                     (
//                                         "Nikkita Steadman",
//                                         Employee {
//                                             age: 52,
//                                             working_hours: (14, 22),
//                                             salary: 858.61,
//                                         },
//                                     ),
//                                     (
//                                         "Reginald Poole",
//                                         Employee {
//                                             age: 32,
//                                             working_hours: (9, 22),
//                                             salary: 1197.64,
//                                         },
//                                     ),
//                                     (
//                                         "Minnie Bull",
//                                         Employee {
//                                             age: 54,
//                                             working_hours: (14, 22),
//                                             salary: 1229.73,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 50,
//                             ),
//                         ),
//                         (
//                             "Sumo Food",
//                             Store::new(
//                                 [
//                                     (
//                                         "Chantelle Barajas",
//                                         Employee {
//                                             age: 20,
//                                             working_hours: (8, 22),
//                                             salary: 969.22,
//                                         },
//                                     ),
//                                     (
//                                         "Hywel Rudd",
//                                         Employee {
//                                             age: 49,
//                                             working_hours: (12, 22),
//                                             salary: 695.74,
//                                         },
//                                     ),
//                                     (
//                                         "Marianne Beasley",
//                                         Employee {
//                                             age: 55,
//                                             working_hours: (8, 14),
//                                             salary: 767.83,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 30,
//                             ),
//                         ),
//                     ]
//                     .into(),
//                     500,
//                 ),
//             ),
//             (
//                 "Supermarket",
//                 Floor::new(
//                     [(
//                         "Pretail",
//                         Store::new(
//                             [
//                                 (
//                                     "Amara Schaefer",
//                                     Employee {
//                                         age: 23,
//                                         working_hours: (9, 14),
//                                         salary: 796.21,
//                                     },
//                                 ),
//                                 (
//                                     "Yara Wickens",
//                                     Employee {
//                                         age: 39,
//                                         working_hours: (9, 14),
//                                         salary: 853.42,
//                                     },
//                                 ),
//                                 (
//                                     "Tomi Boyer",
//                                     Employee {
//                                         age: 64,
//                                         working_hours: (9, 14),
//                                         salary: 881.83,
//                                     },
//                                 ),
//                                 (
//                                     "Greta Dickson",
//                                     Employee {
//                                         age: 42,
//                                         working_hours: (9, 14),
//                                         salary: 775.1,
//                                     },
//                                 ),
//                                 (
//                                     "Caroline Finnegan",
//                                     Employee {
//                                         age: 41,
//                                         working_hours: (9, 14),
//                                         salary: 702.92,
//                                     },
//                                 ),
//                                 (
//                                     "Indiana Baxter",
//                                     Employee {
//                                         age: 33,
//                                         working_hours: (13, 20),
//                                         salary: 991.71,
//                                     },
//                                 ),
//                                 (
//                                     "Jadine Page",
//                                     Employee {
//                                         age: 48,
//                                         working_hours: (13, 20),
//                                         salary: 743.21,
//                                     },
//                                 ),
//                                 (
//                                     "Husna Ryan",
//                                     Employee {
//                                         age: 43,
//                                         working_hours: (13, 20),
//                                         salary: 655.75,
//                                     },
//                                 ),
//                                 (
//                                     "Tyler Hunt",
//                                     Employee {
//                                         age: 63,
//                                         working_hours: (13, 20),
//                                         salary: 668.25,
//                                     },
//                                 ),
//                                 (
//                                     "Dahlia Caldwell",
//                                     Employee {
//                                         age: 56,
//                                         working_hours: (13, 20),
//                                         salary: 781.38,
//                                     },
//                                 ),
//                                 (
//                                     "Chandler Mansell",
//                                     Employee {
//                                         age: 20,
//                                         working_hours: (19, 24),
//                                         salary: 656.75,
//                                     },
//                                 ),
//                                 (
//                                     "Mohsin Mcgee",
//                                     Employee {
//                                         age: 30,
//                                         working_hours: (19, 24),
//                                         salary: 703.83,
//                                     },
//                                 ),
//                                 (
//                                     "Antoine Goulding",
//                                     Employee {
//                                         age: 45,
//                                         working_hours: (19, 24),
//                                         salary: 697.12,
//                                     },
//                                 ),
//                                 (
//                                     "Mark Barnard",
//                                     Employee {
//                                         age: 53,
//                                         working_hours: (19, 24),
//                                         salary: 788.81,
//                                     },
//                                 ),
//                             ]
//                             .into(),
//                             950,
//                         ),
//                     )]
//                     .into(),
//                     1000,
//                 ),
//             ),
//         ]
//         .into(),
//     )
// }