use std::collections::HashMap;

use crate::*;

//tagging these with their "index" so i don't get mixed up
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Inventory {
	ore0: usize,
	clay1: usize,
	obby2: usize,
	geode3: usize,
	ore0bot: usize,
	clay1bot: usize,
	obby2bot: usize,
	geode3bot: usize,
}

impl Default for Inventory {
	fn default() -> Self {
		Self { ore0: 0, clay1: 0, obby2: 0, geode3: 0, ore0bot: 1, clay1bot: 0, obby2bot: 0, geode3bot: 0 }
	}
}

impl Inventory {
	fn tick(&mut self) {
		self.ore0 += self.ore0bot;
		self.clay1 += self.clay1bot;
		self.obby2 += self.obby2bot;
		self.geode3 += self.geode3bot;
	}

	fn try_buy_orebot(self, bp: &Blueprint) -> Option<Inventory> {
		(self.ore0 >= bp.orebot_ore0_cost).then(|| {
			let mut result = self;
			result.ore0 -= bp.orebot_ore0_cost;
			result.ore0bot += 1;
			result
		})
	}

	fn try_buy_claybot(self, bp: &Blueprint) -> Option<Inventory> {
		(self.ore0 >= bp.claybot_ore0_cost).then(|| {
			let mut result = self;
			result.ore0 -= bp.claybot_ore0_cost;
			result.clay1bot += 1;
			result
		})
	}

	fn try_buy_obbybot(self, bp: &Blueprint) -> Option<Inventory> {
		(self.ore0 >= bp.obbybot_ore0_cost && self.clay1 >= bp.obbybot_clay1_cost).then(|| {
			let mut result = self;
			result.ore0 -= bp.obbybot_ore0_cost;
			result.clay1 -= bp.obbybot_clay1_cost;
			result.obby2bot += 1;
			result
		})
	}

	fn try_buy_geodebot(self, bp: &Blueprint) -> Option<Inventory> {
		(self.ore0 >= bp.geodebot_ore0_cost && self.obby2 >= bp.geodebot_obby2_cost).then(|| {
			let mut result = self;
			result.ore0 -= bp.geodebot_ore0_cost;
			result.obby2 -= bp.geodebot_obby2_cost;
			result.geode3bot += 1;
			result
		})
	}
}

struct Blueprint {
	id: usize,
	orebot_ore0_cost: usize,
	claybot_ore0_cost: usize,
	obbybot_ore0_cost: usize,
	obbybot_clay1_cost: usize,
	geodebot_ore0_cost: usize,
	geodebot_obby2_cost: usize,
}

impl From<&str> for Blueprint {
	fn from(value: &str) -> Self {
		let numbers = numbers_from_soup::<false, usize>(value);
		Blueprint {
			id: numbers[0],
			orebot_ore0_cost: numbers[1],
			claybot_ore0_cost: numbers[2],
			obbybot_ore0_cost: numbers[3],
			obbybot_clay1_cost: numbers[4],
			geodebot_ore0_cost: numbers[5],
			geodebot_obby2_cost: numbers[6],
		}
	}
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Action {
	Wait,
	BuyOre0bot,
	BuyClay1bot,
	BuyObby2Bot,
	BuyGeode3Bot,
}

impl Blueprint {
	fn evaluate(&self) -> usize {
		self.evaluate_recur(1, Inventory::default(), &mut HashMap::default())
	}

	fn evaluate_recur(&self, minute: usize, mut inventory: Inventory, dp_cache: &mut HashMap<(Inventory, usize), Action>) -> usize {
		inventory.tick();
		
		if minute == 25 {
			return inventory.geode3;
		}
		
		//Have we seen this situation before?
		let state = (inventory, minute);
		if let Some(known) = dp_cache.get(&state) {
			match known {
    Action::Wait => todo!(),
    Action::BuyOre0bot => todo!(),
    Action::BuyClay1bot => todo!(),
    Action::BuyObby2Bot => todo!(),
    Action::BuyGeode3Bot => todo!(),
}
		}
		
		//try doing nothing
		let mut best_score = self.evaluate_recur(minute + 1, inventory, dp_cache);

		//try each action
		if let Some(i) = inventory.try_buy_orebot(self) {
			best_score = best_score.max(self.evaluate_recur(minute + 1, i, dp_cache));
		}
		if let Some(i) = inventory.try_buy_claybot(self) {
			best_score = best_score.max(self.evaluate_recur(minute + 1, i, dp_cache));
		}
		if let Some(i) = inventory.try_buy_obbybot(self) {
			best_score = best_score.max(self.evaluate_recur(minute + 1, i, dp_cache));
		}
		if let Some(i) = inventory.try_buy_geodebot(self) {
			best_score = best_score.max(self.evaluate_recur(minute + 1, i, dp_cache));
		}
		
		best_score
	}

	fn quality(&self) -> usize {
		self.evaluate() * self.id
	}
}

pub fn a(input: &str) -> impl Display {
	input.lines().map(Blueprint::from).map(|x| x.quality()).sum::<usize>()
}

pub fn b(input: &str) -> impl Display {
	""
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(19)).to_string(), "33");
		//assert_eq!(b(&test_input_as_string(19)).to_string(), "");
	}

	#[test]
	fn real() {
		//assert_eq!(a(&input_as_string(19)).to_string(), "");
		//assert_eq!(b(&input_as_string(19)).to_string(), "");
	}
}
