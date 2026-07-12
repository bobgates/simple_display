use heapless::Vec;

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

const NUMBERS: &'static [f64] = &[
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
    0.00000000123456789
];

impl Stack {
    pub fn new()-> Stack{
        

        Stack { x: 0.0, y: 0.0, z: 0.0, t: 0.0, changed: false, index: 0}
    }

    pub fn test_increment(&mut self){
        self.x = NUMBERS[self.index];
        self.y = 1.0;
        self.z = 2.0;
        self.t = 3.0;
        self.index = (self.index + 1) % NUMBERS.len();

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

    // pub fn fetch_strs(&mut self) -> (Vec<u8,64>, &str, &str){

    //     let y_str: Vec<u8,64> = number_to_string(self.y).unwrap().clone();
    //     // let y = string_to_number(y_str);

    // // let y_str = format!("{:e}", self.y).expect("failed to convert number_to_string ");
    // // let r = y_str.into_bytes();
   
        

    //     (y_str, "0.0", "0.0")
    // }



    // pub fn print(&mut self) {
    //     info!("  Y: {}   Z: {}   T: {}", self.y, self.z, self.t);
    // }

}