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

	pub fn add_transition(&mut self, id0: int, c: char, id1: int)
	{
		self.states[id0].add_transition(c, id1);
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
				for j in range(0, self.states[curr[i]].transitions.len())
				{
					let (t, id) = (self.states[curr[i]].transitions[j]).clone();
					if t == c
					{
						next.push(id);
					}
					else if t == '%'
					{
						curr.push(id);
						l += 1;
					}
				}
				i += 1;
			}
			if next.len() == 0 { return false; }
			return self.check(string.slice_from(1).to_owned(), next);
		}
		else // the NFA has eaten all of the characters
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
				for j in range(0, self.states[curr[i]].transitions.len())
				{
					let (t , id) = (self.states[curr[i]].transitions[j]).clone();
					if t == '%'
					{
						next.push(id);
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

#[deriving(Clone, Eq)]
struct State
{
	accept: bool,
	transitions: ~[(char, int)]
}

impl State
{
	pub fn new(b: bool)	->	State
	{
		State { accept: b, transitions: ~[] }
	}

	pub fn print(&self)
	{
		print(fmt!("matching=%? transitions={", self.accept));
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

	pub fn add_transition(&mut self, c: char, id: int )
	{
		self.transitions.push((c, id));
	}
}

fn generate(lang: &[char], len: uint)	->	~[~str]
{
	let mut set = ~[];
	if len == 0 { return ~[~""]; }
	if len == 1 
	{
		for i in range(0, lang.len())
		{
			set.push(lang[i].to_str());
		}
		return set;
	}
	for i in range(0, lang.len())
	{
		let s = generate(lang, len-1);
		for j in range(0, s.len())
		{
			set.push(lang[i].to_str() + s[j].clone());
		}
	}
	return set;
}

fn main()
{
	let test = std::os::args()[1];
	let lang = ~['a', 'b'];
	let len = 4u;
	let mut strings = ~[];
	for i in range(0, len+1)
	{
		let s = generate(lang, i);
		s.map(|x| { strings.push((x).clone()); x.clone() });
	}
	let mut nfa = NFA::new();
	let mut parenthesis: ~[int] = ~[];
	let mut j = -1;
	let mut escaped = false;
	let mut paren = false;
	let mut i = 1;
	while i < test.len()+1
	{
		let c = test.char_at(i-1);
		println(fmt!("on %c", c));
		if c == '\\'
		{
			escaped = true;
			paren = false;
		}
		else if c == '(' && !escaped
		{
			parenthesis.push(j+1);
			paren = false;
		}
		else if c == ')' && !escaped
		{
			if paren { parenthesis.pop(); }
			else { paren = true; }
		}
		else if c == '[' && !escaped
		{
			if paren { paren = false; }
		
			let mut inner = ~[];
			i += 1;
			while i < test.len()+1 && test.char_at(i-1) != ']'
			{
				inner.push(test.char_at(i-1));
				i += 1;
			}

			if inner.len() > 0
			{
				let state = State::new(false);
				nfa.add_state(state);
				j += 1;
				if inner[0] == '^'
				{
					let mut diff = lang.clone(); 
					diff.retain(|x| { !inner.contains(x) });
					diff.map(|x| { nfa.add_transition(j, *x, j+1); *x });
				}
				else { inner.map(|x| { nfa.add_transition(j, *x, j+1); *x }); }
			}
		}
		else if c == '*' && !escaped
		{
			let mut place = j;
			if paren
			{
				place = parenthesis.pop();
				paren = false;
			}

			// link curr to place
			for i in range(0, nfa.states[j].transitions.len())
			{
				let (a,_) = (nfa.states[j].transitions[i]).clone();
				if a != '%'
				{
					nfa.add_transition(j, a, place);	
				}
			}

			// add epsilon transition to skip this state
			nfa.add_transition(place, '%', j+1);

		}
		else if c == '+' && !escaped
		{
			let mut place = j;
			if paren
			{
				place = parenthesis.pop();
				paren = false;
			}
			
			for i in range(0, nfa.states[j].transitions.len())
			{
				let (a,_) = (nfa.states[j].transitions[i]).clone();
				if a != '%'
				{
					nfa.add_transition(j, a, place);	
				}
			}
		}
		else if c == '?' && !escaped
		{
			let mut place = j;
			if paren
			{
				place = parenthesis.pop();
				paren = false;
			}

			// add epsilon transition
			nfa.add_transition(place, '%', j+1);
		}
		else if c == '.' && !escaped
		{
			if paren
			{
				parenthesis.pop();
				paren = false;
			}
			let state = State::new(false);
			nfa.add_state(state);
			j += 1;

			// unconditional jump 
			for i in range(0, lang.len())
			{
				nfa.add_transition(j, lang[i], j+1);
			}
		}
		else
		{
			if paren
			{
				parenthesis.pop();
				paren = false;
			}

			let state = State::new(false);
			nfa.add_state(state);
			j += 1;

			// link with next 
			nfa.add_transition(j, c, j+1);
			escaped = false;
		}

		nfa.print();
		i += 1;
	}
	// make the last state true 
	let state = State::new(true);
	nfa.add_state(state);
	
	nfa.print();

	let curr = ~[0];
	println(fmt!("Regex is: %s", test));
	for i in range(0, strings.len())
	{
		println(fmt!("%? %?", strings[i], nfa.check(strings[i].clone(), curr.clone())));
	}
	
}
