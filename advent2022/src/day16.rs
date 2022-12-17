use std::collections::HashMap;
use std::fmt::Debug;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Label {
	label: [char; 2],
}

impl Label {
	fn start() -> Self {
		"AA".into()
	}
}

impl From<&str> for Label {
	fn from(value: &str) -> Self {
		let mut chars = value.chars().fuse();
		Label { label: [chars.next().unwrap_or('_'), chars.next().unwrap_or('_')] }
	}
}

impl Debug for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

impl Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&String::from_iter(self.label))
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct EdgeKey {
	pair: (Label, Label),
}

impl From<(Label, Label)> for EdgeKey {
	fn from(value: (Label, Label)) -> Self {
		EdgeKey { pair: if value.0 < value.1 { value } else { (value.1, value.0) } }
	}
}

#[derive(Clone)]
struct Map {
	edge_weights: HashMap<EdgeKey, usize>,
	power: HashMap<Label, usize>,
}

impl Map {
	fn remove_zero_power_nodes(&mut self) {
		//Find a node with power 0
		while let Some((&zero_weight_node, _)) = self.power.iter().find(|(&node, &power)| node != Label::start() && power == 0) {
			//Find all edges containing this node (not a good algorithm lol)
			let adjacencies: Vec<_> = self.power.keys().filter(|&node| self.edge_weights.contains_key(&EdgeKey::from((zero_weight_node, *node)))).collect();
			//If there are exactly two edges, this node is a glorified tunnel, basically
			if adjacencies.len() == 2 {
				let left = *adjacencies[0];
				let right = *adjacencies[1];
				let left_edge = EdgeKey::from((left, zero_weight_node));
				let right_edge = EdgeKey::from((right, zero_weight_node));
				let weight_sum = self.edge_weights[&left_edge] + self.edge_weights[&right_edge];

				//Remove the existing edges between me and my neighbors
				self.edge_weights.remove(&left_edge);
				self.edge_weights.remove(&right_edge);

				//Add a new edge between the neighbors
				self.edge_weights.insert(EdgeKey::from((left, right)), weight_sum);

				//Remove myself from `power`
				self.power.remove(&zero_weight_node);
			}
		}
	}
}

impl From<&str> for Map {
	fn from(value: &str) -> Self {
		let mut edge_weights: HashMap<EdgeKey, usize> = HashMap::new();
		let mut power: HashMap<Label, usize> = HashMap::new();

		for line in value.lines() {
			let label: Label = line[6..=8].into();
			let rate = crate::tools::number_from_soup::<false, usize>(line).unwrap();
			power.insert(label, rate);

			//lol
			if let Some(rest) = line.split_once("; ").map(|x| x.1) {
				if let Some(rest) = rest.strip_prefix("tunnel leads to ").or_else(|| rest.strip_prefix("tunnels lead to ")) {
					if let Some(rest) = rest.strip_prefix("valve ").or_else(|| rest.strip_prefix("valves ")) {
						for out in rest.split(", ") {
							let edgekey = (label, out.into()).into();
							edge_weights.insert(edgekey, 1);
						}
					}
				}
			}
		}

		Map { edge_weights, power }
	}
}

impl Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("graph Map {\n")?;
		for (edgekey, cost) in &self.edge_weights {
			f.write_fmt(format_args!("  {} -- {} [label=\"{cost}\"]\n", edgekey.pair.0, edgekey.pair.1))?;
		}
		for (node, power) in &self.power {
			f.write_fmt(format_args!("  {node} [label=\"{node} ({power})\"]\n"))?;
		}
		f.write_str("}")
	}
}

pub fn a(input: &str) -> impl Display {
	let mut map = Map::from(input);
	map.remove_zero_power_nodes();
	
	
	"x"
}

pub fn b(input: &str) -> impl Display {
	"x"
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(16)).to_string(), "1651");
		assert_eq!(b(&test_input_as_string(16)).to_string(), "x");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(16)).to_string(), "real real");
		assert_eq!(b(&input_as_string(16)).to_string(), "x");
	}
}
