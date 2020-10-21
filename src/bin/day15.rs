use std::ops::Mul;

const FROSTING      : Ingredient = Ingredient { capacity:  4, durability: -2, flavor:  0, texture: 0, calories: 5 };
const CANDY         : Ingredient = Ingredient { capacity:  0, durability:  5, flavor: -1, texture: 0, calories: 8 };
const BUTTERSCOTCH  : Ingredient = Ingredient { capacity: -1, durability:  0, flavor:  5, texture: 0, calories: 6 };
const SUGAR         : Ingredient = Ingredient { capacity:  0, durability:  0, flavor: -2, texture: 2, calories: 1 };

fn main() {
   /*
    * Candy > 2.5 * Frosting.
    * Frosting > 4 * Butterscotch
    * Butterscotch > 2.5 * Sugar + 5 * Candy
    * Sugar > 0
    */
    let mut max_score = 0;
    let mut max_500_score = 0;
    for s in 1..=97 {
        for b in 1..=(98 - s) {
            for f in 1..=(99 - s - b) {
                let c = 100 - s - b - f;
                let (score, calories) = cookie_score(f, c, b, s);
                if score > max_score {
                    max_score = score;
                }
                if calories == 500 && score > max_500_score {
                    max_500_score = score;
                }
            }
        }
    }
    println!("Part 1: {}\nPart 2: {}", max_score, max_500_score);
}

fn cookie_score(frosting: u8, candy: u8, butterscotch: u8, sugar: u8) -> (i32, i32) {
    if frosting + candy + butterscotch + sugar != 100 {
        panic!("Invalid number of ingredients!");
    }
    let f = FROSTING * frosting.into();
    let c = CANDY * candy.into();
    let b = BUTTERSCOTCH * butterscotch.into();
    let s = SUGAR * sugar.into();
    let calories = f.calories + c.calories + b.calories + s.calories;
    let capacity = f.capacity + c.capacity + b.capacity + s.capacity;
    if capacity < 0 { return (0, calories) }
    let durability = f.durability + c.durability + b.durability + s.durability;
    if durability < 0 { return (0, calories) }
    let flavor = f.flavor + c.flavor + b.flavor + s.flavor;
    if flavor < 0 { return (0, calories) }
    let texture = f.texture + c.texture + b.texture + s.texture;
    if texture < 0 { return (0, calories) }
    (capacity * durability * flavor * texture, calories)
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32, //(how well it helps the cookie absorb milk)
    durability: i32, // (how well it keeps the cookie intact when full of milk)
    flavor: i32, // (how tasty it makes the cookie)
    texture: i32, // (how it improves the feel of the cookie)
    calories: i32, //(how many calories it adds to the cookie)
}
impl Mul<i32> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}