use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{env, fmt};

pub use self::Thing::*;

macro_rules! set {
    () => ( ::std::collections::BTreeSet::new() );
    ($($x:expr),*) => ({
        let mut set = ::std::collections::BTreeSet::new();
        $(set.insert($x);)*
        set
    });
    ($($x:expr,)*) => ( set![$($x),*] )
}

/// Things that can be moved around
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Thing {
    Generator(&'static str),
    Microchip(&'static str),
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Generator(id) => f.write_fmt(format_args!("{}G", id)),
            &Microchip(id) => f.write_fmt(format_args!("{}M", id)),
        }
    }
}

/// Current arrangement of things
#[derive(PartialEq, Eq, Clone)]
pub struct State {
    /// Set of things on each floor
    floors: Vec<BTreeSet<Thing>>,
    /// Position of elevator
    elevator: usize,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (floorno, things) in self.floors.iter().enumerate().rev() {
            try!(f.write_str("F")); try!(floorno.fmt(f)); try!(f.write_str(": "));
            if self.elevator == floorno { try!(f.write_str("(E) ")) }
            for (i, thing) in things.iter().enumerate() {
                if i > 0 { try!(f.write_str(", ")) }
                try!(thing.fmt(f));
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("{"));
        for (floorno, things) in self.floors.iter().enumerate() {
            if self.elevator == floorno { try!(f.write_str("*")) }
            try!(floorno.fmt(f)); try!(f.write_str(":"));
            for (i, thing) in things.iter().enumerate() {
                if i > 0 { try!(f.write_str(",")) }
                try!(f.write_fmt(format_args!("{}", thing)));
            }
            try!(f.write_str(" "));
        }
        try!(f.write_str("}"));
        Ok(())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (floorno, things) in self.floors.iter().enumerate() {
            floorno.hash(state);
            let mut things: Vec<Thing> = things.iter().cloned().collect();
            things.sort();
            things.hash(state);
        }
        self.elevator.hash(state);
    }
}

impl State {
    /// Create a state with the given things on each floor
    fn new(floors: Vec<BTreeSet<Thing>>) -> State {
        State { floors: floors, elevator: 0 }
    }

    /// Check if the current arrangement is valid (i.e. no microchip
    /// above the first floor is exposed to a genreator without being
    /// protected)
    fn is_valid(&self) -> bool {
        // on each floor above first..
        for things in &self.floors[1..] {
            for thing in things {
                // if there is a microchip..
                if let &Microchip(e) = thing {
                    // without protection (a matching generator)..
                    if !things.iter().any(|t| t == &Generator(e)) &&
                        // exposed to a (non-matching) generator..
                        things.iter().any(|t| match t { &Generator(ee) if ee != e => true, _ => false })
                    {
                        // ..it'll fry (making this state invalid)
                        return false;
                    }
                }
            }
        }
        // valid only, if elevator position is on an existing floor
        self.elevator < self.floors.len()
    }

    /// Check if done (all things moved to top floor)
    fn is_done(&self) -> bool {
        for things in &self.floors[..self.floors.len()-1] {
            if things.len() > 0 {
                return false;
            }
        }
        true
    }

    /// Iterate over next possible states
    /// FIXME: Should create and return an iterator instead
    fn next_states<F: FnMut(State)>(&self, mut f: F) {
        for &down in [false, true].iter() {
            if down && self.elevator == 0 || !down && self.elevator + 1 >= self.floors.len() { continue; }
            let new_elevator = if down { self.elevator - 1 } else { self.elevator + 1 };
            let mut things = self.floors[self.elevator].iter();
            while let Some(thing1) = things.next() {
                for thing2 in [None].iter().cloned().chain(things.clone().map(|t| Some(t)) ) {
                    let mut new_floors = self.floors.clone();
                    assert!(new_floors[self.elevator].remove(thing1));
                    assert!(new_floors[new_elevator].insert(thing1.clone()));
                    if let Some(thing2) = thing2 {
                        assert!(new_floors[self.elevator].remove(thing2));
                        assert!(new_floors[new_elevator].insert(thing2.clone()));
                    }
                    let new_state = State { floors: new_floors, elevator: new_elevator };
                    if new_state.is_valid() {
                        f(new_state);
                    }
                }
            }
        }
    }

    /// Calculates minimum steps to move all things to the top floor
    fn min_steps(&self) -> usize {
        let mut states = vec![self.clone()];
        let mut depth = 1;
        let mut seen = BTreeSet::new();
        loop {
            // println!("Searching at depth {} ({} states, {} seen)...", depth, states.len(), seen.len());
            let mut new_states = Vec::new();
            let mut done = false;
            for state in &states {
                state.next_states(|new_state| {
                    if !done {
                        if new_state.is_done() {
                            done = true;
                        }
                        let mut hasher = DefaultHasher::new();
                        new_state.hash(&mut hasher);
                        let new_state_hash = hasher.finish();
                        if !seen.contains(&new_state_hash) {
                            seen.insert(new_state_hash);
                            new_states.push(new_state);
                        }
                    }
                });
            }
            if done { return depth; }
            states = new_states;
            depth += 1;
        }
    }
}

fn main() {
    let mut state = State::new(vec![
        // The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip,
        // a promethium generator, a ruthenium generator, a ruthenium-compatible microchip,
        // a cobalt generator, and a cobalt-compatible microchip.
        set![Generator("Po"), Generator("Tm"), Microchip("Tm"), Generator("Pm"), Generator("Ru"), Microchip("Ru"), Generator("Co"), Microchip("Co")],
        // The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
        set![Microchip("Po"), Microchip("Pm")],
        // The third floor contains nothing relevant.
        set![],
        // The fourth floor contains nothing relevant.
        set![],
    ]);
    println!("Minimum number of steps: {}", state.min_steps());
    // Don't run the tedious part two on CI
    if !env::var("CI").is_ok() {
        state.floors[0].insert(Generator("El"));
        state.floors[0].insert(Microchip("El"));
        state.floors[0].insert(Generator("Di"));
        state.floors[0].insert(Microchip("Di"));
        println!("Minimum number of steps with extra parts: {}", state.min_steps());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving() {
        let state = State::new(vec![set![Microchip("H"), Microchip("L")], set![Generator("H")], set![Generator("L")], set![]]);
        assert!(state.is_valid());
        assert!(!state.is_done());
        assert_eq!(state.min_steps(), 11);
    }
}
