use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Boss {
    health: i16,
    damage: i16,
}

const BOSS: Boss = Boss {
    health: 71,
    damage: 10,
};

#[derive(Debug, Copy, Clone)]
struct Player {
    health: i16,
    armour: i16,
    mana: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Spell {
    /*Magic*/ Missile,// costs 53 mana. It instantly does 4 damage.
    Drain,// costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    Shield,// costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    Poison,// costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
    Recharge,// costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
}

enum Turn {
    Player,
    Boss,
}

fn main() {
    let mut manacost = HashMap::new();
    manacost.insert(Spell::Missile, 53);
    manacost.insert(Spell::Drain, 73);
    manacost.insert(Spell::Shield, 113);
    manacost.insert(Spell::Poison, 173);
    manacost.insert(Spell::Recharge, 229);
    fight(&manacost);
}

fn fight(manacost: &HashMap<Spell, i32>) -> Option<i16> {
    let mut player = Player { health: 50, mana: 500, armour: 0 };
    let mut boss = BOSS.clone();
    let mut effects = HashMap::new();
    let mut turn = Turn::Player;
    loop {
        if run_effects(&mut player, &mut boss, &mut effects) { // true = dead boss
            return Some(player.health);
        }
        match turn {
            Turn::Player => {
                let valid = valid_spells(&player, &manacost, &mut effects);
                println!("Effects: {:?}\nValid: {:?}", effects, valid);
                turn = Turn::Boss;
            },
            Turn::Boss => {
                player.health -= std::cmp::max(1, boss.damage - player.armour);
                if player.health <= 0 {
                    return None;
                }
                turn = Turn::Player;
            },
        }
        break None;//XXX
    }
}

fn valid_spells(player: &Player, manacost: &HashMap<Spell, i32>, effects: &mut HashMap<Spell, u8>) -> Vec<Spell> {
    manacost.iter().filter(|(&s, &c)| c < player.mana && !effects.contains_key(&s)).map(|(&s, _)| s).collect()
}

fn cast_spell(player: &mut Player, boss: &mut Boss, effects: &mut HashMap<Spell, u8>, manacost: &HashMap<Spell, i32>, spell: Spell) {
    player.mana -= manacost.get(&spell).unwrap();
    match spell {
        Spell::Missile => boss.health -= 4,
        Spell::Drain => {
            boss.health -= 2;
            player.health += 2;
        },
        Spell::Shield => {
            player.armour += 7;
            effects.insert(spell, 6);
        },
        Spell::Poison => {
            effects.insert(spell, 6);
        },
        Spell::Recharge => {
            effects.insert(spell, 5);
        },
    }
}

fn run_effects(player: &mut Player, boss: &mut Boss, effects: &mut HashMap<Spell, u8>) -> bool {
    for (spell, dur) in effects.iter_mut() {
        if *dur > 0 {
            *dur -= 1;
            match spell {
                Spell::Shield => {
                    if *dur < 1 {
                        player.armour -= 7;
                    }
                },
                Spell::Poison => boss.health -= 3,
                Spell::Recharge => player.mana += 101,
                _ => (),
            }
        }
    }
    effects.retain(|_, &mut d| d > 0);
    boss.health < 1
}