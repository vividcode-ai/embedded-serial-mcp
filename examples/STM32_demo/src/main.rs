#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_time::{Duration, Timer};
use fmt::info;

// Simple state tracking
static mut LED_STATE: bool = false;
static mut COUNTER: u32 = 0;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Serial Interactive Demo Starting!");
    
    // Initialize STM32 peripherals
    let p = embassy_stm32::init(Default::default());
    
    // Initialize LED on PB7
    let mut led = Output::new(p.PB7, Level::Low, Speed::Low);
    
    // Configure USART1 with PA9 (TX) and PA10 (RX)
    let config = Config::default(); // 115200 baud, 8N1
    let mut usart = Uart::new_blocking(p.USART1, p.PA10, p.PA9, config).unwrap();
    
    // Send welcome message
    usart.blocking_write(b"\r\n=== STM32 Serial Demo ===\r\n").unwrap();
    usart.blocking_write(b"Commands:\r\n").unwrap();
    usart.blocking_write(b"  H - Help\r\n").unwrap();
    usart.blocking_write(b"  L - Toggle LED\r\n").unwrap();
    usart.blocking_write(b"  C - Show counter\r\n").unwrap();
    usart.blocking_write(b"  R - Reset counter\r\n").unwrap();
    usart.blocking_write(b"  B - Blink LED 3 times\r\n").unwrap();
    usart.blocking_write(b"  Any other key - Echo\r\n\r\n").unwrap();
    
    info!("USART1 initialized, starting command loop");
    
    let mut buf = [0u8; 1];
    
    loop {
        // Read one character
        usart.blocking_read(&mut buf).unwrap();
        let cmd = buf[0];
        
        // Process commands
        match cmd {
            b'H' | b'h' => {
                // Help command
                usart.blocking_write(b"\r\n=== HELP ===\r\n").unwrap();
                usart.blocking_write(b"Available commands:\r\n").unwrap();
                usart.blocking_write(b"  H/h - Show this help\r\n").unwrap();
                usart.blocking_write(b"  L/l - Toggle LED state\r\n").unwrap();
                usart.blocking_write(b"  C/c - Show counter value\r\n").unwrap();
                usart.blocking_write(b"  R/r - Reset counter to 0\r\n").unwrap();
                usart.blocking_write(b"  B/b - Blink LED 3 times\r\n").unwrap();
                usart.blocking_write(b"  Other - Echo character\r\n\r\n").unwrap();
                info!("Help command executed");
            }
            
            b'L' | b'l' => {
                // Toggle LED
                unsafe {
                    LED_STATE = !LED_STATE;
                    if LED_STATE {
                        led.set_high();
                        usart.blocking_write(b"\r\nLED: ON\r\n").unwrap();
                    } else {
                        led.set_low();
                        usart.blocking_write(b"\r\nLED: OFF\r\n").unwrap();
                    }
                }
                info!("LED toggled to: {}", unsafe { LED_STATE });
            }
            
            b'C' | b'c' => {
                // Show counter
                unsafe {
                    COUNTER += 1;
                    usart.blocking_write(b"\r\nCounter: ").unwrap();
                    
                    // Simple number to string conversion
                    let mut num = COUNTER;
                    let mut buffer = [0u8; 10];
                    let mut i = 9;
                    
                    if num == 0 {
                        usart.blocking_write(b"0").unwrap();
                    } else {
                        while num > 0 && i > 0 {
                            buffer[i] = b'0' + (num % 10) as u8;
                            num /= 10;
                            i -= 1;
                        }
                        usart.blocking_write(&buffer[i+1..]).unwrap();
                    }
                    
                    usart.blocking_write(b"\r\n").unwrap();
                }
                info!("Counter shown: {}", unsafe { COUNTER });
            }
            
            b'R' | b'r' => {
                // Reset counter
                unsafe {
                    COUNTER = 0;
                }
                usart.blocking_write(b"\r\nCounter reset to 0\r\n").unwrap();
                info!("Counter reset");
            }
            
            b'B' | b'b' => {
                // Blink LED 3 times
                usart.blocking_write(b"\r\nBlinking LED 3 times...\r\n").unwrap();
                
                for i in 0..3 {
                    led.set_high();
                    Timer::after(Duration::from_millis(200)).await;
                    led.set_low();
                    Timer::after(Duration::from_millis(200)).await;
                    info!("Blink {}/3", i + 1);
                }
                
                // Restore LED state
                unsafe {
                    if LED_STATE {
                        led.set_high();
                    }
                }
                usart.blocking_write(b"Blink complete!\r\n").unwrap();
            }
            
            b'\r' => {
                // Handle carriage return
                usart.blocking_write(b"\r\n").unwrap();
            }
            
            b'\n' => {
                // Ignore line feed
            }
            
            _ => {
                // Echo other characters
                usart.blocking_write(&[cmd]).unwrap();
                if cmd >= 32 && cmd <= 126 {
                    info!("Echo: '{}'", cmd as char);
                } else {
                    info!("Echo: 0x{:02x}", cmd);
                }
            }
        }
        
        // Small async delay to prevent CPU hogging
        Timer::after(Duration::from_micros(10)).await;
    }
}
