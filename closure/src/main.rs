/// 14. クロージャ
/// pp.299-300

struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

fn main() {
    let mut city: Vec<City> = Vec::new();
    city.push(City{name: "London".to_string(), population: 9425622, country: "UK".to_string()});
    city.push(City{name: "Tokyo".to_string(), population: 13000000, country: "JP".to_string()});
    city.push(City{name: "New York".to_string(), population: 8230190, country: "US".to_string()});
    sort_cities(&mut city);

    for c in city {
        println!("name: {}, population: {}, country: {}", c.name, c.population, c.country);
    }
}
