
use cortex_m::asm::delay;
use defmt::info;
use core::f64;
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


pub struct DisplayStruct <'a>{
    pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    reset_pin: Output<'a>,
    font: MonoTextStyle<'a, BinaryColor>,
    stack: stack::Stack,
}

impl <'a> DisplayStruct <'a>{
    pub fn new(mut display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
                mut reset_pin: Output<'a>, font: MonoTextStyle<'a, BinaryColor>) -> Self {
        
        display.reset(&mut reset_pin, &mut Delay).unwrap();

        let stack = stack::Stack::new();


        Self { 
            display, 
            reset_pin,
            font,
            stack,
        }
    }

    pub fn num_to_string(&self, n: f64, sf: i32)->String<20>{
        if n == 0.0 {
            let mut output: String<20>=format!("").unwrap();
            output.push('0');
            output.push('.');
            for _ in 0..sf {
                output.push('0');
            }
            output.push('E');
            output.push('0');
            return output;
        }

        let exponent: i32 = 1 + libm::log10(n).floor() as i32;
        let mut before_dp = exponent % 3;  // This gives everything powers for 10^3, 10^-3, etc

        if before_dp ==0 {before_dp=3}; 
        if before_dp<0 {
            before_dp=3+before_dp
        };
        let exp = exponent - before_dp;

        info!("n: {}, exponent: {}, sf:before_dp: {}", n, exponent, before_dp);

        let n = (n/(10.0_f64).powi(exponent-sf)).trunc()/10_f64.powi(sf-before_dp);

        format!("{}E{}", n, exp).unwrap()
    }


    pub fn set_on(&mut self, on: bool) {
        self.display.set_display_on(on).unwrap();

        let _ = self.display.flush();
        self.display.set_display_on(true).unwrap();

        let num_str: String<20> =  format!("{}", "Screen on").unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(0, 13), self.font).draw(&mut self.display);
    }


    pub fn update_stack(&mut self) {
        self.display.clear(BinaryColor::Off);

        let n_decimals = 4;

        // let format: StringInner<usize, VecStorageInner<[MaybeUninit<u8>; 4]>> = "{:e}";
        // let mut num_str: String<20>;

        let (x, y, z, t) = self.stack.fetch_values();
        // num_str =  format!("{:.*}", n_decimals, x).unwrap();
        
        let num_str: String<20> =  self.num_to_string(x, 3 as i32);
        let _ =Text::new(&num_str, Point::new(0, 13), self.font).draw(&mut self.display);
        
        let num_str: String<20> =  format!("{:.*}", n_decimals, y).unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(0, 29), self.font).draw(&mut self.display);
        let num_str: String<20> =  format!("{:.*}", n_decimals, z).unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(3, 45), self.font).draw(&mut self.display);
        let num_str: String<20> =  format!("{:.*}", n_decimals, t).unwrap();//Format!("{}".num);
        let _ =Text::new(&num_str, Point::new(3, 61), self.font).draw(&mut self.display);

        self.display.flush().unwrap();       // Flushes internal buffer to the display

        self.stack.test_increment();




        // delay(1_000);
        // info!("looping");
    }
    //     let num_str: String<20> =  format!("{}", num).unwrap();//Format!("{}".num);
    //     let _ =Text::new(&num_str, Point::new(0, 13), self.font).draw(&mut self.display);
    //     let _ =Text::new("123.4567", Point::new(0, 29), self.font).draw(&mut self.display);
    //     let _ =Text::new("34.5678", Point::new(3, 45), self.font).draw(&mut self.display);
    //     let _ =Text::new("88.8888", Point::new(3, 61), self.font).draw(&mut self.display);

    //     info!("before flush");
    //     self.display.flush().unwrap();       // Flushes internal buffer to the display
    //     delay(100_000_000);
    //     info!("looping");
    // }


}