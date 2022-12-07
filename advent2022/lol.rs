//! I wrote my own PeekingIterator adapter then found Iterator.peekable() in Rust immediately after.
//! I'll include it here for entertainment purposes

struct PeekingIterator<ITEM, ITER: Iterator<Item = ITEM>> {
	iterator: ITER,
	coming_up: Option<ITEM>,
}

impl<ITEM, ITER: Iterator<Item = ITEM>> PeekingIterator<ITEM, ITER> {
	fn new(iter: ITER) -> PeekingIterator<ITEM, ITER> {
		PeekingIterator { iterator: iter, coming_up: None }
	}
	
	fn peek(&mut self) -> &Option<ITEM> {
		if self.coming_up.is_none() {
			self.coming_up = self.iterator.next();
		}
		return &self.coming_up;
	}
}

impl<ITEM, ITER: Iterator<Item = ITEM>> Iterator for PeekingIterator<ITEM, ITER> {
	type Item = ITEM;

	fn next(&mut self) -> Option<Self::Item> {
		//if there's something in the delay slot, clear the slot and return it
		if self.coming_up.is_some() {
			self.coming_up.take()
		} else {
			self.iterator.next()
		}
	}
}