// Sparse vector class. Gives consecutive storage with non-consecutive
// indices. Space usage is on the order of num_elements + max_index --
// not designed for large amounts of sparsity.

pub struct SparseVec<T>
{
	// Storage for the indices and the values. Should be kept sorted
	// by index!
	vals: Vec<(usize, T)>,

	// This maps indices with their location in vals
	idx_map: Vec<Option<usize>>
}

impl<T> SparseVec<T>
{
	// Constructs a new, empty SparseVec
	pub fn new() -> SparseVec<T>
	{
		SparseVec { vals: Vec::new(), idx_map: Vec::new() }
	}

	// Expands the index map until it has the given size
	fn expand_idx_map(&mut self, new_size: usize)
	{
		while self.idx_map.len() < new_size
		{
			self.idx_map.push(None);
		}
	}

	// Grabs an immutable reference to the given element
	pub fn get(&self, idx: usize) -> Option<&T>
	{
		// Grab the index within vals, or return None if it
		// doesn't exist
		let v_idx = match self.idx_map.get(idx)
		{
			Some(&Some(i)) => i,
			_ => { return None }
		};

		// It's guaranteed to exist... so grab and peek inside the
		// tuple reference
		Some(& self.vals[v_idx].1)
	}

	// Grabs a mutable reference to the given element
	pub fn get_mut(&mut self, idx: usize) -> Option<&mut T>
	{
		// Grab the index within vals, or return None if it
		// doesn't exist
		let v_idx = match self.idx_map.get(idx)
		{
			Some(&Some(i)) => i,
			_ => { return None }
		};

		// It exists... grab the tuple reference from vals and return its value
		Some(&mut self.vals[v_idx].1)
	}

	// Adds an element at the given location. If the element already
	// exists, replace it instead.
	pub fn push_at(&mut self, idx: usize, val: T)
	{
		self.expand_idx_map(idx + 1);

		match self.vals.binary_search_by(|&(i, _)| i.cmp(&idx))
		{
			Ok(v_idx) => { self.vals[v_idx] = (idx, val); self.idx_map[idx] = Some(v_idx) },
			Err(v_idx) => { self.vals.insert(v_idx, (idx, val)); self.idx_map[idx] = Some(v_idx) }
		}
	}
}
