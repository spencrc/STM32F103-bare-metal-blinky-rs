#![no_std]

core::arch::global_asm! {
    ".global Reset
     .thumb_func
     .type Reset, %function
     Reset:",
    // Copy .data section to SRAM
    "ldr r0, = _sdata
     ldr r1, = _edata
     ldr r2, = _etext
     0:
     cmp r1, r0
     beq 1f
     ldm r2!, {{r3}}
     stm r0!, {{r3}}
     b 0b
     1:",
    // Zero out the .bss section to zero in SRAM
    "ldr r0, = _sbss
     ldr r1, = _ebss
     movs r2, #0
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",
    // Call main, which should never return
    "bl main
     udf #0",
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler() -> ! {
    loop {}
}

#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

unsafe extern "C" {
    fn Reset() -> !;
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();
    fn WWDG();
    fn PVD();
    fn TAMPER();
    fn RTC();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_Channel1();
    fn DMA1_Channel2();
    fn DMA1_Channel3();
    fn DMA1_Channel4();
    fn DMA1_Channel5();
    fn DMA1_Channel6();
    fn DMA1_Channel7();
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTCAlarm();
    fn USBWakeup();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FSMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_Channel1();
    fn DMA2_Channel2();
    fn DMA2_Channel3();
    fn DMA2_Channel4_5();
}

#[unsafe(link_section = ".vector_table.reset")]
#[unsafe(no_mangle)]
pub static __RESET: unsafe extern "C" fn() -> ! = Reset;

#[unsafe(link_section = ".vector_table.vectors")]
#[unsafe(no_mangle)]
pub static __VECTORS: [Vector; 74] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector {
        handler: DebugMonitor,
    },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
    Vector { handler: WWDG },
    Vector { handler: PVD },
    Vector { handler: TAMPER },
    Vector { handler: RTC },
    Vector { handler: FLASH },
    Vector { handler: RCC },
    Vector { handler: EXTI0 },
    Vector { handler: EXTI1 },
    Vector { handler: EXTI2 },
    Vector { handler: EXTI3 },
    Vector { handler: EXTI4 },
    Vector {
        handler: DMA1_Channel1,
    },
    Vector {
        handler: DMA1_Channel2,
    },
    Vector {
        handler: DMA1_Channel3,
    },
    Vector {
        handler: DMA1_Channel4,
    },
    Vector {
        handler: DMA1_Channel5,
    },
    Vector {
        handler: DMA1_Channel6,
    },
    Vector {
        handler: DMA1_Channel7,
    },
    Vector { handler: ADC1_2 },
    Vector {
        handler: USB_HP_CAN_TX,
    },
    Vector {
        handler: USB_LP_CAN_RX0,
    },
    Vector { handler: CAN_RX1 },
    Vector { handler: CAN_SCE },
    Vector { handler: EXTI9_5 },
    Vector { handler: TIM1_BRK },
    Vector { handler: TIM1_UP },
    Vector {
        handler: TIM1_TRG_COM,
    },
    Vector { handler: TIM1_CC },
    Vector { handler: TIM2 },
    Vector { handler: TIM3 },
    Vector { handler: TIM4 },
    Vector { handler: I2C1_EV },
    Vector { handler: I2C1_ER },
    Vector { handler: I2C2_EV },
    Vector { handler: I2C2_ER },
    Vector { handler: SPI1 },
    Vector { handler: SPI2 },
    Vector { handler: USART1 },
    Vector { handler: USART2 },
    Vector { handler: USART3 },
    Vector { handler: EXTI15_10 },
    Vector { handler: RTCAlarm },
    Vector { handler: USBWakeup },
    Vector { handler: TIM8_BRK },
    Vector { handler: TIM8_UP },
    Vector {
        handler: TIM8_TRG_COM,
    },
    Vector { handler: TIM8_CC },
    Vector { handler: ADC3 },
    Vector { handler: FSMC },
    Vector { handler: SDIO },
    Vector { handler: TIM5 },
    Vector { handler: SPI3 },
    Vector { handler: UART4 },
    Vector { handler: UART5 },
    Vector { handler: TIM6 },
    Vector { handler: TIM7 },
    Vector {
        handler: DMA2_Channel1,
    },
    Vector {
        handler: DMA2_Channel2,
    },
    Vector {
        handler: DMA2_Channel3,
    },
    Vector {
        handler: DMA2_Channel4_5,
    },
];
