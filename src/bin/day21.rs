use itertools::Itertools;

const BOSS: Actor = Actor {
    health: 100,
    damage: 8,
    armour: 2,
};

fn main() {
    let rings = [
        Modifier::none(),
        Modifier::none(),
        Modifier::new(25, 1, 0),
        Modifier::new(50, 2, 0),
        Modifier::new(100, 3, 0),
        Modifier::new(20, 0, 1),
        Modifier::new(40, 0, 2),
        Modifier::new(80, 0, 3),
    ];
    let mut rings: Vec<_> = rings.iter().permutations(2).collect();
    rings.sort_by_key(|r| r[0].cost + r[1].cost);
    println!("Rings: {:?}", rings);
    let mut min_cost = u16::MAX;
    let mut max_cost = 0;
    for w in [
        Modifier::new(8, 4, 0),
        Modifier::new(10, 5, 0),
        Modifier::new(25, 6, 0),
        Modifier::new(40, 7, 0),
        Modifier::new(74, 8, 0),
    ].iter() {
        for a in [
            Modifier::none(),
            Modifier::new(13, 0, 1),
            Modifier::new(31, 0, 2),
            Modifier::new(53, 0, 3),
            Modifier::new(75, 0, 4),
            Modifier::new(102, 0, 5),
        ].iter() {
            for rs in rings.iter() {
                let (cost, result) = fight([*w, *a, *rs[0], *rs[1]]);
                match result {
                    Some(_) => if cost < min_cost {
                        min_cost = cost;
                    },
                    None => if cost >max_cost {
                        max_cost = cost;
                    },
                }
                //println!("Cost: {}; Result: {}", cost, result.map(|hp| format!("Won with {}hp remaining!", hp)).unwrap_or("Lost!".to_string()));
            }
        }
    }
    println!("Part 1: {}\nPart 2: {}", min_cost, max_cost);
}

fn fight(equipment: [Modifier; 4]) -> (u16, Option<i16>) {
    let mut player = Actor { health: 100, damage: 0, armour: 0 };
    let mut cost: u16 = 0;
    for e in equipment.iter() {
        cost += e.cost;
        player.damage += e.damage;
        player.armour += e.armour;
    }
    let mut boss = BOSS.clone();
    loop {
        boss.health -= player.damage - boss.armour;
        if boss.health <= 0 {
            return (cost, Some(player.health));
        }
        player.health -= boss.damage - player.armour;
        if player.health <= 0 {
            return (cost, None);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Actor {
    health: i16,
    damage: i16,
    armour: i16,
}
#[derive(Debug, Clone, Copy)]
struct Modifier {
    damage: i16,
    armour: i16,
    cost: u16,
}
impl Modifier {
    fn new(cost: u16, damage: i16, armour: i16) -> Modifier {
        Modifier { cost, damage, armour }
    }
    fn none() -> Modifier {
        Modifier { cost: 0, damage: 0, armour: 0 }
    }
}
