struct NFA
{
	states: ~[State],
}

impl NFA
{
	pub fn new()	->	NFA
	{
		NFA { states: ~[] }
	}

	pub fn add_state(&mut self, state: State)
	{
		self.states.push(state);
	}

	pub fn link_states(&mut self, id0: int, id1: int)
	{
		self.states[id0].add_transition(id1);
	}

	pub fn print(&self)
	{
		println("\nprinting states:");
		for i in range(0, self.states.len())
		{
			print(fmt!("%u: ", i));
			self.states[i].print();
		}
		println("");
	}

	pub fn pop(&mut self)	->	State
	{
		self.states.pop()
	}

	pub fn len(&self)	->	int
	{
		self.states.len() as int
	}

	pub fn check(&self, string: ~str, c: ~[int])		->	bool
	{
		let mut curr = c.clone();
		//println(fmt!("check: string=%? c=%?", string, c));
		if string.len() > 0
		{
			let c = string.char_at(0);
			let mut next = ~[];
			// look to see if any current states have c or an
			// epsilon transition as a value
			let mut l = curr.len();
			let mut i = 0;
			while i < l
			{
				//println(fmt!("on character %?, state=%? val=%?", c, curr[i], self.states[curr[i]].val));
				if self.states[curr[i]].val == c
				{
					for j in range(0, self.states[curr[i]].transitions.len())
					{
						//println(fmt!("adding %d", self.states[curr[i]].transitions[j]));
						next.push(self.states[curr[i]].transitions[j]);
					}
				} 
				else if self.states[curr[i]].val == '%'
				{
					for j in range(0, self.states[curr[i]].transitions.len())
					{
						//println(fmt!("pushing %d", self.states[curr[i]].transitions[j]));
						curr.push(self.states[curr[i]].transitions[j]);
						l += 1;
					}
				}
				i += 1;
				//println(fmt!("curr=%? next=%? l=%? i=%?", curr,next,l, i));
			}
			if next.len() == 0 { return false; }
			return self.check(string.slice_from(1).to_owned(), next);
		}
		else
		{ 	
			// see if we're in a matching state
			for i in range(0, curr.len())
			{
				if self.states[curr[i]].accept
				{ 
					return true;
				}
			}
			// check to see if there are any epsilon transitions we
			// can make
			let mut next = ~[];
			for i in range(0, curr.len())
			{
				if self.states[curr[i]].val == '%'
				{
					for j in range(0, self.states[curr[i]].transitions.len())
					{
						next.push(self.states[curr[i]].transitions[j]);
					}
				}
			}
			//println(fmt!("curr=%? next=%?", curr, next));
			if next.len() > 0
			{
				return self.check(string, next);
			}
			return false;
		}
	}
}

#[deriving(Clone,DeepClone, Eq)]
struct State
{
	val: char,
	accept: bool,
	transitions: ~[int]
}

impl State
{
	pub fn new(c: char, b: bool)	->	State
	{
		State { val: c, accept: b, transitions: ~[] }
	}

	pub fn print(&self)
	{
		print(fmt!("value=%? matching=%? transitions={", self.val, self.accept));
		for i in range(0, self.transitions.len())
		{
			print(fmt!("%?,", self.transitions[i]));
		}
		println("}");
	}

	pub fn is_accept(&mut self, b: bool)
	{
		self.accept = b;
	}

	pub fn add_transition(&mut self, id: int)
	{
		self.transitions.push(id);
	}

	pub fn change_val(&mut self, c: char)
	{
		self.val = c;
	}
}

fn main()
{
	let test = "abb*";
	let strings = ~[~"a", ~"b", ~"aa", ~"bb", ~"ab", ~"ba", ~"aab", ~"aba", ~"baa", ~"abb", ~"bab", ~"bba", ~"bbb", ~"aaa"];
	let mut nfa = NFA::new();
	let head_state = State::new('%', false);
	nfa.add_state(head_state);
	let mut j = 0;
	for i in range(1,(test.len()+1) as int)
	{
		let c = test.char_at((i-1) as uint);
		match c
		{
			'*'	=>	{
						// prev state j
						// state_in 
						// state_curr
						// state_out
						let mut state_in = nfa.pop();
						j -= 1;
						let c = state_in.val;
						let state_curr = State::new(c, false);
						state_in.change_val('%');
						let state_out = State::new('%', false);

						// don't need to link
						nfa.add_state(state_in);
						j += 1;
						// prev state 
						// state_in j
						// state_curr
						// state_out 

						// link to state_in
						nfa.add_state(state_curr);
						j += 1;
						// prev state 
						// state_in
						// state_curr j
						// state_out 
						nfa.link_states(j-1, j);

						// link to state_curr
						nfa.add_state(state_out);
						j += 1;
						// prev state 
						// state_in
						// state_curr
						// state_out j
						nfa.link_states(j-1, j);

						// link prev state to state_out
						nfa.link_states(j-3, j);

						// link state_out to state_in,
						// since it's a *
						nfa.link_states(j, j-2);
					}
			'?'	=>	{
						let mut state_in = nfa.pop();
						j -= 1;
						let c = state_in.val;
						let state_curr = State::new(c, false);
						state_in.change_val('%');
						let state_out = State::new('%', false);

						// don't need to link
						nfa.add_state(state_in);
						j += 1;

						// link to state_in
						nfa.add_state(state_curr);
						j += 1;
						nfa.link_states(j-1, j);

						// link to state_curr
						nfa.add_state(state_out);
						j += 1;
						nfa.link_states(j-1, j);

						// link prev state to state_out
						nfa.link_states(j-3, j);

						// link state_in to state_out,
						// since it's a ?
						nfa.link_states(j-2, j);
					}
			_	=>	{
						let state = State::new(c, false);
						nfa.add_state(state);
						j += 1;
						// link with previous
						nfa.link_states(j-1, j);
					}
		};

		//println("");
		//nfa.print();
		//println("");
	}
	
	// add a tail state
	let tail_state = State::new('%', true);	
	nfa.add_state(tail_state);
	// link with previous
	let l = nfa.len();
	nfa.link_states(l-2, l-1);

	nfa.print();

	let curr = ~[0];
	println(fmt!("Regex is: %s", test));
	for i in range(0, strings.len())
	{
		println(fmt!("%? %?", strings[i], nfa.check(strings[i].clone(), curr.clone())));
	}
}
