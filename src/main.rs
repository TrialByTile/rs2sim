use std::collections::HashMap;
use rand::{rng, rngs::ThreadRng, Rng};

#[derive(Clone, Debug)]
struct Item {
    name: String,
    quantity: usize
}

impl Item {
    fn new(name: &str, quantity: usize) -> Self {
        Self {
            name: name.into(),
            quantity: quantity
        }
    }

    fn slots_needed(&self) -> usize {
        if self.name.starts_with("cert_") {
            1
        } else {
            self.quantity
        }
    }
}

#[derive(Clone, Debug)]
struct Inventory {
    pub items: [Option<Item>; 28],
    pub indices: HashMap<String, usize>
}

#[derive(Debug)]
struct Bank {
    lookup: HashMap<String, usize>,
}

impl Bank {
    pub fn store(&mut self, item: &Item) {
        match self.lookup.get_mut(&item.name) {
            Some(existing) => *existing = existing.wrapping_add(item.quantity),
            None => {
                self.lookup.insert(item.name.clone(), item.quantity);
            }
        }
    }
}

impl Inventory {
    pub fn total_of(&self, item_name: &str) -> usize {
        for item in &self.items {
            match item {
                Some(item) => if item.name == item_name {
                    return item.quantity;
                },
                _ => continue
            }
        }
        0
    }

    pub fn index_of(&self, item_name: &str) -> Option<usize> {
        self.indices.get(item_name).copied()
    }

    pub fn first_available(&self) -> Option<usize> {
        for (i, item) in self.items.iter().enumerate() {
            match item {
                Some(_) => continue,
                None => return Some(i)
            }
        }
        None
    }

    pub fn can_loot(&self) -> bool {
        self.first_available().is_some()
    }

    pub fn add_item(&mut self, item: Item) {
        if self.total_of(&item.name) > 0 {
            let idx = match self.indices.get(&item.name) {
                Some(i) => i,
                _ => panic!("invariant broken, total nonzero but indices has no record of {}", item.name)
            };
            match self.items.get_mut(*idx) {
                Some(slot) => {
                    match slot {
                        Some(existing) => {
                            existing.quantity += item.quantity;
                        },
                        None => panic!("invariant broken, there should be an item to modify but there isn't")
                    }
                },
                None => panic!("invariant broken, there should be an item to modify but there isn't")
            }
        } else {
            let slot = self.first_available().unwrap();
            self.indices.insert(item.name.clone(), slot);
            self.items[slot] = Some(item);
        }
    }

    pub fn clear(&mut self) {
        self.items.fill(None);
        self.indices.clear();
    }

    pub fn bank(&mut self, bank: &mut Bank) {
        for item in self.items.iter() {
            match item {
                Some(item) => {
                    bank.store(item)
                },
                _ => continue
            }
        }
        self.clear();
        ()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: core::array::from_fn(|elt| None),
            indices: HashMap::new()
        }
    }
}



#[derive(Debug, Clone)]
struct PlayerCoords {
    x: i32, // east/west
    y: i32, // vertical aka dungeons
    z: i32, // north/south
}

impl PlayerCoords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn coordz(&self) -> i32 {
        self.z
    }
}

struct GameContext {
    pub is_members: bool,
    pub player: Player,
}

impl GameContext {
    pub fn coordz(&self) -> i32 {
        self.player.coords.coordz()
    }

    pub fn new(is_members: bool, player: Player) -> Self {
        Self {
            is_members,
            player,
        }
    }
}

fn ultrarare_table(context: &GameContext, rng: &mut ThreadRng) -> Option<Item> {
    let choice = rng.random::<u32>() % 128;

    match choice {
        0..3 => {
            Some(Item::new("naturerune", 67))
        },
        3..5 => {
            Some(Item::new("adamant_javelin", 20))
        },
        5..7 => {
            Some(Item::new("deathrune", 45))
        },
        7..9 => {
            Some(Item::new("lawrune", 45))
        },
        9..11 => {
            Some(Item::new("rune_arrow", 42))
        },
        11..13 => {
            Some(Item::new("steel_arrow", 150))
        },
        13..16 => {
            Some(Item::new("rune_2h_sword", 1))
        },
        16..19 => {
            Some(Item::new("rune_battleaxe", 1))
        },
        19..21 => {
            Some(Item::new("rune_sq_shield", 1))
        },
        21..22 => {
            Some(Item::new("dragon_med_helm", 1))
        },
        22..23 => {
            Some(Item::new("rune_kiteshield", 1))
        },
        23..44 => {
            Some(Item::new("coins", 3000))
        },
        44..64 => {
            Some(Item::new("half_key1", 1))
        },
        64..84 => {
            Some(Item::new("half_key2", 1))
        }
        84..89 => {
            Some(Item::new("runite_bar", 1))
        },
        89..91 => {
            Some(Item::new("dragonstone", 1))
        },
        91..93 => {
            Some(Item::new("cert_silver_ore", 100))
        },
        93..113 => {
            random_jewel(context, rng)
        },
        113..128 => {
            megarare_table(context, rng)
        },
        _ => panic!("shouldn't happen")
    }
}

fn megarare_table(context: &GameContext, rng: &mut ThreadRng) -> Option<Item> {
    let choice = rng.random::<u32>() % 128;

    match choice {
        0..8 => {
            Some(Item::new("rune_spear", 1))
        },
        8..12 => {
            Some(Item::new("shield_left_half", 1))
        },
        12..15 => {
            Some(Item::new("dragon_spear", 1))
        },
        _ => None
    }
}

fn random_jewel(context: &GameContext, rng: &mut ThreadRng) -> Option<Item> {

    let modulus = if context.player.inventory.total_of(&"ring_of_wealth") > 0 {
        65
    } else {
        128
    };
    let choice = rng.random::<u32>() % modulus;

    // should never happen
    if choice >= modulus {
        panic!("Something is wonky with the rng/modulus")
    }

    match choice {
        0..32 => {
            Some(Item::new("uncut_sapphire", 1))
        },
        32..48 => {
            Some(Item::new("uncut_emerald", 1))
        },
        48..56 => {
            Some(Item::new("uncut_ruby", 1))
        },
        56..58 => {
            Some(Item::new("uncut_diamond", 1))
        },
        58..59 => {
            if context.is_members {
                Some(Item::new("rune_javelin", 5))
            } else {
                None
            }
        }
        59..60 => {
            if context.is_members {
                Some(Item::new("half_key1", 1))
            } else {
                None
            }
        },
        60..61 => {
            if context.is_members {
                Some(Item::new("half_key2", 1))
            } else {
                None
            }
        },
        61..62 => {
            if context.is_members {
                megarare_table(context, rng)
            } else {
                None
            }
        },
        62..65 => {
            if context.is_members {
                if context.coordz() > 6400 {
                    Some(Item::new("chaos_talisman", 1))
                } else {
                    Some(Item::new("nature_talisman", 1))
                }
            } else {
                None
            }
        },
        _ => None
    }
}

#[derive(Debug, Clone)]
struct RollsGemtable {
    name: String,
    chance: u32,
    outof: u32,
    stats: CombatStats,
    ticks_between_trips: usize,
    available_npcs: u32,
    attack_rate: usize,
    strength: u32,
    accuracy: u32, // with chosen combat style
    style_defense: u32, // defense against player's attach style (slash)
    //style_defense: u32, // against assumed players chosen DPS style, TODO this needs to account for all diff styles
    respawn_rate: usize // ticks between respawns
}

#[derive(Debug, Clone)]
enum MeleeStyle {
    Accurate,
    Aggressive,
    Controlled,
    Defensive
}

#[derive(Debug, Clone)]
struct MeleeDps {
    str_bonus: u32,
    style: MeleeStyle,
    accuracy: u32, // TODO make this pickable instead of assuming best DPS choice
    rate: usize, // ticks per attack
    def_bonus: u32, // use the def bonus of the style the mob you're fighting uses
}

trait HasCombatStats {
    fn str_level(&self) -> u32;

    fn is_dead(&self) -> bool;

    fn equipment_accuracy(&self) -> u32;

    fn def_level(&self) -> u32;

    fn att_level(&self) -> u32;

    fn attack_rate(&self) -> usize;

    fn equipment_strength(&self) -> u32;

    fn style_defense(&self) -> u32;

    fn deduct_hp(&mut self, amount: u32);

    fn is_npc(&self) -> bool;

    fn is_player(&self) -> bool;
}

fn run_combat_tick<A, B>(tick: usize, start_tick: usize, attacker: &A, defender: &mut B, rng: &mut ThreadRng)
where A: HasCombatStats, B: HasCombatStats {
    // osrs dps calc from wiki, probably unchanged for 04
    if tick % attacker.attack_rate() == start_tick {
        let mut eff_str = attacker.str_level(); // no boosts or prayer assumed
        eff_str += if attacker.is_npc() {1} else {3}; // assume theyre using correct style
        eff_str += 8;
        // ignore void bonus

        // todo: level up the player, increasing max hit
        let mut max_hit = eff_str;
        max_hit *= (attacker.equipment_strength() + 64);
        max_hit += 320;
        // no target-specific gear bonus
        max_hit /= 640; // integer division automatically rounds down

        let mut eff_att = attacker.att_level(); // ignore boosts
        eff_att += if defender.is_npc() {1} else {0}; // always using aggressive
        eff_att += 8;

        let eff_def = defender.def_level() + 8;

        let att_roll = eff_att * (attacker.equipment_accuracy() + 64);

        let def_roll = if defender.is_npc() {
            (defender.def_level() + 9) * (defender.style_defense() + 64)
        } else {
            eff_def * (defender.style_defense() + 64)
        };

        let hit_chance = if att_roll > def_roll {
            1.0 - (def_roll as f64 + 2.0) / (2.0*(att_roll as f64 + 1.0))
        } else {
            att_roll as f64 / (2.0*(def_roll as f64 + 1.0))
        };
        if rng.random::<f64>() < hit_chance {
            let amount = rng.random::<u32>() % max_hit + 1;
            defender.deduct_hp(amount)
        }
    }

}


#[derive(Debug, Clone)]
enum RangedStyle {
    Accurate,
    Rapid,
    Longrange
}

#[derive(Debug, Clone)]
struct RangedDps {
    ammo_str: u32,
    accuracy: u32,
    style: RangedStyle,
    rate: u32,
}

#[derive(Debug, Clone)]
struct MagicDps();

#[derive(Debug, Clone)]
enum Loadout {
    Melee(MeleeDps),
    Ranged(RangedDps),
    Magic(MagicDps),
}

#[derive(Debug, Clone)]
struct CombatStats {
    str_level: u32,
    def_level: u32,
    att_level: u32,
    hp_level: u32,
    current_hp: u32,
}

impl CombatStats {
    fn die(&mut self) {
        self.current_hp = 0;
    }

    fn deduct_hp(&mut self, amount: u32) {
        if amount > self.current_hp {
            self.die()
        } else {
            self.current_hp -= amount;
        }
    }

    fn heal_hp(&mut self, amount: u32) {
        if self.current_hp + amount > self.hp_level {
            self.current_hp = self.hp_level;
        } else {
            self.current_hp += amount;
        }
    }

    fn is_dead(&self) -> bool {
        self.current_hp == 0
    }
}

#[derive(Debug, Clone)]
struct Player {
    loadout: Loadout,
    inventory: Inventory,
    coords: PlayerCoords,
    stats: CombatStats,
}

impl Player {
    fn new(loadout: Loadout, inventory: Inventory, coords: PlayerCoords, stats: CombatStats) -> Self {
        Self {
            loadout, inventory, coords, stats,
        }
    }
}

impl HasCombatStats for  Player {
    fn is_dead(&self) -> bool {
        self.stats.is_dead()
    }

    fn str_level(&self) -> u32 {
        self.stats.str_level
    }

    fn att_level(&self) -> u32 {
        self.stats.att_level
    }

    fn def_level(&self) -> u32 {
        self.stats.def_level
    }

    fn deduct_hp(&mut self, amount: u32) {
        if amount > self.stats.current_hp {
            self.stats.die()
        } else {
            self.stats.current_hp -= amount;
        }
    }

    fn is_npc(&self) -> bool {
        false
    }

    fn is_player(&self) -> bool {
        true
    }

    fn attack_rate(&self) -> usize {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.rate
            },
            _ => todo!()
        }
    }

    fn equipment_accuracy(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.accuracy
            },
            _ => todo!()
        }
    }

    fn equipment_strength(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.str_bonus
            },
            _ => todo!()
        }
    }

    fn style_defense(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.def_bonus
            },
            _ => todo!()
        }
    }
}


impl HasCombatStats for &mut Player {
    fn is_dead(&self) -> bool {
        self.stats.is_dead()
    }

    fn str_level(&self) -> u32 {
        self.stats.str_level
    }

    fn att_level(&self) -> u32 {
        self.stats.att_level
    }

    fn def_level(&self) -> u32 {
        self.stats.def_level
    }

    fn deduct_hp(&mut self, amount: u32) {
        if amount > self.stats.current_hp {
            self.stats.die()
        } else {
            self.stats.current_hp -= amount;
        }
    }

    fn is_npc(&self) -> bool {
        false
    }

    fn is_player(&self) -> bool {
        true
    }

    fn attack_rate(&self) -> usize {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.rate
            },
            _ => todo!()
        }
    }

    fn equipment_accuracy(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.accuracy
            },
            _ => todo!()
        }
    }

    fn equipment_strength(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.str_bonus
            },
            _ => todo!()
        }
    }

    fn style_defense(&self) -> u32 {
        match &self.loadout {
            Loadout::Melee(melee) => {
                melee.def_bonus
            },
            _ => todo!()
        }
    }
}

impl HasCombatStats for RollsGemtable {
    fn is_npc(&self) -> bool {
        self.stats.is_dead()
    }

    fn str_level(&self) -> u32 {
        self.stats.str_level
    }

    fn att_level(&self) -> u32 {
        self.stats.att_level
    }

    fn def_level(&self) -> u32 {
        self.stats.def_level
    }

    fn deduct_hp(&mut self, amount: u32) {
        self.stats.deduct_hp(amount)
    }

    fn attack_rate(&self) -> usize {
        self.attack_rate
    }

    fn equipment_accuracy(&self) -> u32 {
        self.accuracy
    }

    fn equipment_strength(&self) -> u32 {
        self.strength
    }

    fn is_dead(&self) -> bool {
        self.stats.is_dead()
    }

    fn is_player(&self) -> bool {
        false
    }

    fn style_defense(&self) -> u32 {
        self.style_defense
    }
}
impl HasCombatStats for &mut RollsGemtable {
    fn is_npc(&self) -> bool {
        self.stats.is_dead()
    }

    fn str_level(&self) -> u32 {
        self.stats.str_level
    }

    fn att_level(&self) -> u32 {
        self.stats.att_level
    }

    fn def_level(&self) -> u32 {
        self.stats.def_level
    }

    fn deduct_hp(&mut self, amount: u32) {
        self.stats.deduct_hp(amount)
    }

    fn attack_rate(&self) -> usize {
       self.attack_rate
    }

    fn equipment_accuracy(&self) -> u32 {
        self.accuracy
    }

    fn equipment_strength(&self) -> u32 {
        self.strength
    }

    fn is_dead(&self) -> bool {
        self.stats.is_dead()
    }

    fn is_player(&self) -> bool {
        false
    }

    fn style_defense(&self) -> u32 {
        self.style_defense
    }
}

#[derive(Debug)]
struct TallyReport {
    food_hp: u32,
    food_eaten: u32,
    ticks_between_trips: usize,
    ticks_waiting_for_spawn: usize,
}

impl TallyReport {
    fn new(food_hp: u32) -> Self {
        Self {
            food_hp: food_hp,
            food_eaten: 0,
            ticks_between_trips: 0,
            ticks_waiting_for_spawn: 0
        }
    }

    fn bank(&mut self, ticks_till_return: usize) {
        self.ticks_between_trips += ticks_till_return;
    }

    fn eat(&mut self) {
        self.food_eaten += 1;
    }

    fn food_hp(&self) -> u32 {
        self.food_hp
    }

    fn wait_for_spawn(&mut self, ticks_till_spawn: usize) {
        self.ticks_waiting_for_spawn += ticks_till_spawn;
    }

    fn to_ticks(&self) -> usize {
        self.ticks_between_trips + self.ticks_waiting_for_spawn
    }
}

fn search_talisman(base_mob: &RollsGemtable, context: &GameContext, rng: &mut ThreadRng) -> Option<TallyReport> {
    let mut player = context.player.clone();
    let mut mob = (*base_mob).clone();
    let mut live_mobs = base_mob.available_npcs;
    let mut spawn_on = None; // next tick to spawn a mob if it had died previously
    let mut food_eaten = 0;
    let mut report = TallyReport::new(9);

    for (tick, _) in (0..1).cycle().enumerate() {
        // every minute we heal 1 hp
        if tick % 100 == 0 {
            // This gets desynchronized when we bank, TODO fix
            player.stats.heal_hp(1);
        }
        // TODO allow configurable danger level
        if player.stats.current_hp < player.stats.hp_level - 20 {
            // TODO: allow configurable food
            // we need to bank
            if food_eaten == 28 {
                food_eaten = 0;
                report.bank(mob.ticks_between_trips);
                player.stats.heal_hp(mob.ticks_between_trips as u32 / 100);
                player.stats.heal_hp(99); // assume we heal up before coming out
                mob.stats.heal_hp(99); // mob regens while we're gone
            }
            // for now we use salmon, assume we bring 28 and bank between
            player.stats.heal_hp(report.food_hp());
            // TODO resync the start_tick based on which tick we ate
            // eg start_tick = tick % player.attack_rate
            food_eaten += 1;
            report.eat()
        }
        if spawn_on.is_some() {
            if Some(tick) == spawn_on {
                live_mobs += 1;
                mob = base_mob.clone();
                spawn_on = None;
            }
        }
        if live_mobs == 0 {
            continue; // idle
        }
        run_combat_tick(tick, 0, &mut player, &mut mob, rng);
        // takes mob a tick to respond
        run_combat_tick(tick, 1, &mut mob, &mut player, rng);
        if player.is_dead() {
            return None
        }
        if mob.is_dead() {
            if rng.random::<u32>() % mob.outof < mob.chance {
                match random_jewel(context, rng) {
                    Some(item) => {
                        if item.name == "nature_talisman" {
                            break;
                        }
                    },
                    _ => {}
                }
            }
            live_mobs -= 1;
            spawn_on = Some(mob.respawn_rate + tick);
            if live_mobs == 0 {
                report.wait_for_spawn(mob.respawn_rate);
            }
        }

    }
    Some(report)
}

fn summarize_search(mob: &RollsGemtable, context: &GameContext, trial_ticks: Vec<Option<TallyReport>>) {
    let successes: Vec<usize> = trial_ticks.iter()
        .filter_map(|t| match t {
            Some(t) => Some(t.to_ticks()),
            None => None
        })
        .collect();
    let avg_ticks = successes.iter().sum::<usize>() as f32 / successes.len() as f32;
    let avg_hr = avg_ticks / 6000.0;
//    let deaths = trial_ticks.iter().take_while(|el| el.is_none()).collect::<Vec<_>>().len();
    let (total_food, total_trials) = trial_ticks.iter()
        .filter_map(|t| match t {
            Some(report) => Some(report.food_eaten),
            _ => None
        })
        .fold((0, 0), |(sum, count), val| (sum + val, count + 1));
    let food_eaten = total_food as f64 / total_trials as f64;
    println!("{:?} dropped in {avg_hr:.1} hours, {food_eaten} food eaten", mob.name)
}

fn search_talismans(mob: &RollsGemtable, context: &GameContext, trials: usize, rng: &mut ThreadRng) {
    let mut trial_ticks = Vec::new();
    for trial in 0..trials {
        let ticks_to_talisman = search_talisman(mob, context, rng);
        trial_ticks.push(ticks_to_talisman);
    }
    summarize_search(mob, context, trial_ticks);
}

fn main() {
    let mut rng = rand::rng();
    let coords = PlayerCoords::new(0, 0, 0);
    let invent: Inventory = Default::default();
    let player = Player::new(
        Loadout::Melee(
            MeleeDps {
                str_bonus: 30,
                style: MeleeStyle::Aggressive,
                accuracy: 69,
                def_bonus: 103, // against chosen mob's style! not automatically inferred
                rate: 5
            }
        ), invent, coords, CombatStats {
            str_level: 60, def_level: 40, hp_level: 60, att_level: 60, current_hp: 60
        }
    );
    let context = GameContext::new(true, player);

    let mut candidates: Vec<RollsGemtable> = Vec::new();
    candidates.push(RollsGemtable {
        name: "dwarf".to_string(),
        chance: 1,
        outof: 129,
        stats: CombatStats {
            str_level: 6,
            def_level: 6,
            att_level: 6,
            hp_level: 10,
            current_hp: 10,
        },
        attack_rate: 4,
        ticks_between_trips: 100,
        available_npcs: 5,
        respawn_rate: 50,
        style_defense: 0,
        accuracy: 5,
        strength: 7,

    });
    candidates.push(RollsGemtable {
        name: "jogre".into(),
        chance: 1,
        outof: 129,
        available_npcs: 8,
        respawn_rate: 30,
        attack_rate: 6,
        ticks_between_trips: 200,
        style_defense: 0,
        accuracy: 22,
        strength: 20,
        stats: CombatStats {
            str_level: 43,
            att_level: 43,
            def_level: 43,
            hp_level: 60,
            current_hp: 60
        }
    });
    candidates.push(RollsGemtable {
        name: "ice giant".to_string(),
        chance: 4,
        outof: 129,
        ticks_between_trips: 200,
        available_npcs: 9, // frozen waste plateau
        attack_rate: 5,
        respawn_rate: 30,
        strength: 31,
        accuracy: 29,
        style_defense: 3,
        stats: CombatStats {
            att_level: 40,
            def_level: 40,
            str_level: 40,
            hp_level: 70,
            current_hp: 70
        }

    });
    candidates.push(RollsGemtable {
        name: "paladin".to_string(),
        chance: 2,
        outof: 129,
        ticks_between_trips: 100,
        available_npcs: 13,
        attack_rate: 5,
        respawn_rate: 50,
        strength: 22,
        accuracy: 20,
        style_defense: 84,
        stats: CombatStats {
            hp_level: 57,
            current_hp: 57,
            att_level: 54,
            str_level: 54,
            def_level: 54,
        }
    });
    candidates.push(RollsGemtable {
        name: "pirate".to_string(),
        available_npcs: 8, // brimhaven pub
        chance: 1,
        outof: 129,
        ticks_between_trips: 50,
        attack_rate: 5,
        respawn_rate: 25,
        strength: 10,
        accuracy: 8,
        style_defense: 2,
        stats: CombatStats {
            att_level: 21,
            str_level: 21,
            def_level: 21,
            hp_level: 20,
            current_hp: 20
        }

    });
    candidates.push(RollsGemtable {
        name: "armed skeleton".to_string(),
        available_npcs: 5, // se crandor, north of edgeville
        chance: 2,
        outof: 129,
        ticks_between_trips: 100, // edgeville
        attack_rate: 4,
        respawn_rate: 60,
        strength: 14,
        accuracy: 15,
        style_defense: 11,
        stats: CombatStats {
            att_level: 24,
            str_level: 24,
            def_level: 24,
            hp_level: 17,
            current_hp: 17
        }
    });
    candidates.push(RollsGemtable {
        name: "chaos dwarf".to_string(),
        available_npcs: 3, // or 4, with a much farther bank distance
        chance: 5,
        outof: 129,
        ticks_between_trips: 400,
        attack_rate: 4,
        respawn_rate: 150,
        strength: 9,
        accuracy: 13,
        style_defense: 34,
        stats: CombatStats {
            hp_level: 61,
            current_hp: 61,
            att_level: 38,
            str_level: 42,
            def_level: 28
        }
    });
    candidates.push(RollsGemtable {
        name: "lv28 hobgoblin".to_string(),
        available_npcs: 10, // crafting guild, 8 for outpost (investigate)
        chance: 2,
        outof: 129,
        ticks_between_trips: 150,
        attack_rate: 4,
        respawn_rate: 100, // default rate is 100 when unspecified
        accuracy: 0,
        strength: 0,
        style_defense: 0,
        stats: CombatStats {
            hp_level: 29,
            current_hp: 29,
            str_level: 24,
            att_level: 22,
            def_level: 24
        }
    });
    candidates.push(RollsGemtable {
        name: "lv42 hobgoblin".to_string(),
        available_npcs: 8, // 10 crafting guild, 8 for outpost (investigate)
        chance: 2,
        outof: 129,
        ticks_between_trips: 250,
        attack_rate: 4,
        respawn_rate: 100, // TODO get a source for the real respawn rate
        accuracy: 8,
        strength: 10,
        style_defense: 1,
        stats: CombatStats {
            hp_level: 49,
            current_hp: 49,
            str_level: 31,
            att_level: 33,
            def_level: 36
        }
    });
    candidates.push(RollsGemtable {
        name: "fire giant".to_string(), // questionable if they can drop nature tally, will be camped
        available_npcs: 1, // or 4, in the other room. heavily competitive, maybe only get 1 or 2
        chance: 11,
        outof: 129,
        ticks_between_trips: 300,
        attack_rate: 5,
        respawn_rate: 30,
        accuracy: 29,
        strength: 31,
        style_defense: 3,
        stats: CombatStats {
            hp_level: 111,
            current_hp: 111,
            att_level: 65,
            str_level: 65,
            def_level: 65
        }
    });
    candidates.push(RollsGemtable {
        name: "black knight".to_string(),
        available_npcs: 5,
        chance: 3,
        outof: 129,
        ticks_between_trips: 250,
        attack_rate: 5,
        respawn_rate: 25,
        accuracy: 18,
        strength: 16,
        style_defense: 76,
        stats: CombatStats {
            hp_level: 42,
            current_hp: 42,
            att_level: 25,
            str_level: 25,
            def_level: 25,
        }
    });
    candidates.push(RollsGemtable {
        name: "barbarian".to_string(),
        chance: 1,
        outof: 129,
        ticks_between_trips: 75, // running over to fishing spot
        available_npcs: 5, // longhall or running around
        attack_rate: 6,
        respawn_rate: 25,
        strength: 10,
        accuracy: 8,
        style_defense: 1,
        stats: CombatStats {
            hp_level: 14,
            current_hp: 14,
            att_level: 6,
            str_level: 5,
            def_level: 5,
        }
    });
    candidates.push(RollsGemtable {
        name: "hill giant".to_string(),
        available_npcs: 6, // north of observatory
        ticks_between_trips: 200, // can fish trout/salmon at observatory pond
        chance: 3,
        outof: 129,
        attack_rate: 6,
        respawn_rate: 30,
        strength: 16,
        accuracy: 18,
        style_defense: 0,
        stats: CombatStats {
            hp_level: 35,
            current_hp: 35,
            att_level: 18,
            str_level: 22,
            def_level: 26,
        }
    });
    candidates.push(RollsGemtable {
        name: "moss giant".to_string(),
        chance: 4,
        outof: 129,
        ticks_between_trips: 200,
        available_npcs: 5, // brimhaven island
        attack_rate: 6,
        respawn_rate: 30,
        strength: 31,
        accuracy: 33,
        style_defense: 0,
        stats: CombatStats {
            hp_level: 60,
            current_hp: 60,
            att_level: 30,
            str_level: 30,
            def_level: 30,
        }

    });

    for candidate in &candidates {
        search_talismans(candidate, &context, 10000, &mut rng);
    }

}
