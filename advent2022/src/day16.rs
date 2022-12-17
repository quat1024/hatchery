use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
	label: [char; 2],
}

impl Node {
	fn start() -> Self {
		"AA".into()
	}
}

impl From<&str> for Node {
	fn from(value: &str) -> Self {
		let mut chars = value.chars().fuse();
		Node { label: [chars.next().unwrap_or('_'), chars.next().unwrap_or('_')] }
	}
}

impl Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&String::from_iter(self.label))
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct EdgeKey {
	pair: (Node, Node),
}

impl EdgeKey {
	fn new(a: Node, b: Node) -> EdgeKey {
		EdgeKey { pair: if a < b { (a, b) } else { (b, a) } }
	}

	fn contains(&self, x: Node) -> bool {
		self.pair.0 == x || self.pair.1 == x
	}
}

impl From<(Node, Node)> for EdgeKey {
	fn from(value: (Node, Node)) -> Self {
		EdgeKey::new(value.0, value.1)
	}
}

impl Display for EdgeKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{} -- {}", self.pair.0, self.pair.1))
	}
}

impl Debug for EdgeKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self}"))
	}
}

#[derive(Clone, Debug)]
struct Map {
	///It just so happens that the graph is undirected; i leverage this without checking.
	///Please do not put egg on my face by part 2!!
	edge_weights: HashMap<EdgeKey, usize>,
	power: HashMap<Node, isize>,
	walkability: HashMap<Node, HashMap<Node, isize>>,
	walkability_2: HashMap<EdgeKey, isize>,
}

impl From<&str> for Map {
	#[allow(clippy::cast_possible_wrap)]
	fn from(value: &str) -> Self {
		let mut edge_weights: HashMap<EdgeKey, usize> = HashMap::new();
		let mut power: HashMap<Node, isize> = HashMap::new();

		for line in value.lines() {
			let label: Node = line[6..=8].into();
			let rate = crate::tools::number_from_soup::<false, isize>(line).unwrap();
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

		//Optimize it!

		//Please do not bite me in the tail, rust hashmap nondeterminism
		//I'm pretty sure this algorithm always ends up producing the same map, there aren't any cases
		//where removing a node ends up giving some other node three edges making it not allowed to be removed, etc
		//
		//Find a node with power 0
		while let Some((&zero_weight_node, _)) = power.iter().find(|(&node, &power)| node != Node::start() && power == 0) {
			//Find all edges containing this node
			let adjacencies: Vec<_> = power.keys().filter(|&node| edge_weights.contains_key(&EdgeKey::new(zero_weight_node, *node))).collect();
			//If there are exactly two edges, this node is a glorified tunnel, basically
			if adjacencies.len() == 2 {
				let left = *adjacencies[0];
				let right = *adjacencies[1];
				let left_edge = EdgeKey::new(left, zero_weight_node);
				let right_edge = EdgeKey::new(right, zero_weight_node);
				let weight_sum = edge_weights[&left_edge] + edge_weights[&right_edge];

				//Remove the existing edges between me and my neighbors
				edge_weights.remove(&left_edge);
				edge_weights.remove(&right_edge);

				//Add a new edge between the neighbors
				edge_weights.insert(EdgeKey::new(left, right), weight_sum);

				//Remove myself from `power`
				power.remove(&zero_weight_node);
			}
		}

		//build walkability map
		let mut walkability = HashMap::<Node, HashMap<Node, isize>>::new();
		let mut walkability_2 = HashMap::<EdgeKey, isize>::new();
		for &src in power.keys() {
			//dijkstra's algorithm basically rote copied from wikipedia
			//1
			let mut unvisited_set: HashSet<Node> = power.keys().copied().collect();

			//2
			let mut best_dist: HashMap<Node, isize> = HashMap::new();
			best_dist.insert(src, 0);

			//3. for the current node, consider all of its unvisited neighbors
			let mut current_node = src;
			loop {
				for unvisited_neighbor in unvisited_set.iter().filter(|&x| {
					let real = EdgeKey::new(current_node, *x);
					edge_weights.contains_key(&real)
				}) {
					//3b. calculate their tentative distances through the current node
					let current_distance = best_dist[&current_node];
					let distance_through_me = current_distance + edge_weights[&EdgeKey::new(current_node, *unvisited_neighbor)] as isize;
					if best_dist.get(unvisited_neighbor).unwrap_or(&isize::MAX) > &distance_through_me {
						best_dist.insert(*unvisited_neighbor, distance_through_me);
					}
				}

				//4.
				unvisited_set.remove(&current_node);

				//uhhhh 6.
				if let Some((_, next)) = unvisited_set.iter().filter_map(|x| best_dist.get(x).zip(Some(x))).min_by_key(|t| t.0) {
					current_node = *next;
				} else {
					//5.
					break;
				}
			}

			walkability.insert(src, best_dist.clone());
			for (dst, cost) in &best_dist {
				walkability_2.insert(EdgeKey::new(src, *dst), *cost);
			}
		}

		Map { edge_weights, power, walkability, walkability_2 }
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("graph Map {\n")?;
		for (edgekey, cost) in &self.walkability_2 {
			if self.edge_weights.contains_key(edgekey) {
				f.write_fmt(format_args!("  {edgekey} [label=\"{cost}\"]\n"))?;
			} else {
				//f.write_fmt(format_args!("  {edgekey} [label=\"{cost}\", color=\"#999999\"]\n"))?;
				//https://cdn.discordapp.com/attachments/591703651496493088/894736565874618448/2.mp4
			}
		}
		for (node, power) in &self.power {
			f.write_fmt(format_args!("  {node} [label=\"{node} ({power})\"]\n"))?;
		}
		f.write_str("}")
	}
}

impl Map {
	#[allow(clippy::too_many_arguments)] //merry christmas to you too clippy
	fn wander_around_and_cause_problems(
		&self,
		src: Node,
		budget: isize,
		score: isize,
		flow_rate: isize,
		already_turned: &HashSet<Node>,
		allow_navigation: bool,
	) -> isize {
		let mut next = score;

		//If there's a valve unturned in this room, try turning it on.
		if budget >= 1 && !already_turned.contains(&src) && self.power[&src] > 0 {
			let mut new_turned = already_turned.clone();
			new_turned.insert(src);
			let flow_rate = flow_rate + self.power[&src];

			next = next.max(self.wander_around_and_cause_problems(src, budget - 1, score, flow_rate, &new_turned, true));
		}

		let mut walked = false;
		if allow_navigation {
			//Try navigation to all the rooms with unturned valves.
			for (dst, cost) in &self.walkability[&src] {
				if src == *dst || *cost + 1 > budget || already_turned.contains(dst) || self.power[dst] == 0 {
					continue;
				}
				walked = true;

				//I spent {cost} minutes moving, increment the score by the elapsed time
				let score = score + flow_rate * cost;
				next = next.max(self.wander_around_and_cause_problems(*dst, budget - *cost, score, flow_rate, already_turned, false));
			}

			//Wait.
			if !walked && budget > 1 {
				let score = score + flow_rate;

				next = next.max(self.wander_around_and_cause_problems(src, budget - 1, score, flow_rate, already_turned, false));
			}
		}

		next
	}
}

pub fn a(input: &str) -> impl Display {
	let map = Map::from(input);
	println!("{map}");
	//map.wander_around_and_cause_problems(Node::start(), 30, 0, 0, &HashSet::new(), true)
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
		assert_eq!(a(&input_as_string(16)).to_string(), "???");
		assert_eq!(b(&input_as_string(16)).to_string(), "x");
	}
}
