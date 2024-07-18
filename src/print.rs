#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n\r")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}{}", format_args!($($arg)*), "\r\n");
    }
}

/// kprint and kprinln
/// are used to output debug information
/// to the serial console/kernel ring buffer(TODO)
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        // These brackets are very important becuse they drop the mutex 
        // after it's done printing calling release here will not work
        {
            use core::fmt::Write;
            let mut console = crate::SERIAL_CONSOLE.aquire();
            let _ = console.write_fmt(format_args!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n\r")
    };
    ($($arg:tt)*) => {
        $crate::kprint!("{}{}", format_args!($($arg)*), "\r\n");
    }
}

