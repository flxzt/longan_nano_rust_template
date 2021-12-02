#![no_std]
#![no_main]

const FRAME_TIME: u32 = 20;

use embedded_graphics::style::PrimitiveStyle;
use panic_halt as _;

use embedded_graphics::pixelcolor::{raw::RawU16, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitive_style;
use embedded_graphics::primitives::{Rectangle, Triangle};
use embedded_hal::blocking::delay::DelayMs;
use longan_nano::hal::{delay::McycleDelay, pac, prelude::*};
use longan_nano::lcd::{self, Lcd};
use longan_nano::lcd_pins;
use longan_nano::led::{rgb, Led};
use riscv_rt::entry;

fn draw_rect<C>(lcd: &mut Lcd, c: C, ul: (i32, i32), lr: (i32, i32))
where
    C: Into<Rgb565>,
{
    let rect = Rectangle::new(Point::new(ul.0, ul.1), Point::new(lr.0, lr.1));
    let _ = rect
        .into_styled(primitive_style!(fill_color = c.into()))
        .draw(lcd);
}

fn draw_triangle<C>(lcd: &mut Lcd, c: C, p0: Point, p1: Point, p2: Point)
where
    C: Into<Rgb565>,
{
    // Triangle with red 1 px wide stroke
    let _ = Triangle::new(p0, p1, p2)
        .into_styled(PrimitiveStyle::with_stroke(c.into(), 1))
        .draw(lcd);
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks.
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    // Take peripherals.
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    let mut leds: [&mut dyn Led; 3] = [&mut red, &mut green, &mut blue];

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (screen_width, screen_height) = (160, 80);

    let mut delay = McycleDelay::new(&rcu.clocks);

    // Blacken LEDs
    for c in &mut leds {
        c.off();
    }

    // Clear screen
    draw_rect(
        &mut lcd,
        RawU16::from(0u16),
        (0, 0),
        (screen_width, screen_height),
    );

    let mut c = 0;
    let m = 64 * leds.len();
    static BALL_COLORS: [u16; 8] = [
        (0x03 << 11) + (0x00 << 5) + 0x07,
        (0x07 << 11) + (0x00 << 5) + 0x03,
        (0x1f << 11) + (0x00 << 5) + 0x00,
        (0x07 << 11) + (0x03 << 5) + 0x00,
        (0x03 << 11) + (0x07 << 5) + 0x00,
        (0x00 << 11) + (0x1f << 5) + 0x03,
        (0x00 << 11) + (0x07 << 5) + 0x07,
        (0x00 << 11) + (0x03 << 5) + 0x1f,
    ];

    let (p0, p1, p2) = (Point::new(50, 20), Point::new(110, 20), Point::new(80, 60));

    loop {
        // Change LED color.
        leds[c / 64].off();
        c = (c + 1) % m;
        leds[c / 64].on();

        draw_triangle(&mut lcd,RawU16::from(BALL_COLORS[c / 24]), p0, p1, p2);

        // Limit update rate to FRAME_TIME
        delay.delay_ms(FRAME_TIME);
    }
}
