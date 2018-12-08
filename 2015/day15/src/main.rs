
fn main() {
    let input = "Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2\n\
                 Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9\n\
                 Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1\n\
                 Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8";
    println!("{:?}", Ingredient::from_str(input.lines().next().unwrap()).unwrap().with_quantity(10));
}

fn process_a(input: &str) -> usize {
    let ingredients: Vec<Ingredient> = input.lines().filter_map(Ingredient::from_str).collect();
    ingredients.len()
}

fn total_ingredients(ingredients: Vec<Ingredient>) -> usize {
    let input: (isize, isize, isize, isize) = (0, 0, 0, 0);
    ingredients
        .fold(input, |acc, i| {
            (acc.0 + i.capacity,
             acc.1 + i.durability,
             acc.2 + i.flavor,
             acc.3 + i.texture)
        })
        .len()
}

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    count: usize
}

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

impl Ingredient {
    fn from_str(i: &str) -> Option<Ingredient> {
        let input = i.replace(":", "").replace(",", "");
        let mut words = input.split_whitespace();
        let name = get!(words.next());
        let capacity = get!(isize::from_str_radix(get!(words.nth(1)), 10).ok());
        let durability = get!(isize::from_str_radix(get!(words.nth(1)), 10).ok());
        let flavor = get!(isize::from_str_radix(get!(words.nth(1)), 10).ok());
        let texture = get!(isize::from_str_radix(get!(words.nth(1)), 10).ok());
        Some(Ingredient{
            name: name.to_owned(),
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            count: 1
        })
    }

    fn with_quantity(&self, count: usize) -> Ingredient {
        assert_eq!(self.count, 1); // only starting from 1 is supported
        let multiplier = count as isize;
        Ingredient {
            name: self.name.clone(),
            capacity: self.capacity * multiplier,
            durability: self.durability * multiplier,
            flavor: self.flavor * multiplier,
            texture: self.texture * multiplier,
            count: count
        }
    }
}

#[test]
fn exercise_from_str() {
    let input_a = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
    assert_eq!(Ingredient::from_str(input_a), Some(Ingredient{
        name: "Butterscotch".to_owned(),
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        count: 1
    }));
    let input_b = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(Ingredient::from_str(input_b), Some(Ingredient{
        name: "Cinnamon".to_owned(),
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        count: 1
    }));
}

#[test]
fn exercise_with_quantity() {
    let input_a = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
    assert_eq!(Ingredient::from_str(input_a).unwrap().with_quantity(10), Ingredient{
        name: "Butterscotch".to_owned(),
        capacity: -10,
        durability: -20,
        flavor: 60,
        texture: 30,
        count: 10
    });
    let input_b = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(Ingredient::from_str(input_b).unwrap().with_quantity(5), Ingredient{
        name: "Cinnamon".to_owned(),
        capacity: 10,
        durability: 15,
        flavor: -10,
        texture: -5,
        count: 5
    });
}
