#![no_std]
#![no_main]

use stm32f1::stm32f103::Peripherals;

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let peripherals = Peripherals::take().unwrap();
    // enable the clock (APB2) connected to GPIOC (which PC13 lays within)
    peripherals
        .RCC
        .apb2enr()
        .modify(|_, w| w.iopcen().enabled());
    // set PC13 to push-pull mode at 2MHz
    peripherals
        .GPIOC
        .crh()
        .modify(|_, w| w.mode13().output2().cnf13().push_pull());

    loop {
        peripherals.GPIOC.bsrr().write(|w| w.bs13().set_bit());
        cortex_m::asm::delay(5_000_000);

        peripherals.GPIOC.bsrr().write(|w| w.br13().set_bit());
        cortex_m::asm::delay(5_000_000);
    }
}
