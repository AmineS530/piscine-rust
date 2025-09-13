pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let portions = food.nbr_of_portions;

        let kcal = parse_kcal(&food.calories.1);

        total_cals += kcal * portions;
        total_carbs += food.carbs * portions;
        total_proteins += food.proteins * portions;
        total_fats += food.fats * portions;
    }
    json::object! {
        "cals" => round_decimal(total_cals),
        "carbs" => round_decimal(total_carbs),
        "proteins" => round_decimal(total_proteins),
        "fats" => round_decimal(total_fats)
    }
}

fn parse_kcal(kcal_str: &str) -> f64 {
    kcal_str.trim_end_matches("kcal").parse::<f64>().unwrap()
}

fn round_decimal(value: f64) -> f64 {
    let multiplier = 10.0_f64.powi(2 as i32);
    (value * multiplier).round() / multiplier
}
