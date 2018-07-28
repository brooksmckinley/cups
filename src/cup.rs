use rand::Rng;

pub trait Cup {
    fn fill(&mut self, amount: f64);
    fn empty(&mut self, amount: f64);
    fn drop_cup(&mut self);
    fn break_cup(&mut self);
    
    // Fields
    fn name(&self) -> String;
    fn serial(&self) -> i32;
    fn color(&self) -> String;
    fn handle(&self) -> bool;
    fn max_fluid(&self) -> f64;
    fn current_fluid(&self) -> f64;
    fn is_broken(&self) -> bool;
}

pub struct Mug {
    name: String,
    serial_number: i32,
    color: String,
    handle: bool,
    max_fluid: f64,
    current_fluid: f64,
    is_broken: bool,
}

impl Mug {
    pub fn new(name: String, serial_number: i32, color: String, handle: bool, max_fluid: f64) -> Mug {
        Mug {
            name: name,
            serial_number: serial_number,
            color: color,
            handle: handle,
            max_fluid: max_fluid,
            current_fluid: 0.0,
            is_broken: false,
        }
    }
}

impl Cup for Mug {
    fn fill(&mut self, amount: f64) {
        if self.current_fluid + amount >= self.max_fluid {
            self.current_fluid = self.max_fluid;
        }
        else {
            self.current_fluid += amount;
        }
    }
    fn empty(&mut self, amount: f64) {
        if self.current_fluid - amount <= 0.0 {
            self.current_fluid = 0.0;
        }
        else {
            self.current_fluid -= amount;
        }
    }
    fn drop_cup(&mut self) {
        let mut rng = ::rand::thread_rng();
        let chance = rng.gen_range(1, 10);
        if chance < 3 {
            self.is_broken = true;
        }
    }
    fn break_cup(&mut self) {
        self.is_broken = true;
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn serial(&self) -> i32 {
        self.serial_number.clone()
    }
    fn color(&self) -> String {
        self.color.clone()
    }
    fn handle(&self) -> bool {
        self.handle.clone()
    }
    fn max_fluid(&self) -> f64 {
        self.max_fluid.clone()
    }
    fn current_fluid(&self) -> f64 {
        self.current_fluid.clone()
    }
    fn is_broken(&self) -> bool {
        self.is_broken
    }
}

pub struct Glass {
    name: String,
    serial_number: i32,
    color: String,
    handle: bool,
    max_fluid: f64,
    current_fluid: f64,
    is_broken: bool,
}

impl Glass {
    pub fn new(name: String, serial_number: i32, color: String, handle: bool, max_fluid: f64) -> Mug {
        Mug {
            name: name,
            serial_number: serial_number,
            color: color,
            handle: handle,
            max_fluid: max_fluid,
            current_fluid: 0.0,
            is_broken: false,
        }
    }
}

impl Cup for Glass {
    fn fill(&mut self, amount: f64) {
        if self.current_fluid + amount >= self.max_fluid {
            self.current_fluid = self.max_fluid;
        }
        else {
            self.current_fluid += amount;
        }
    }
    fn empty(&mut self, amount: f64) {
        if self.current_fluid - amount <= 0.0 {
            self.current_fluid = 0.0;
        }
        else {
            self.current_fluid -= amount;
        }
    }
    fn drop_cup(&mut self) {
        let mut rng = ::rand::thread_rng();
        let chance = rng.gen_range(1, 10);
        if chance < 9 {
            self.is_broken = true;
        }
    }
    fn break_cup(&mut self) {
        self.is_broken = true;
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    fn serial(&self) -> i32 {
        self.serial_number.clone()
    }
    fn color(&self) -> String {
        self.color.clone()
    }
    fn handle(&self) -> bool {
        self.handle.clone()
    }
    fn max_fluid(&self) -> f64 {
        self.max_fluid.clone()
    }
    fn current_fluid(&self) -> f64 {
        self.current_fluid.clone()
    }
    fn is_broken(&self) -> bool {
        self.is_broken
    }
}


pub struct Tumbler {
    name: String,
    serial_number: i32,
    color: String,
    handle: bool,
    max_fluid: f64,
    current_fluid: f64,
    is_broken: bool,
}

impl Tumbler {
    pub fn new(name: String, serial_number: i32, color: String, max_fluid: f64) -> Mug {
        Mug {
            name: name,
            serial_number: serial_number,
            color: color,
            handle: false,
            max_fluid: max_fluid,
            current_fluid: 0.0,
            is_broken: false,
        }
    }
}

impl Cup for Tumbler {
    fn fill(&mut self, amount: f64) {
        if self.current_fluid + amount >= self.max_fluid {
            self.current_fluid = self.max_fluid;
        }
        else {
            self.current_fluid += amount;
        }
        self.current_fluid -= self.current_fluid * 0.2;
    }
    fn empty(&mut self, amount: f64) {
        if self.current_fluid - amount <= 0.0 {
            self.current_fluid = 0.0;
        }
        else {
            self.current_fluid -= amount;
        }
    }
    fn drop_cup(&mut self) {
        // Tumblers don't break when dropped
    }
    fn break_cup(&mut self) {
        self.is_broken = true;
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    fn serial(&self) -> i32 {
        self.serial_number.clone()
    }
    fn color(&self) -> String {
        self.color.clone()
    }
    fn handle(&self) -> bool {
        self.handle.clone()
    }
    fn max_fluid(&self) -> f64 {
        self.max_fluid.clone()
    }
    fn current_fluid(&self) -> f64 {
        self.current_fluid.clone()
    }
    fn is_broken(&self) -> bool {
        self.is_broken.clone()
    }
}


