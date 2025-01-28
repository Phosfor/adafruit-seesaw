#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use adafruit_seesaw::{devices::NeoTrellis, prelude::*, SeesawRefCell};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{gpio::GpioExt, i2c::I2c, pac, prelude::*, rcc::RccExt};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Begin");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let clocks = dp.RCC.constrain().cfgr.freeze();
    let delay = cp.SYST.delay(&clocks);
    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    let seesaw = SeesawRefCell::new(delay, i2c);
    rprintln!("Seesaw created");
    let mut trellis = NeoTrellis::new_with_default_addr(seesaw.acquire_driver())
        .init()
        .expect("Failed to start NeoTrellis");

    rprintln!(
        "Capabilities {:#?}",
        trellis.capabilities().expect("Failed to get options")
    );

    for x in 0..trellis.cols() {
        for y in 0..trellis.rows() {
            trellis
                .set_key_events(x, y, &[KeyEventType::Pressed, KeyEventType::Released], true)
                .expect("Failed to set key events");
            trellis
                .set_nth_neopixel_color((y * trellis.cols() + x).into(), 0, 50, 0)
                .expect("Failed to set neopixel color");
        }
    }
    // NeoTrellis::<I2c>::keys().for_each(|k| {
    //     trellis
    //         .set_key_events(
    //             k.x(),
    //             k.y(),
    //             &[KeyEventType::Pressed, KeyEventType::Released],
    //             true,
    //         )
    //         .and_then(|_| trellis.set_nth_neopixel_color(k.index() as u16, 0, 0,
    // 0))         .expect("Failed to set key events");
    // });

    trellis.sync_neopixel().expect("Failed to sync neopixel");
    let color1 = color_wheel(0);
    let color2 = color_wheel(64);

    rprintln!("Looping...");

    loop {
        // let mut events: [Option<KeyEvent>; 16] = [None; 16];
        let events_iter = trellis.read_events().expect("Failed to read events");
        // events_iter.m
        for event in events_iter {
            rprintln!("Event {:#?}", event);
            match event.event {
                KeyEventType::Pressed => {
                    // rprintln!("Pressed");
                    trellis
                        .set_nth_neopixel_color(
                            (event.x + event.y * 4) as u16,
                            color2.0,
                            color2.1,
                            color2.2,
                        )
                        .and_then(|_| trellis.sync_neopixel())
                        .expect("Failed to set neopixel color");
                }
                KeyEventType::Released => {
                    trellis
                        .set_nth_neopixel_color(
                            (event.x + event.y * 4) as u16,
                            color1.0,
                            color1.1,
                            color1.2,
                        )
                        .and_then(|_| trellis.sync_neopixel())
                        .expect("Failed to set neopixel color");
                    // rprintln!("Released");
                }
                _ => {}
            }
            // rprintln!("Event {:#?}", event);
        }
        // rprintln!("Events {:#?}", events);
        // delay.delay_ms(100);
    }
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC! {}", info.message());
    if let Some(location) = info.location() {
        rprintln!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        rprintln!("Panic occurred but can't get location information...");
    }
    loop {}
}

fn color_wheel(byte: u8) -> Color {
    match byte {
        0..=84 => Color(255 - byte * 3, 0, byte * 3),
        85..=169 => Color(0, (byte - 85) * 3, 255 - (byte - 85) * 3),
        _ => Color((byte - 170) * 3, 255 - (byte - 170) * 3, 0),
    }
}

#[derive(Copy, Clone, Debug)]
struct Color(pub u8, pub u8, pub u8);

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.0, value.1, value.2)
    }
}
