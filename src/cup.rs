use rand::Rng;

pub trait Cup {
    fn fill(&mut self, amount: f64) {
        if self.current_fluid() + amount >= self.max_fluid() {
            let max_fluid = self.max_fluid();
            self.set_current_fluid(max_fluid);
        }
        else {
            let current_fluid = self.current_fluid();
            self.set_current_fluid(current_fluid + amount);
        }
    }
    fn empty(&mut self, amount: f64) {
        if self.current_fluid() - amount <= 0.0 {
            self.set_current_fluid(0.0);
        }
        else {
            let current_fluid = self.current_fluid();
            self.set_current_fluid(current_fluid - amount);
        }
    }
    fn drop_cup(&mut self) {
        let mut rng = ::rand::thread_rng();
        let chance = rng.gen_range(1, 10);
        if chance < 3 {
            self.set_is_broken(true);
        }
    }
    fn break_cup(&mut self) {
        self.set_is_broken(true);
    }

    
    // Getters
    fn name(&self) -> String;
    fn serial(&self) -> i32;
    fn color(&self) -> String;
    fn handle(&self) -> bool;
    fn max_fluid(&self) -> f64;
    fn current_fluid(&self) -> f64;
    fn is_broken(&self) -> bool;

    // Setters
    fn set_name(&mut self, name: String);
    fn set_serial(&mut self, serial: i32);
    fn set_color(&mut self, color: String);
    fn set_max_fluid(&mut self, max_fluid: f64);
    fn set_current_fluid(&mut self, current_fluid: f64);
    fn set_is_broken(&mut self, is_broken: bool);
}

// Setter/Getter macro
macro_rules! cup_sg {
    () => {
        fn name(&self) -> String {
            self.value_name.clone()
        }
        fn serial(&self) -> i32 {
            self.value_serial.clone()
        }
        fn color(&self) -> String {
            self.value_color.clone()
        }
        fn handle(&self) -> bool {
            self.value_handle.clone()
        }
        fn max_fluid(&self) -> f64 {
            self.value_max_fluid.clone()
        }
        fn current_fluid(&self) -> f64 {
            self.value_current_fluid.clone()
        }
        fn is_broken(&self) -> bool {
            self.value_is_broken.clone()
        }
        fn set_name(&mut self, name: String) {
            self.value_name = name;
        }
        fn set_serial(&mut self, serial: i32) {
            self.value_serial = serial;
        }
        fn set_color(&mut self, color: String) {
            self.value_color = color;
        }
        fn set_max_fluid(&mut self, max_fluid: f64) {
            self.value_max_fluid = max_fluid;
        }
        fn set_current_fluid(&mut self, current_fluid: f64) {
            self.value_current_fluid = current_fluid;
        }
        fn set_is_broken(&mut self, is_broken: bool) {
            self.value_is_broken = is_broken;
        }
    };
}

macro_rules! cup {
    ( $x:ident ) => {
        pub struct $x {
            value_name: String,
            value_serial: i32,
            value_color: String,
            value_handle: bool,
            value_max_fluid: f64,
            value_current_fluid: f64,
            value_is_broken: bool,
        }
    };
}

cup!(Mug);

impl Mug {
    pub fn new(name: String, serial_number: i32, color: String, handle: bool, max_fluid: f64) -> Mug {
        Mug {
            value_name: name,
            value_serial: serial_number,
            value_color: color,
            value_handle: handle,
            value_max_fluid: max_fluid,
            value_current_fluid: 0.0,
            value_is_broken: false,
        }
    }
}

impl Cup for Mug {
    cup_sg!();
}

cup!(Glass);

impl Glass {
    pub fn new(name: String, serial_number: i32, color: String, handle: bool, max_fluid: f64) -> Mug {
        Mug {
            value_name: name,
            value_serial: serial_number,
            value_color: color,
            value_handle: handle,
            value_max_fluid: max_fluid,
            value_current_fluid: 0.0,
            value_is_broken: false,
        }
    }
}

impl Cup for Glass {
    fn drop_cup(&mut self) {
        let mut rng = ::rand::thread_rng();
        let chance = rng.gen_range(1, 10);
        // Glasses have an 80% chance of breaking when dropped
        if chance < 9 {
            self.set_is_broken(true);
        }
    }
    cup_sg!();
}

cup!(Tumbler);

impl Tumbler {
    pub fn new(name: String, serial_number: i32, color: String, max_fluid: f64) -> Mug {
        Mug {
            value_name: name,
            value_serial: serial_number,
            value_color: color,
            value_handle: false,
            value_max_fluid: max_fluid,
            value_current_fluid: 0.0,
            value_is_broken: false,
        }
    }
}

impl Cup for Tumbler {
    fn fill(&mut self, amount: f64) {
        if self.current_fluid() + amount >= self.max_fluid() {
            let max_fluid = self.max_fluid();
            self.set_current_fluid(max_fluid);
        }
        else {
            let current_fluid = self.current_fluid();
            self.set_current_fluid(current_fluid + amount);
        }
        // Tumblers leak
        let current_fluid = self.current_fluid();
        self.set_current_fluid(current_fluid - (current_fluid * 0.2));
    }
    fn empty(&mut self, amount: f64) {
        if self.current_fluid() - amount <= 0.0 {
            self.set_current_fluid(0.0);
        }
        else {
            let current_fluid = self.current_fluid();
            self.set_current_fluid(current_fluid - amount);
        }
        // Tumblers leak
        let current_fluid = self.current_fluid();
        self.set_current_fluid(current_fluid - (current_fluid * 0.2));
    }
    fn drop_cup(&mut self) {
        // Tumblers don't break when dropped
    }
    cup_sg!();
}
