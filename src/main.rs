use rand::Rng;

fn main() {

    let winning_strategy_2 = Strategy::new(
        vec![
            StrategyElement(vec![0, 1]),
            StrategyElement(vec![0]),
            StrategyElement(vec![0, 1]),
        ]
    );

    let winning_strategy_4 = Strategy::new(
        vec![
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 2]), // D
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 1]), // T
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 2]), // D
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0]), // w
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 2]), // D
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 1]), // T
            StrategyElement(vec![0, 1, 2, 3]), // -1
            StrategyElement(vec![0, 2]), // D
            StrategyElement(vec![0, 1, 2, 3]), // -1
        ]
    );

    let n_trials = 100000;
    for i in (0..n_trials) {
        let table = set_up_table(4);

        let strategy = Strategy::new(
            vec![
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 2]), // D
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 1]), // T
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 2]), // D
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0]), // w
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 2]), // D
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 1]), // T
                StrategyElement(vec![0, 1, 2, 3]), // -1
                StrategyElement(vec![0, 2]), // D
                StrategyElement(vec![0, 1, 2, 3]), // -1
            ]
        );

        let applier = StrategyApplier::new(table, strategy);

        let result = test_strategy(applier);
        match result {
            Ok(n) => println!("Finished in {} steps!", n),
            Err(n) => panic!("Timed out after {} steps.", n),
        };
    }
}

fn set_up_table(n: usize) -> RotatingTable {
    let switches = (0..n).into_iter().map(|_x| Switch::random()).collect();
    return RotatingTable::new(switches);
}

fn test_strategy(mut applier: StrategyApplier) -> Result<usize, usize> {
    let timeout = 1000;
    for i in 0..1000 {
        if applier.is_on() {
            return Ok(i);
        }
        applier.apply();
    }
    return Err(timeout);
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Switch {
    On,
    Off,
}

impl Switch {

    pub fn new(state: bool) -> Self {
        match state {
            true => Self::On,
            false => Self::Off,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<bool>())
    }

    pub fn get_state(&self) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
        }
    }

    pub fn flip(&mut self) {
        *self = Self::new(!self.get_state())
    }
}


#[derive(Debug)]
pub struct RotatingTable {
    start_idx: usize,
    switches: Vec<Switch>,
}

impl RotatingTable {

    pub fn new(switches: Vec<Switch>) -> Self {
        Self {
            start_idx: 0,
            switches,
        }
    }

    pub fn len(&self) -> usize {
        self.switches.len()
    }

    pub fn get(&self, n: usize) -> bool {
        self.switches.get((self.start_idx + n) % self.len()).unwrap().get_state()
    }

    pub fn is_on(&self) -> bool {
        self.switches.iter().all(|x| x.get_state())
    }

    pub fn flip(&mut self, n: usize) {
        let idx = self.get_index(n);
        let mut switch = self.switches.get(idx).unwrap().clone();
        switch.flip();
        self.switches[idx] = switch;
    }

    pub fn rotate(&mut self, n: usize) {
        self.start_idx = (self.start_idx + n) % self.len()
    }

    pub fn rotate_randomly(&mut self) {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0..self.len());
        self.rotate(n)
    }

    pub fn apply_strategy(&mut self, mut strategy: Strategy) {
        let element = strategy.get_next();
        self.apply_strategy_element(element);
    }

    pub fn pretty_print(&self) -> String {
        let states: Vec<String> = self.switches.iter().map(|x| if x.get_state() {"1".to_string()} else {"0".to_string()}).collect();
        states.join("|")
    }

    fn apply_strategy_element(&mut self, strategy_element: StrategyElement) {
        let before = self.pretty_print();
        strategy_element.0.iter().for_each(|x| self.flip(*x));
        println!("{} ---{}---> {}", before, strategy_element.pretty_print(self.len()), self.pretty_print());
    }

    fn get_index(&self, n: usize) -> usize {
        (self.start_idx + n) % self.len()
    }
}

#[derive(Debug)]
pub struct Strategy {
    idx: usize,
    elements: Vec<StrategyElement>,
}

impl Strategy {
    
    pub fn new(elements: Vec<StrategyElement>) -> Self {
        Self {
            idx: 0,
            elements,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get_next(&mut self) -> StrategyElement {
        let idx = self.idx;
        self.idx = (self.idx + 1) % self.len();
        return self.elements.get(idx).unwrap().clone();
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct StrategyElement(Vec<usize>);

impl StrategyElement {

    pub fn pretty_print(&self, l: usize) -> String {
        let strings: Vec<String> = (0..l).into_iter().map(|x| if self.0.contains(&x) {"1".to_string()} else {"0".to_string()}).collect();
        return strings.join("|");
    }

}

#[derive(Debug)]
pub struct StrategyApplier {
    table: RotatingTable,
    strategy: Strategy,
}

impl StrategyApplier {

    pub fn new(table: RotatingTable, strategy: Strategy) -> Self {
        Self {
            table,
            strategy,
        }
    }

    pub fn is_on(&self) -> bool {
        self.table.is_on()
    }

    pub fn apply(&mut self) {
        let element = self.strategy.get_next();
        self.table.apply_strategy_element(element);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch() {

        let mut switch = Switch::On;
        assert!(switch.get_state());

        switch.flip();
        assert!(!switch.get_state());

        switch.flip();
        assert!(switch.get_state());

        let switch = Switch::random();
        assert!(switch.get_state() || !switch.get_state());

    }

    #[test]
    fn test_rotating_table() {
        let mut table = RotatingTable::new(vec![Switch::On, Switch::Off]);

        assert_eq!(table.len(), 2);

        assert!(!table.is_on());

        table.rotate(0);
        assert!(table.get(0));
        assert!(!table.get(1));

        table.rotate(1);
        assert!(!table.get(0));
        assert!(table.get(1));

        table.rotate(1);
        assert!(table.get(0));
        assert!(!table.get(1));

        table.rotate(12);
        assert!(table.get(0));
        assert!(!table.get(1));

        table.flip(1);
        assert!(table.get(0));
        assert!(table.get(1));

        table.rotate_randomly();
        assert!(table.get(0));
        assert!(table.get(1));

        assert!(table.is_on());

    }

    #[test]
    fn test_strategy() {
        
        let mut strategy = Strategy::new(
            vec![
                StrategyElement(vec![0, 1, 2, 3]),
                StrategyElement(vec![0, 2]),
                StrategyElement(vec![]),
            ]
        );

        let mut table = RotatingTable::new(
            vec![
                Switch::On,
                Switch::Off,
                Switch::On,
                Switch::Off,
            ]
        );

        let mut applier = StrategyApplier::new(table, strategy);

        assert!(!applier.is_on());

        applier.apply();
        applier.apply();
        applier.apply();

        assert!(applier.is_on());

        applier.apply();
        assert!(!applier.is_on());
        applier.apply();
        assert!(!applier.is_on());
        applier.apply();
        assert!(!applier.is_on());
        applier.apply();
        assert!(!applier.is_on());
        applier.apply();
        assert!(applier.is_on());
        applier.apply();
        assert!(applier.is_on());

    }


}