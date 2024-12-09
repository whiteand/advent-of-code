use std::collections::BinaryHeap;

use smallvec::SmallVec;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Player {
    hp: usize,
    mana: usize,
    armor: usize,
    spent_mana: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Effects {
    active: usize,
}

impl Effects {
    #[inline(always)]
    fn is_active(&self, spell: Spell) -> bool {
        self.active & (spell as usize) != 0
    }
    #[inline(always)]
    fn activate(&mut self, spell: Spell) {
        self.active = self.active | (spell as usize)
    }
    #[inline(always)]
    fn deactivate(&mut self, spell: Spell) {
        self.active = self.active & !(spell as usize)
    }
}

impl Player {
    fn can_spell(&self, spell: Spell) -> bool {
        self.mana >= spell.mana_cost()
    }

    fn inc_mana(&mut self, new_mana: usize) {
        self.mana += new_mana;
    }

    /// Returns true if player is dead
    fn take_damage(&mut self, damage: usize) -> bool {
        let damage = damage.saturating_sub(self.armor).max(1);
        self.hp = self.hp.saturating_sub(damage);
        self.hp == 0
    }
}

impl Player {
    fn new(hp: usize, mana: usize) -> Self {
        Self {
            hp,
            mana,
            armor: 0,
            spent_mana: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Boss {
    hp: usize,
    damage: usize,
}

impl Boss {
    fn take_damage(&mut self, damage: usize) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spell {
    MagicMissile = 1,
    Drain = 2,
    Shield = 4,
    Poison = 8,
    Recharge = 16,
}

impl Spell {
    fn all() -> [Spell; 5] {
        use Spell::*;
        [MagicMissile, Drain, Shield, Poison, Recharge]
    }

    fn mana_cost(&self) -> usize {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
    fn duration(&self) -> usize {
        match self {
            Spell::MagicMissile => 0,
            Spell::Drain => 0,
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
        }
    }
    fn cast(&self, player: &mut Player, boss: &mut Boss) -> Option<Effect> {
        let cost = self.mana_cost();
        player.mana -= cost;
        player.spent_mana += cost;

        match self {
            Spell::MagicMissile => {
                boss.hp = boss.hp.saturating_sub(4);
                None
            }
            Spell::Drain => {
                boss.hp = boss.hp.saturating_sub(2);
                player.hp = boss.hp.saturating_add(2);
                None
            }
            x => Some(Effect {
                spell: *x,
                remaining: x.duration(),
            }),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Effect {
    spell: Spell,
    remaining: usize,
}

impl Effect {
    /// returns true when the effect should be removed
    fn cast(&mut self, effects: &mut Effects, player: &mut Player, boss: &mut Boss) -> bool {
        match self.spell {
            Spell::MagicMissile | Spell::Drain => {}
            Spell::Shield => {
                if !effects.is_active(Spell::Shield) {
                    player.armor += 7;
                    effects.activate(Spell::Shield);
                }
            }
            Spell::Poison => {
                effects.activate(Spell::Poison);
                boss.take_damage(3);
            }
            Spell::Recharge => {
                effects.activate(Spell::Poison);
                player.inc_mana(101);
            }
        }
        self.remaining -= 1;
        let cancelled = self.remaining == 0;
        if cancelled {
            match self.spell {
                Spell::MagicMissile | Spell::Drain => {}
                Spell::Shield => {
                    if effects.is_active(self.spell) {
                        effects.deactivate(self.spell);
                        player.armor = player.armor.saturating_sub(7);
                    }
                }
                Spell::Poison => {
                    effects.deactivate(Spell::Poison);
                }
                Spell::Recharge => {
                    effects.deactivate(Spell::Recharge);
                }
            }
        }
        cancelled
    }
}

#[tracing::instrument]
pub fn solve_part_1(file_content: &str) -> usize {
    let boss = parse_boss(file_content);
    least_mana_spent(Player::new(50, 500), boss)
}

fn parse_boss(input: &str) -> Boss {
    let mut lines = input.lines().filter(|x| !x.is_empty());
    let line = lines.next().unwrap();
    let mut boss = Boss { hp: 0, damage: 0 };

    if let Some(text) = line.strip_prefix("Hit Points: ") {
        boss.hp = text.trim().parse().unwrap();
    }
    if let Some(text) = line.strip_prefix("Damage: ") {
        boss.damage = text.trim().parse().unwrap();
    }

    boss
}

// Magic Missile  (-53 mana) (boss.hp -= 4)
// Drain          (-73 mana) (boss.hp -= 2) (player.hp += 2)
// Recharge      (-229 mana) (player.mana += 101)            (effect 5 turns)
// Poison        (-173 mana) (boss.hp -= 3)                  (effect 6 turns)
// Shield        (-113 mana) (+7 armor)                      (effect 6 turns)

/// 127 max hp              = 7  bit
/// 2047 max mana           = 12 bit
/// shield-effect counter   = 3  bit
/// recharge-effect counter = 3  bit
/// poison-effect counter   = 3  bit
/// 127 boss_hp             = 7  bit
#[derive(Copy, Clone, Eq, PartialEq)]
struct BitState(u64);

macro_rules! bit_field {
    ($get:ident, $set:ident, $dec:ident,$inc:ident, $offset:expr, $mask:expr, $typ:ty) => {
        impl BitState {
            #[allow(dead_code)]
            pub fn $get(&self) -> $typ {
                ((self.0 >> $offset) & $mask) as $typ
            }
            #[allow(dead_code)]
            pub fn $set(&self, new_value: $typ) -> Self {
                let disabled = ($mask << $offset) & self.0;
                let enabled = disabled | ((new_value as u64) << $offset);
                Self(enabled)
            }
            #[allow(dead_code)]
            pub fn $dec(&self, amount: $typ) -> Self {
                let value = self.$get();
                if value <= amount {
                    return self.$set(0);
                } else {
                    return self.$set(unsafe { value.unchecked_sub(1) });
                }
            }
            #[allow(dead_code)]
            pub fn $inc(&self, amount: $typ) -> Self {
                let value = self.$get();
                let new_value = value + amount;
                debug_assert!(new_value <= $mask);
                self.$set(value + amount)
            }
        }
    };
}

const PLAYER_HP_BIT_SIZE: usize = 7;
const PLAYER_MANA_BIT_SIZE: usize = 12;
const PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE: usize = 3;
const PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE: usize = 3;
const PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE: usize = 3;
const PLAYER_SPENT_MANA_BIT_SIZE: usize = 13;
const BOSS_HP_BIT_SIZE: usize = 12;

bit_field!(
    get_player_hp,
    set_player_hp,
    dec_player_hp,
    inc_player_hp,
    0,
    ((1 << (PLAYER_HP_BIT_SIZE + 1)) - 1),
    u8
);
bit_field!(
    get_player_mana,
    set_player_mana,
    dec_player_mana,
    inc_player_mana,
    PLAYER_HP_BIT_SIZE,
    ((1 << (PLAYER_MANA_BIT_SIZE + 1)) - 1),
    u8
);
bit_field!(
    get_shield_effect_counter,
    set_shield_effect_counter,
    dec_shield_effect_counter,
    inc_shield_effect_counter,
    (PLAYER_HP_BIT_SIZE+PLAYER_MANA_BIT_SIZE),
    ((1 << (PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE + 1)) - 1),
    u16
);
bit_field!(
    get_recharge_effect_counter,
    set_recharge_effect_counter,
    dec_recharge_effect_counter,
    inc_recharge_effect_counter,
    (PLAYER_HP_BIT_SIZE+PLAYER_MANA_BIT_SIZE+PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE),
    ((1 << (PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE + 1)) - 1),
    u8
);
bit_field!(
    get_poison_effect_counter,
    set_poison_effect_counter,
    dec_poison_effect_counter,
    inc_poison_effect_counter,
    (PLAYER_HP_BIT_SIZE+PLAYER_MANA_BIT_SIZE+PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE+PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE),
    ((1 << (PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE + 1)) - 1),
    u8
);
bit_field!(
    get_spent_mana,
    set_spent_mana,
    dec_spent_mana,
    inc_spent_mana,
    (PLAYER_HP_BIT_SIZE+PLAYER_MANA_BIT_SIZE+PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE+PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE+PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE),
    ((1 << (PLAYER_SPENT_MANA_BIT_SIZE + 1)) - 1),
    u16
);
bit_field!(
    get_boss_hp,
    set_boss_hp,
    dec_boss_hp,
    inc_boss_hp,
    (PLAYER_HP_BIT_SIZE+PLAYER_MANA_BIT_SIZE+PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE+PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE+PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE+PLAYER_SPENT_MANA_BIT_SIZE),
    ((1 << (BOSS_HP_BIT_SIZE + 1)) - 1),
    u8
);

#[derive(Eq, PartialEq, Debug, Clone)]
struct State {
    player: Player,
    effects: smallvec::SmallVec<[Effect; 5]>,
    active: Effects,
    boss: Boss,
    turn: Turn,
    spells: smallvec::SmallVec<[Spell; 32]>,
}
impl State {
    fn apply_effects(&mut self) {
        let effects = std::mem::replace(&mut self.effects, SmallVec::new());
        for mut effect in effects {
            if !effect.cast(&mut self.active, &mut self.player, &mut self.boss) {
                self.effects.push(effect);
            }
        }
    }

    /// Returns true if player is dead
    fn boss_turn(&mut self) -> bool {
        let res = self.player.take_damage(self.boss.damage);
        self.turn = Turn::Player;
        res
    }

    fn player_turn(&self) -> SmallVec<[Self; 5]> {
        let mut res = SmallVec::new();

        for spell in Spell::all() {
            if !self.player.can_spell(spell) {
                continue;
            }
            if self.active.is_active(spell) {
                continue;
            }
            let mut new_state = self.clone();
            if let Some(effect) = spell.cast(&mut new_state.player, &mut new_state.boss) {
                new_state.effects.push(effect);
            }
            new_state.turn = Turn::Boss;
            new_state.spells.push(spell);
            res.push(new_state);
        }

        res
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .boss
            .hp
            .cmp(&self.boss.hp)
            .then_with(|| other.player.spent_mana.cmp(&self.player.spent_mana))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Turn {
    Boss,
    Player,
}

#[tracing::instrument]
fn least_mana_spent(player: Player, boss: Boss) -> usize {
    let mut heap = BinaryHeap::new();

    let state = {
        player.
        let mut res = BitState(0)
            .player
        res.
    }
    
    heap.push(State {
        player,
        effects: SmallVec::new(),
        boss,
        spells: SmallVec::new(),
        active: Effects { active: 0 },
        turn: Turn::Player,
    });

    let mut min_spent_mana_at_boss_death = usize::MAX;

    while let Some(mut state) = heap.pop() {
        state.apply_effects();
        if state.boss.hp == 0 {
            tracing::info!(
                ?state.spells,
                spent_mana = state.player.spent_mana,
                "boss dead"
            );
            if state.player.spent_mana < min_spent_mana_at_boss_death {
                min_spent_mana_at_boss_death = state.player.spent_mana;
            }
            continue;
        }
        if state.player.hp == 0 {
            // dead player
            continue;
        }
        if state.player.spent_mana >= min_spent_mana_at_boss_death {
            continue;
        }
        match state.turn {
            Turn::Boss => {
                if state.boss_turn() {
                    // dead player
                    continue;
                }

                heap.push(state);
            }
            Turn::Player => {
                for new_state in state.player_turn() {
                    if new_state.player.spent_mana >= min_spent_mana_at_boss_death {
                        continue;
                    }
                    heap.push(new_state);
                }
            }
        }
    }

    // 5 actions
    // effects
    min_spent_mana_at_boss_death
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    todo!("part 2 is not implemented yet: {file_content}")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::least_mana_spent;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(10, 250, 13, 8, 226)]
    #[case(10, 250, 14, 8, 641)]
    #[case(50, 500, 71, 10, 0)]
    fn test_part1(
        #[case] player_hp: usize,
        #[case] player_mana: usize,
        #[case] boss_hp: usize,
        #[case] boss_damage: usize,
        #[case] expected_result: usize,
    ) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            expected_result,
            least_mana_spent(
                super::Player::new(player_hp, player_mana),
                super::Boss {
                    hp: boss_hp,
                    damage: boss_damage,
                }
            )
        );
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
