use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Player {
    hp: usize,
    mana: usize,
    armor: usize,
    spent_mana: usize,
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
/// 15 boss_damage          = 4  bit
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct BitState(u64);

macro_rules! bit_field {
    ($get:ident, $set:ident, $dec:ident,$inc:ident, $offset:expr, $mask:expr, $typ:ty) => {
        impl BitState {
            #[allow(dead_code)]
            #[inline(always)]
            pub fn $get(&self) -> $typ {
                ((self.0 >> $offset) & $mask) as $typ
            }
            #[allow(dead_code)]
            #[inline(always)]
            pub fn $set(&self, new_value: $typ) -> Self {
                assert!(new_value as u64 <= $mask);
                Self(((new_value as u64) << $offset) | (self.0 & !($mask << $offset)))
            }
            #[allow(dead_code)]
            #[inline(always)]
            pub fn $dec(&self, amount: $typ) -> Self {
                assert!(amount as u64 <= $mask);
                let value = self.$get();
                if value <= amount {
                    return self.$set(0);
                } else {
                    return self.$set(unsafe { value.unchecked_sub(amount) });
                }
            }
            #[allow(dead_code)]
            #[inline(always)]
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
const BOSS_HP_BIT_SIZE: usize = 7;
const _BOSS_DAMAGE_BIT_SIZE: usize = 4;

bit_field!(
    get_player_hp,
    set_player_hp,
    dec_player_hp,
    inc_player_hp,
    0,
    0b0111_1111,
    u8
);
bit_field!(
    get_player_mana,
    set_player_mana,
    dec_player_mana,
    inc_player_mana,
    PLAYER_HP_BIT_SIZE,
    0b1111_1111_1111,
    u16
);
bit_field!(
    get_shield_effect_counter,
    set_shield_effect_counter,
    dec_shield_effect_counter,
    inc_shield_effect_counter,
    (PLAYER_HP_BIT_SIZE + PLAYER_MANA_BIT_SIZE),
    0b0111,
    u8
);
bit_field!(
    get_recharge_effect_counter,
    set_recharge_effect_counter,
    dec_recharge_effect_counter,
    inc_recharge_effect_counter,
    (PLAYER_HP_BIT_SIZE + PLAYER_MANA_BIT_SIZE + PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE),
    0b111,
    u8
);
bit_field!(
    get_poison_effect_counter,
    set_poison_effect_counter,
    dec_poison_effect_counter,
    inc_poison_effect_counter,
    (PLAYER_HP_BIT_SIZE
        + PLAYER_MANA_BIT_SIZE
        + PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE),
    0b111,
    u8
);
bit_field!(
    get_spent_mana,
    set_spent_mana,
    dec_spent_mana,
    inc_spent_mana,
    (PLAYER_HP_BIT_SIZE
        + PLAYER_MANA_BIT_SIZE
        + PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE),
    0b0001_1111_1111_1111,
    u16
);
bit_field!(
    get_boss_hp,
    set_boss_hp,
    dec_boss_hp,
    inc_boss_hp,
    (PLAYER_HP_BIT_SIZE
        + PLAYER_MANA_BIT_SIZE
        + PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_SPENT_MANA_BIT_SIZE),
    0b1111111,
    u8
);
bit_field!(
    get_boss_damage,
    set_boss_damage,
    dec_boss_damage,
    inc_boss_damage,
    (PLAYER_HP_BIT_SIZE
        + PLAYER_MANA_BIT_SIZE
        + PLAYER_SHIELD_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_RECHARGE_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_POISON_EFFECT_COUNTER_BIT_SIZE
        + PLAYER_SPENT_MANA_BIT_SIZE
        + BOSS_HP_BIT_SIZE),
    0b1111,
    u8
);

impl std::fmt::Debug for BitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "player_hp={}, player_mana={}, shield_effect_counter={}, recharge_effect_counter={}, poison_effect_counter={}, spent_mana={}, boss_hp={}, boss_damage={}",
            self.get_player_hp(),
            self.get_player_mana(),
            self.get_shield_effect_counter(),
            self.get_recharge_effect_counter(),
            self.get_poison_effect_counter(),
            self.get_spent_mana(),
            self.get_boss_hp(),
            self.get_boss_damage(),
        )
    }
}

impl BitState {
    fn apply_effects(&self, armor: &mut u8) -> Game {
        let mut new_state = *self;
        if new_state.get_poison_effect_counter() > 0 {
            new_state = new_state.dec_boss_hp(3).dec_poison_effect_counter(1);
            if new_state.get_boss_hp() == 0 {
                return Game::Win(new_state);
            }
        }
        if new_state.get_shield_effect_counter() > 0 {
            *armor = 7;
            new_state = new_state.dec_shield_effect_counter(1);
        } else {
            *armor = 0;
        }
        if new_state.get_recharge_effect_counter() > 0 {
            new_state = new_state
                .inc_player_mana(101)
                .dec_recharge_effect_counter(1);
        }
        Game::Playing(new_state)
    }

    fn boss_move(&self) -> Game {
        let mut armor = 0u8;
        let state = match self.apply_effects(&mut armor) {
            Game::Playing(bit_state) => bit_state,
            game => {
                return game;
            }
        };
        let boss_damage = state.get_boss_damage().saturating_sub(armor).max(1);
        let state = state.dec_player_hp(boss_damage);
        if state.get_player_hp() == 0 {
            Game::Loser
        } else {
            Game::Playing(state)
        }
    }
    fn spend_mana(&self, mana: u16) -> Self {
        self.dec_player_mana(mana).inc_spent_mana(mana)
    }
    fn cast_magic_missile(&self) -> Option<Game> {
        let mana = self.get_player_mana();
        const MAGIC_MISSILE_COST: u16 = 53;
        (mana >= MAGIC_MISSILE_COST).then(|| {
            let state = self.spend_mana(MAGIC_MISSILE_COST).dec_boss_hp(4);
            if state.get_boss_hp() == 0 {
                Game::Win(state)
            } else {
                Game::Playing(state)
            }
        })
    }
    fn cast_drain(&self) -> Option<Game> {
        const DRAIN_COST: u16 = 73;
        let mana = self.get_player_mana();
        (mana >= DRAIN_COST).then(|| {
            let state = self.spend_mana(DRAIN_COST).dec_boss_hp(2).inc_player_hp(2);
            if state.get_boss_hp() == 0 {
                Game::Win(state)
            } else {
                Game::Playing(state)
            }
        })
    }
}

impl PartialOrd for BitState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BitState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .get_boss_hp()
            .cmp(&self.get_boss_hp())
            .then_with(|| other.get_spent_mana().cmp(&self.get_spent_mana()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Game {
    Win(BitState),
    Loser,
    Playing(BitState),
}

impl Game {
    fn and_then(&self, f: impl FnOnce(&BitState) -> Game) -> Game {
        match self {
            Game::Playing(bit_state) => f(bit_state),
            game => *game,
        }
    }
    fn and_then_maybe(&self, f: impl FnOnce(&BitState) -> Option<Game>) -> Option<Game> {
        match self {
            Game::Playing(bit_state) => f(bit_state),
            game => Some(*game),
        }
    }
}

#[tracing::instrument(skip(player, boss))]
fn least_mana_spent(player: Player, boss: Boss) -> usize {
    let mut heap = BinaryHeap::new();

    let state = BitState(0)
        .set_player_hp(player.hp as u8)
        .set_player_mana(player.mana as u16)
        .set_boss_hp(boss.hp as u8)
        .set_boss_damage(boss.damage as u8);

    heap.push(state);
    tracing::info!(?state);

    let mut min_spent_mana_at_boss_death = u16::MAX;

    let mut min_boss_hp = boss.hp as u8;

    let mut visited = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.get_spent_mana() >= min_spent_mana_at_boss_death {
            continue;
        }
        if !visited.insert(state) {
            continue;
        }

        // before player move we should apply effects
        let mut armor = 0;

        let game = state.apply_effects(&mut armor);

        match game
            .and_then_maybe(|s| s.cast_magic_missile())
            .map(|x| x.and_then(|x| x.boss_move()))
        {
            Some(Game::Win(s)) => {
                if s.get_spent_mana() < min_spent_mana_at_boss_death {
                    min_spent_mana_at_boss_death = s.get_spent_mana();
                }
            }
            Some(Game::Playing(s)) => {
                heap.push(s);
            }
            _ => {}
        }
        match game
            .and_then_maybe(|s| s.cast_drain())
            .map(|x| x.and_then(|x| x.boss_move()))
        {
            Some(Game::Win(s)) => {
                if s.get_spent_mana() < min_spent_mana_at_boss_death {
                    min_spent_mana_at_boss_death = s.get_spent_mana();
                }
            }
            Some(Game::Playing(s)) => {
                heap.push(s);
            }
            _ => {}
        }

        let state = match game {
            Game::Win(s) => {
                if s.get_spent_mana() < min_spent_mana_at_boss_death {
                    min_spent_mana_at_boss_death = s.get_spent_mana();
                }
                continue;
            }
            Game::Playing(s) => s,
            Game::Loser => {
                continue;
            }
        };

        const RECHARGE_COST: u16 = 229;
        const RECHARGE_TURNS: u8 = 5;
        let mana = state.get_player_mana();
        if mana >= RECHARGE_COST && state.get_recharge_effect_counter() == 0 {
            // try to cast magic missile
            let state = state
                .spend_mana(RECHARGE_COST)
                .set_recharge_effect_counter(RECHARGE_TURNS);

            if state.get_boss_hp() == 0 {
                min_spent_mana_at_boss_death =
                    min_spent_mana_at_boss_death.min(state.get_spent_mana());
            } else {
                match state.boss_move() {
                    Game::Playing(state) => {
                        heap.push(state);
                    }
                    Game::Win(state) => {
                        if state.get_spent_mana() < min_spent_mana_at_boss_death {
                            min_spent_mana_at_boss_death = state.get_spent_mana();
                        }
                    }
                    Game::Loser => {
                        // do nothing
                    }
                };
            }
        }
        const POISON_COST: u16 = 173;
        const POISON_TURNS: u8 = 6;
        if mana >= POISON_COST && state.get_poison_effect_counter() == 0 {
            let mut move_armor = armor;

            // try to cast magic missile
            let state = state
                .spend_mana(POISON_COST)
                .set_poison_effect_counter(POISON_TURNS);

            if state.get_boss_hp() == 0 {
                min_spent_mana_at_boss_death =
                    min_spent_mana_at_boss_death.min(state.get_spent_mana());
            } else {
                match state.boss_move() {
                    Game::Playing(state) => {
                        heap.push(state);
                    }
                    Game::Win(state) => {
                        if state.get_spent_mana() < min_spent_mana_at_boss_death {
                            min_spent_mana_at_boss_death = state.get_spent_mana();
                        }
                    }
                    Game::Loser => {
                        // do nothing
                    }
                };
            }
        }
        const SHIELD_COST: u16 = 113;
        const SHIELD_TURNS: u8 = 6;
        if mana >= SHIELD_COST && state.get_poison_effect_counter() == 0 {
            let mut move_armor = armor;

            // try to cast magic missile
            let state = state
                .spend_mana(SHIELD_COST)
                .set_shield_effect_counter(SHIELD_TURNS);

            if state.get_boss_hp() == 0 {
                min_spent_mana_at_boss_death =
                    min_spent_mana_at_boss_death.min(state.get_spent_mana());
            } else {
                match state.boss_move() {
                    Game::Playing(state) => {
                        heap.push(state);
                    }
                    Game::Win(state) => {
                        if state.get_spent_mana() < min_spent_mana_at_boss_death {
                            min_spent_mana_at_boss_death = state.get_spent_mana();
                        }
                    }
                    Game::Loser => {
                        // do nothing
                    }
                };
            }
        }
    }

    // 5 actions
    // effects
    min_spent_mana_at_boss_death as usize
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    todo!("part 2 is not implemented yet: {file_content}")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{least_mana_spent, BitState};

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(10, 250, 13, 8, 226)]
    #[case(10, 250, 14, 8, 641)]
    #[case(50, 500, 71, 10, 1242)] // 1242 is too low
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

    #[test]
    fn test_bit_state() {
        let mut state = BitState(u64::MAX);
        //
        assert_eq!(state.get_player_hp(), 127);
        state = state.set_player_hp(3);
        assert_eq!(state.get_player_hp(), 3);
        assert_eq!(state.0, 0);

        state = state.set_player_mana(3);
        assert_eq!(state.get_player_mana(), 3);
        state = state.set_player_mana(0);
        assert_eq!(state.0, 0);

        state = state.set_shield_effect_counter(3);
        assert_eq!(state.get_shield_effect_counter(), 3);
        state = state.set_shield_effect_counter(0);
        assert_eq!(state.0, 0);

        state = state.set_recharge_effect_counter(3);
        assert_eq!(state.get_recharge_effect_counter(), 3);
        state = state.set_recharge_effect_counter(0);
        assert_eq!(state.0, 0);

        state = state.set_poison_effect_counter(3);
        assert_eq!(state.get_poison_effect_counter(), 3);
        state = state.set_poison_effect_counter(0);
        assert_eq!(state.0, 0);

        state = state.set_spent_mana(3);
        assert_eq!(state.get_spent_mana(), 3);
        state = state.set_spent_mana(0);
        assert_eq!(state.0, 0);

        state = state.set_boss_hp(3);
        assert_eq!(state.get_boss_hp(), 3);
        state = state.set_boss_hp(0);
        assert_eq!(state.0, 0);

        state = state.set_boss_damage(3);
        assert_eq!(state.get_boss_damage(), 3);
        state = state.set_boss_damage(0);
        assert_eq!(state.0, 0);
    }
}
