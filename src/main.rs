#![no_std]
#![no_main]

use core::cell::RefCell;

// use core::mem::MaybeUninit;

// use nostd::format;

use cortex_m::asm::delay;
// use defmt::*;
// use defmt::{Format};

use {defmt_rtt as _, panic_probe as _};

use defmt::info; //unnecessary?

mod display;
use display::DisplayStruct;
use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
// use embassy_embedded_hal::shared_bus::SpiDeviceError;

use embassy_rp::gpio::{Level, Output};
// use embassy_rp::gpio::{Input, Level, Pull};
use embassy_rp::peripherals::{SPI0};
// use embassy_rp::{Peri, PeripheralType};
use embassy_rp::rom_data;
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, ClkPin, Config, MisoPin, MosiPin, Spi};


use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_time::Delay;
// use embassy_time::{Duration, Timer};

//, text};

// use embedded_hal::spi::SpiDevice;
// use embedded_hal::digital::{InputPin, OutputPin};

use embassy_executor::Spawner;
// use embassy_rp::gpio;

use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;

// use rp235x_hal as hal;

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
use st7565::ST7565;
use st7565::modes::GraphicsMode;

mod stack;



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


#[embassy_executor::main]
async fn main (_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Started");


    let pico_led = Output::new(p.PIN_25, Level::High);
    let mut flash_led = FlashLedStruct::new(pico_led, 20_000_000);
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
    let stacknames_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

    let mut display: DisplayStruct = DisplayStruct::new(display, reset_pin, font, stacknames_font);

    display.set_on(true);
    let mut stack = stack::Stack::new();

    // display.display.reset(&mut reset, &mut Delay).unwrap();
    let _ = display.display.flush();
    display.set_on(true);

    display.update_stack_display();

    //     let num_str: String<20> =  format!("{}", num).unwrap();//Format!("{}".num);
    //     let _ =Text::new(&num_str, Point::new(0, 13), font)
    //             .draw(&mut display);
 
    loop{
        // info!("In loop");
        display.update_stack_display(); 
        stack.swapxy();
        stack.set_changed();
        delay(100_000_000);

    }

}
