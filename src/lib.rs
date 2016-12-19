extern crate glob;
#[macro_use]
extern crate lazy_static;
extern crate sysfs_gpio;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use glob::glob;
use sysfs_gpio::Pin;

/// CHIP GPIO pins by name
#[allow(non_camel_case_types)]
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub enum ChipPin {
    PWM0,
    AP_EINT3,
    TWI1_SCK,
    TWI1_SDA,
    TWI2_SCK,
    TWI2_SDA,
    LCD_D2,
    LCD_D3,
    LCD_D4,
    LCD_D5,
    LCD_D6,
    LCD_D7,
    LCD_D10,
    LCD_D11,
    LCD_D12,
    LCD_D13,
    LCD_D14,
    LCD_D15,
    LCD_D18,
    LCD_D19,
    LCD_D20,
    LCD_D21,
    LCD_D22,
    LCD_D23,
    LCD_CLK,
    LCD_DE,
    LCD_HSYNC,
    LCD_VSYNC,
    CSIPCK,
    CSICK,
    CSIHSYNC,
    CSIVSYNC,
    CSID0,
    CSID1,
    CSID2,
    CSID3,
    CSID4,
    CSID5,
    CSID6,
    CSID7,
    AP_EINT1,
    UART1_TX,
    UART1_RX,
    XIO_P0,
    XIO_P1,
    XIO_P2,
    XIO_P3,
    XIO_P4,
    XIO_P5,
    XIO_P6,
    XIO_P7,
}

lazy_static! {
    static ref XIO_BASE: u64 = get_xio_base();
}

fn get_xio_base() -> u64 {
    for entry in glob("/sys/class/gpio/*/*label").expect("Failed to read glob pattern") {
        let path = entry.expect("Error listing /sys/class/gpio/*/*label");
        let mut buf = [0; 8];
        File::open(&path)
            .expect("Failed to open label file")
            .read(&mut buf)
            .expect("Failed to read label file");
        if &buf == b"pcf8574a" {
            let file = File::open(&path.parent().unwrap().join("base"))
                .expect("Failed to read base file");
            let reader = BufReader::new(file);
            return reader.lines()
                .next()
                .expect("Hit EOF reading XIO base file")
                .expect("Error reading XIO base file")
                .parse()
                .expect("Error parsing content of XIO base file as u64");
        }
    }
    panic!("Failed to find XIO base");
}


impl ChipPin {
    pub fn get(self) -> Pin {
        Pin::new(self.num())
    }

    pub fn num(self) -> u64 {
        match self {
            ChipPin::PWM0 => 34,
            ChipPin::AP_EINT3 => 35,
            ChipPin::TWI1_SCK => 47,
            ChipPin::TWI1_SDA => 48,
            ChipPin::TWI2_SCK => 49,
            ChipPin::TWI2_SDA => 50,
            ChipPin::LCD_D2 => 98,
            ChipPin::LCD_D3 => 99,
            ChipPin::LCD_D4 => 100,
            ChipPin::LCD_D5 => 101,
            ChipPin::LCD_D6 => 102,
            ChipPin::LCD_D7 => 103,
            ChipPin::LCD_D10 => 106,
            ChipPin::LCD_D11 => 107,
            ChipPin::LCD_D12 => 108,
            ChipPin::LCD_D13 => 109,
            ChipPin::LCD_D14 => 110,
            ChipPin::LCD_D15 => 111,
            ChipPin::LCD_D18 => 114,
            ChipPin::LCD_D19 => 115,
            ChipPin::LCD_D20 => 116,
            ChipPin::LCD_D21 => 117,
            ChipPin::LCD_D22 => 118,
            ChipPin::LCD_D23 => 119,
            ChipPin::LCD_CLK => 120,
            ChipPin::LCD_DE => 121,
            ChipPin::LCD_HSYNC => 122,
            ChipPin::LCD_VSYNC => 123,
            ChipPin::CSIPCK => 128,
            ChipPin::CSICK => 129,
            ChipPin::CSIHSYNC => 130,
            ChipPin::CSIVSYNC => 131,
            ChipPin::CSID0 => 132,
            ChipPin::CSID1 => 133,
            ChipPin::CSID2 => 134,
            ChipPin::CSID3 => 135,
            ChipPin::CSID4 => 136,
            ChipPin::CSID5 => 137,
            ChipPin::CSID6 => 138,
            ChipPin::CSID7 => 139,
            ChipPin::AP_EINT1 => 193,
            ChipPin::UART1_TX => 195,
            ChipPin::UART1_RX => 196,
            ChipPin::XIO_P0 => *XIO_BASE,
            ChipPin::XIO_P1 => *XIO_BASE + 1,
            ChipPin::XIO_P2 => *XIO_BASE + 2,
            ChipPin::XIO_P3 => *XIO_BASE + 3,
            ChipPin::XIO_P4 => *XIO_BASE + 4,
            ChipPin::XIO_P5 => *XIO_BASE + 5,
            ChipPin::XIO_P6 => *XIO_BASE + 6,
            ChipPin::XIO_P7 => *XIO_BASE + 7,
        }
    }
}
