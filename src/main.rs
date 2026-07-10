#![no_std]
#![no_main]

use core::cell::RefCell;

// use core::mem::MaybeUninit;

// use nostd::format;

use cortex_m::asm::delay;
use defmt::*;
use defmt::info; //unnecessary?

use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
// use embassy_embedded_hal::shared_bus::SpiDeviceError;

use embassy_rp::gpio::{Level, Output};
// use embassy_rp::gpio::{Input, Level, Pull};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, ClkPin, Config, MisoPin, MosiPin, Spi};


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_time::Delay;
// use embassy_time::{Duration, Timer};

//, text};
// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*};
// use embedded_graphics::text::Text;

// use embedded_hal::spi::SpiDevice;
// use embedded_hal::digital::{InputPin, OutputPin};

use embassy_executor::Spawner;
// use embassy_rp::gpio;

// use heapless::{format, String};

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

mod display;
use display::DisplayStruct;

// use defmt::{Format};
use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

struct FlashLedStruct {
    led: Output<'static>,
    delay: u32,
}

impl FlashLedStruct {
    fn new(led: Output<'static>, delay: u32) -> Self {
        Self { led, delay }
    }

    fn flash(&mut self) {
        self.led.set_high();
        delay(self.delay);
        self.led.set_low();
        delay(self.delay);
    }
}


// struct DisplayStruct <'a>{
//     pub display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//     reset_pin: Output<'a>,
//     font: MonoTextStyle<'a, BinaryColor>,
// }

// impl <'a> DisplayStruct <'a>{
//     pub fn new(display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//                 reset_pin: Output<'a>, font: MonoTextStyle<'a, BinaryColor>) -> Self {

//         Self { 
//             display, 
//             reset_pin,
//             font,
//         }
//     }

//     pub fn set_on(&mut self, on: bool) {
//         self.display.set_display_on(on).unwrap();

//     }
// }

#[embassy_executor::main]
async fn main (_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Started");


    let pico_led = Output::new(p.PIN_25, Level::High);
    let mut flash_led = FlashLedStruct::new(pico_led, 10_000_000);
    flash_led.flash();


    let mosi = p.PIN_19;
    let miso  = p.PIN_20;
    let display_cs = p.PIN_21;
    let clk = p.PIN_18;
    let reset  = p.PIN_28;
    let a0 = p.PIN_27;

    let a0 = Output::new(a0, Level::Low);   
    let display_config = spi::Config::default();

    let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);

       info!("display interface created");

    let mut page_buffer = GraphicsPageBuffer::new();
    let display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'_, NoopRawMutex, embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8> = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);   
    let reset_pin = Output::new(reset, Level::Low);
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    let mut display: DisplayStruct = DisplayStruct::new(display, reset_pin, font);

    display.set_on(true);


    // display.display.reset(&mut reset, &mut Delay).unwrap();
    let _ = display.display.flush();
    display.set_on(true);
    // display.set_display_on(true).unwrap();
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    // let mut i =0; 
        
    // let mut num = 0 as u8;
    // loop{
    //     let _ = display.flush();
    //     flash_led.flash();


    //     num += 1;

    //     let num_str: String<20> =  format!("{}", num).unwrap();//Format!("{}".num);
    //     let _ =Text::new(&num_str, Point::new(0, 13), font)
    //             .draw(&mut display);
    //     let _ =Text::new("123.4567", Point::new(0, 29), font)
    //             .draw(&mut display);
    //     let _ =Text::new("34.5678", Point::new(3, 45), font)
    //             .draw(&mut display);
    //     let _ =Text::new("88.8888", Point::new(3, 61), font)
    //             .draw(&mut display);

    //     info!("before flush");
    //     display.flush().unwrap();       // Flushes internal buffer to the display
    //     delay(100_000_000);
    //     info!("looping");

    // }

    loop{

    }

}
