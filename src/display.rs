
use cortex_m::asm::delay;
use defmt::info;
use core::{f64, num};
use display_interface_spi::SPIInterface;

use embassy_time::Delay;


// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*};
use embedded_graphics::text::Text;

use heapless::{format, String};
use heapless::string::StringInner;

use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;


use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{SPI0};

use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use crate::stack;
use num_traits::float::FloatCore;


const NAME_LEFT: i32 = 1;
const COLON_LEFT: i32 = 6;
const NUM_LEFT: i32 = 15; 
const LINE_SPACING: i32 = 15;
const X_NUM_BOTTOM: i32 = 62;
const Y_NUM_BOTTOM: i32 = X_NUM_BOTTOM - LINE_SPACING;
const Z_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 2*LINE_SPACING;
const T_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 3*LINE_SPACING;
const X_LABEL_BOTTOM: i32 = 59;
const Y_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - LINE_SPACING;
const Z_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 2*LINE_SPACING;
const T_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 3*LINE_SPACING;

enum DisplayStyle{
    E(i32),
    S(i32),
    FIXED,
    ALL,
}


pub struct DisplayStruct <'a>{
    pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    reset_pin: Output<'a>,
    font: MonoTextStyle<'a, BinaryColor>,
    stack_names_font: MonoTextStyle<'a, BinaryColor>,
    stack: stack::Stack,
    number_style: DisplayStyle,
}

impl <'a> DisplayStruct <'a>{
    pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
                mut reset_pin: Output<'a>, font: MonoTextStyle<'a, BinaryColor>,
                stack_names_font: MonoTextStyle<'a, BinaryColor>
            ) -> Self {
        
        display.reset(&mut reset_pin, &mut Delay).unwrap();

        let stack = stack::Stack::new();
        let number_style = DisplayStyle::E(4);


        Self { 
            display, 
            reset_pin,
            font,
            stack,
            stack_names_font,
            number_style,
        }
    }

    pub fn num_to_string(&self, number: f64 )->String<20>{
        if number == 0.0 {
            let mut output: String<20>=format!("").unwrap();
            let _ = output.push('0');
            let _ = output.push('.');
            match self.number_style {
                DisplayStyle::E(sf) => {
                    for _ in 0..sf {
                        let _ = output.push('0');
                    }
                },
                _ => { let _ = output.push('X');}
            }
            let _ = output.push('E');
            let _ = output.push('0');
            return output;
        } else {

        // let mut n;
            match self.number_style {
                DisplayStyle::E(sf) => {
                    let exponent: i32 = 1 + libm::log10(number).floor() as i32;

                    let mut before_dp = exponent % 3;  // This gives everything powers for 10^3, 10^-3, etc

                    if before_dp ==0 {before_dp=3}; 
                    if before_dp<0 {
                        before_dp=3+before_dp
                    };
                    let exp = exponent - before_dp;

                    let n = (number/(10.0_f64).powi(exponent-sf)).trunc()/10_f64.powi(sf-before_dp);
                    // info!("n {}", n);
                    info!("output:{} exp{}", n, exp);

                    // 1. the cutting off of the number to the correct number of significant figures
                    // Leaves exp
                    // if exp == 0

// sf here is the number of significant figures to display, 
// but it is being interpreted as the number of decimal places 
// so we need to fix this the number accordingly

                    let num_str: String<20> =format!("{:.*}", sf as usize, n).unwrap();
                    let mut output: String<20>=format!("").unwrap();
                    let _ = output.push_str(&num_str);

                    info!("{}E{}", n, exp);
                    format!("{}E{}", output, exp).unwrap()
                },
                DisplayStyle::S(sf) => {
                    format!("Not implemented").unwrap()
                },
                DisplayStyle::FIXED => {
                    format!("Not implemented").unwrap()
                },
                DisplayStyle::ALL => { 
                    format!("Not implemented").unwrap()
                }
            }
        }
    }


    pub fn set_on(&mut self, on: bool) {
        self.display.set_display_on(on).unwrap();

        let _ = self.display.flush();
        self.display.set_display_on(true).unwrap();

        let num_str: String<20> =  format!("{}", "Screen on").unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(0, 13), self.font).draw(&mut self.display);
    }


    pub fn update_stack_display(&mut self) {
        self.display.clear(BinaryColor::Off);

        // let n_decimals = 4;

        let (x, y, z, t) = self.stack.fetch_values();
        // info!("x: {}, y: {}, z: {}, t: {}", x, y, z, t);
        let sf: i32 =  3; 
        
        let x_buffer_str = self.num_to_string(x);
        let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&x_buffer_str, Point::new(NUM_LEFT, X_NUM_BOTTOM), self.font).draw(&mut self.display);

        let y_buffer_str = self.num_to_string(y);
        let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&y_buffer_str, Point::new(NUM_LEFT, Y_NUM_BOTTOM), self.font).draw(&mut self.display);

        let z_buffer_str = self.num_to_string(z,);
        let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&z_buffer_str, Point::new(NUM_LEFT, Z_NUM_BOTTOM), self.font).draw(&mut self.display);

        let t_buffer_str = self.num_to_string(t);
        let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(&t_buffer_str, Point::new(NUM_LEFT, T_NUM_BOTTOM), self.font).draw(&mut self.display);


        self.display.flush().unwrap();       // Flushes internal buffer to the display

        self.stack.test_increment();




        // delay(1_000);
        // info!("looping");
    }

}