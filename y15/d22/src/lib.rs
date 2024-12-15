use std::collections::{BinaryHeap, HashSet};

use advent_utils::declare_field;

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
    tracing::info!(?boss);
    least_mana_spent(Player::new(50, 500), boss, 0)
}
#[tracing::instrument]
pub fn solve_part_2(file_content: &str) -> usize {
    let boss = parse_boss(file_content);
    tracing::info!(?boss);
    least_mana_spent(Player::new(50, 500), boss, 1)
}

fn parse_boss(input: &str) -> Boss {
    let mut lines = input.lines().filter(|x| !x.is_empty());

    let mut boss = Boss { hp: 0, damage: 0 };

    if let Some(text) = lines.next().unwrap().strip_prefix("Hit Points: ") {
        boss.hp = text.trim().parse().unwrap();
    }

    if let Some(text) = lines.next().unwrap().strip_prefix("Damage: ") {
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

impl BitState {
    declare_field!(u64, u8, get_player_hp, set_player_hp, 0, 0b0111_1111);
    declare_field!(
        u64,
        u16,
        get_player_mana,
        set_player_mana,
        7,
        0b1111_1111_1111
    );
    declare_field!(
        u64,
        u8,
        get_shield_effect_counter,
        set_shield_effect_counter,
        19,
        0b111
    );
    declare_field!(
        u64,
        u8,
        get_recharge_effect_counter,
        set_recharge_effect_counter,
        22,
        0b111
    );
    declare_field!(
        u64,
        u8,
        get_poison_effect_counter,
        set_poison_effect_counter,
        25,
        0b111
    );
    declare_field!(
        u64,
        u16,
        get_spent_mana,
        set_spent_mana,
        28,
        0b0001_1111_1111_1111
    );
    declare_field!(u64, u8, get_boss_hp, set_boss_hp, 41, 0b0111_1111);
    declare_field!(u64, u8, get_boss_damage, set_boss_damage, 48, 0b1111);

    #[inline(always)]
    pub fn dec_player_hp(&self, amount: u8) -> Self {
        assert!(amount as u64 <= 0b0111_1111);
        let value = self.get_player_hp();
        if value <= amount {
            return self.set_player_hp(0);
        } else {
            return self.set_player_hp(unsafe { value.unchecked_sub(amount) });
        }
    }
    #[inline(always)]
    pub fn inc_player_hp(&self, amount: u8) -> Self {
        let value = self.get_player_hp();
        let new_value = value + amount;
        debug_assert!(new_value <= 0b0111_1111);
        self.set_player_hp(value + amount)
    }
    #[inline(always)]
    pub fn dec_player_mana(&self, amount: u16) -> Self {
        assert!(amount as u64 <= 0b1111_1111_1111);
        let value = self.get_player_mana();
        if value <= amount {
            return self.set_player_mana(0);
        } else {
            return self.set_player_mana(unsafe { value.unchecked_sub(amount) });
        }
    }
    #[inline(always)]
    pub fn inc_player_mana(&self, amount: u16) -> Self {
        let value = self.get_player_mana();
        let new_value = value + amount;
        debug_assert!(new_value <= 0b1111_1111_1111);
        self.set_player_mana(value + amount)
    }

    #[inline(always)]
    pub fn dec_shield_effect_counter(&self, amount: u8) -> Self {
        assert!(amount as u64 <= 0b0111);
        let value = self.get_shield_effect_counter();
        if value <= amount {
            return self.set_shield_effect_counter(0);
        } else {
            return self.set_shield_effect_counter(unsafe { value.unchecked_sub(amount) });
        }
    }
    #[inline(always)]
    pub fn dec_recharge_effect_counter(&self, amount: u8) -> Self {
        assert!(amount as u64 <= 0b111);
        let value = self.get_recharge_effect_counter();
        if value <= amount {
            return self.set_recharge_effect_counter(0);
        } else {
            return self.set_recharge_effect_counter(unsafe { value.unchecked_sub(amount) });
        }
    }

    #[inline(always)]
    pub fn dec_poison_effect_counter(&self, amount: u8) -> Self {
        assert!(amount as u64 <= 0b111);
        let value = self.get_poison_effect_counter();
        if value <= amount {
            return self.set_poison_effect_counter(0);
        } else {
            return self.set_poison_effect_counter(unsafe { value.unchecked_sub(amount) });
        }
    }

    #[inline(always)]
    pub fn inc_spent_mana(&self, amount: u16) -> Self {
        let value = self.get_spent_mana();
        let new_value = value + amount;
        debug_assert!(new_value <= 0b0001_1111_1111_1111);
        self.set_spent_mana(value + amount)
    }

    #[inline(always)]
    pub fn dec_boss_hp(&self, amount: u8) -> Self {
        assert!(amount as u64 <= 0b1111111);
        let value = self.get_boss_hp();
        if value <= amount {
            return self.set_boss_hp(0);
        } else {
            return self.set_boss_hp(unsafe { value.unchecked_sub(amount) });
        }
    }
}

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
    fn try_cast_magic_missile(&self) -> Option<Game> {
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
    fn try_cast_drain(&self) -> Option<Game> {
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
    fn try_cast_recharge(&self) -> Option<Self> {
        const RECHARGE_COST: u16 = 229;
        const RECHARGE_TURNS: u8 = 5;

        (self.get_player_mana() >= RECHARGE_COST && self.get_recharge_effect_counter() == 0).then(
            || {
                self.spend_mana(RECHARGE_COST)
                    .set_recharge_effect_counter(RECHARGE_TURNS)
            },
        )
    }
    fn try_cast_poison(&self) -> Option<Self> {
        const POISON_COST: u16 = 173;
        const POISON_TURNS: u8 = 6;

        (self.get_player_mana() >= POISON_COST && self.get_poison_effect_counter() == 0).then(
            || {
                self.spend_mana(POISON_COST)
                    .set_poison_effect_counter(POISON_TURNS)
            },
        )
    }
    fn try_cast_shield(&self) -> Option<Self> {
        const SHIELD_COST: u16 = 113;
        const SHIELD_TURNS: u8 = 6;

        (self.get_player_mana() >= SHIELD_COST && self.get_shield_effect_counter() == 0).then(
            || {
                self.spend_mana(SHIELD_COST)
                    .set_shield_effect_counter(SHIELD_TURNS)
            },
        )
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
    fn map_maybe(&self, f: impl FnOnce(&BitState) -> Option<BitState>) -> Option<Self> {
        match self {
            Game::Playing(bit_state) => f(bit_state).map(Game::Playing),
            game => Some(*game),
        }
    }
}

#[tracing::instrument(skip(player, boss))]
fn least_mana_spent(player: Player, boss: Boss, player_hp_per_move: u8) -> usize {
    let mut heap = BinaryHeap::new();

    let state = BitState(0)
        .set_player_hp(player.hp as u8)
        .set_player_mana(player.mana as u16)
        .set_boss_hp(boss.hp as u8)
        .set_boss_damage(boss.damage as u8);

    heap.push(state);

    let mut min_spent_mana_at_boss_death = u16::MAX;

    let mut visited = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.get_spent_mana() >= min_spent_mana_at_boss_death {
            continue;
        }
        if !visited.insert(state) {
            continue;
        }

        let state = state.dec_player_hp(player_hp_per_move);

        if state.get_player_hp() == 0 {
            continue;
        }

        // before player move we should apply effects
        let mut armor = 0;

        let game = state.apply_effects(&mut armor);

        match game
            .and_then_maybe(|s| s.try_cast_magic_missile())
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
            .and_then_maybe(|s| s.try_cast_drain())
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
            .map_maybe(|s| s.try_cast_recharge())
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
            .map_maybe(|s| s.try_cast_poison())
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
            .map_maybe(|s| s.try_cast_shield())
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
    }

    // 5 actions
    // effects
    min_spent_mana_at_boss_death as usize
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{least_mana_spent, BitState};

    #[rstest]
    #[case(10, 250, 13, 8, 0, 226)]
    #[case(10, 250, 14, 8, 0, 641)]
    #[case(50, 500, 71, 10, 0, 1824)]
    #[case(50, 500, 71, 10, 1, 1937)]
    fn test_part1(
        #[case] player_hp: usize,
        #[case] player_mana: usize,
        #[case] boss_hp: usize,
        #[case] boss_damage: usize,
        #[case] hp_per_move: u8,
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
                },
                hp_per_move
            )
        );
    }

    #[test]
    fn test_bit_state() {
        let mut state = BitState(0);
        //
        state = state.set_player_hp(3);
        assert_eq!(state.get_player_hp(), 3);
        state = state.set_player_hp(0);
        assert_eq!(state.get_player_hp(), 0);

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
