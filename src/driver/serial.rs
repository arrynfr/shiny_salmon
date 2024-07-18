use core::{fmt::{Result,Write}};

pub struct Serial {
    pub serial_addr: *mut u8
}

unsafe impl Send for Serial {}

impl Serial {
    pub fn new(serial_addr: *mut u8) -> Self {
        Serial {
            serial_addr: serial_addr as *mut u8
        }
    }

    pub fn serial_putc(&self, c: char) {
        unsafe {
            self.serial_addr.write_volatile(c as u8);
        }
    }
    
    pub fn serial_puts(&self, string: &str) {
        for c in string.chars() {
            self.serial_putc(c);
        }
    }
    
    pub fn serial_getchar(&self) -> Option<u8> {
        if self.serial_addr != 0 as *mut u8 {
            unsafe {
                if self.serial_addr.add(0x18).read_volatile() & 1 << 4 == 0 {
                    return Some(self.serial_addr.add(0).read_volatile())
                }
            }
        }
        None
    }
}

impl Write for Serial {
    fn write_str(&mut self, string: &str) -> Result {
        self.serial_puts(string);
        Ok(())
    }
}
