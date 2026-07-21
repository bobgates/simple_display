
#[derive(Copy, Clone)]


pub struct Stack{
    x: f64,
    y: f64,
    z: f64,
    t: f64,
    changed: bool,
    // numbers: Vec<f64, 18>,
    index: usize,
}

const NUMBERS_A: &'static [f64] = &[
    123456789.0,
    12345678.9,
    1234567.89,
    123456.789,
    12345.6789,
    1234.56789,
    123.456789,
    12.3456789,
    1.23456789,
    0.123456789,
    0.0123456789,
    0.00123456789,
    0.000123456789,
    0.0000123456789,
    0.00000123456789,
    0.000000123456789,
    0.0000000123456789,
    0.00000000123456789,
    0.0,
];

const NUMBERS_B: &'static [f64] = &[
    10000000000.1,
    1000000000.1,
    10000000.001,
    1000000.0001,
    100000.00001,
    10000.0000,
    1000.0001,
    100.000001,
    10.0000001,
    1.00000001,
    0.100000001,
    0.0100000001,
    0.00100000001,
    0.000100000001,
    0.0000100000001,
    0.00000100000001,
    0.000000100000001,
    0.000000010000010,
    0.0,
];


impl Stack {
    pub fn new()-> Stack{
    
        Stack { x: 0.0, y: 1.0, z: 5.0e20, t: 08e-6, changed: false, index: 0}
    }

    pub fn test_increment(&mut self){
        let delta: f64 = 0.000000000000001;
        self.x = NUMBERS_A[self.index];
        self.y = 9.0 as f64 + delta;
        self.z = 2.0 as f64 + delta;
        self.t = NUMBERS_B[self.index]; // currenlty routed to .y
        self.index = (self.index + 1) % NUMBERS_A.len();

    }

    pub fn push(&mut self, x: f64) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = x;
        // self.x = entry;   /
        self.changed = true;
        // Leaves x in y and in x
    }

    // Pops and returns bottom, x, value
    pub fn pop(&mut self)-> f64 {
        let temp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.changed = true;
        // Leaves a in a and in z
        temp
    }
    pub fn set_changed(&mut self) {
        self.changed = true;
    }
    pub fn changed(&mut self)->bool{
        self.changed
    }
    
    pub fn fetch_values(&mut self) -> (f64, f64, f64, f64){
        (self.x, self.y, self.z, self.t)
    }

    pub fn swapxy(&mut self){
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }


    pub fn swapx_with_new_y(&mut self, new_y: f64){
        self.x = self.y;
        self.y = new_y;
    }


    pub fn get_x(&mut self)->f64{
        return self.x;
    }
    
    pub fn get_y(&mut self)->f64{
        return self.y;
    }


}