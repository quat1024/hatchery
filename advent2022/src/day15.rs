use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
	x: isize,
	y: isize,
}

impl Coord {
	#[allow(clippy::cast_possible_wrap)]
	fn manhattan_distance(self, other: Coord) -> isize {
		(self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
	}

	fn tuning_frequency(self) -> isize {
		self.x * 4_000_000 + self.y
	}
}

impl From<(isize, isize)> for Coord {
	fn from(value: (isize, isize)) -> Self {
		Coord { x: value.0, y: value.1 }
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Sensor {
	sensor: Coord,
	closest_beacon: Coord,
	distance_to_closest_beacon: isize,
}

impl Sensor {
	fn parse(line: &str) -> Sensor {
		let numbers = numbers_from_soup_2::<isize>(line);
		let sensor = (numbers[0], numbers[1]).into();
		let closest_beacon = (numbers[2], numbers[3]).into();
		Sensor { sensor, closest_beacon, distance_to_closest_beacon: sensor.manhattan_distance(closest_beacon) }
	}
}

pub fn a(input: &str) -> impl Display {
	let sensors: Vec<_> = input.lines().map(Sensor::parse).collect();

	//we're interested in beacons along the y=2_000_000 line that aren't the closest ones to the cursor.
	//Where along the line do we *start* scanning? At some point if we continue off left enough, the closest
	//sensor to us will not change. Same for the right. I thiiiiink adding and subtracting the distance to their
	//associated beacon is a good estimate of the limit of their "sphere of influence" ?
	let min_x = sensors.iter().map(|s| s.sensor.x - s.distance_to_closest_beacon).min().unwrap();
	let max_x = sensors.iter().map(|s| s.sensor.x + s.distance_to_closest_beacon).max().unwrap();

	//hack to get it to work in the test input lol
	let y = if min_x == -8 { 10 } else { 2_000_000 };

	let mut answer = 0;
	for x in min_x..=max_x {
		let cursor: Coord = (x, y).into();
		//println!("examining {cursor:?}");

		//Examine each sensor, see if the cursor is within anyone's sphere of influence.
		//(and also that it's not a beacon we know about)
		if sensors.iter().any(|s| cursor.manhattan_distance(s.sensor) <= s.distance_to_closest_beacon) && sensors.iter().all(|s| cursor != s.closest_beacon) {
			answer += 1;
		}
	}

	answer
}

pub fn b(input: &str) -> impl Display {
	let sensors: Vec<_> = input.lines().map(Sensor::parse).collect();
	
	let range = if sensors.iter().map(|s| s.sensor.x - s.distance_to_closest_beacon).min().unwrap() == -8 {
		//demo mode
		0..=20
	} else {
		0..=4_000_000
	};

	for s in &sensors {
		println!("checking {s:?}");
		let border_dist = s.distance_to_closest_beacon + 1;
		
		let mut border = Vec::<Coord>::new();
		
		let bottom_stop: Coord = (s.sensor.x, s.sensor.y + border_dist).into();
		let right_stop: Coord = (s.sensor.x + border_dist, s.sensor.y).into();
		let top_stop: Coord = (s.sensor.x, s.sensor.y - border_dist).into();
		let left_stop: Coord = (s.sensor.x - border_dist, s.sensor.y).into();
		let mut cursor: Coord = bottom_stop;
		while cursor != right_stop {
			border.push(cursor);
			cursor.x += 1;
			cursor.y -= 1;
		}
		while cursor != top_stop {
			border.push(cursor);
			cursor.x -= 1;
			cursor.y -= 1;
		}
		while cursor != left_stop {
			border.push(cursor);
			cursor.x -= 1;
			cursor.y += 1;
		}
		while cursor != bottom_stop {
			border.push(cursor);
			cursor.x += 1;
			cursor.y += 1;
		}
		border.push(cursor);
		
		border.retain(|c| range.contains(&c.x) && range.contains(&c.y));
		
		for coord in border {
			if sensors.iter().any(|s| coord.manhattan_distance(s.sensor) <= s.distance_to_closest_beacon) {
				//nope
			} else {
   			return coord.tuning_frequency().to_string();
   		}
		}
	}

	"couldnt find it".to_string()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(a(&test_input_as_string(15)).to_string(), "26");
		assert_eq!(b(&test_input_as_string(15)).to_string(), "56000011");
	}

	#[test]
	fn real() {
		assert_eq!(a(&input_as_string(15)).to_string(), "5525847");
		assert_eq!(b(&input_as_string(15)).to_string(), "13340867187704");
	}
}
