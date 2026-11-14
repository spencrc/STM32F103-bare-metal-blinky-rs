EXTERN(__RESET)
EXTERN(Reset)
ENTRY(Reset)

EXTERN(__VECTORS)

EXTERN(DefaultHandler)

MEMORY
{
    FLASH(rx): ORIGIN = 0x08000000, LENGTH = 64K
    SRAM(rwx): ORIGIN = 0x20000000, LENGTH = 20K
}

_stack_start = ORIGIN(SRAM) + LENGTH(SRAM);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        LONG(_stack_start);
        KEEP(*(.vector_table.reset));
        KEEP(*(.vector_table.vectors));
    } > FLASH

    .text :
    {
        . = ALIGN(4);
        *(.text .text.*)
        *(.rodata .rodata.*)
        . = ALIGN(4);
        _etext = .;
    }> FLASH

    .data :
    {
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;
    }> SRAM AT> FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    }> SRAM

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
        *(.ARM.extab .ARM.extab.*);
    }
}

PROVIDE(NMI = DefaultHandler);
PROVIDE(HardFault = DefaultHandler);
PROVIDE(MemManage = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(WWDG = DefaultHandler);
PROVIDE(PVD = DefaultHandler);
PROVIDE(TAMPER = DefaultHandler);
PROVIDE(RTC = DefaultHandler);
PROVIDE(FLASH = DefaultHandler);
PROVIDE(RCC = DefaultHandler);
PROVIDE(EXTI0 = DefaultHandler);
PROVIDE(EXTI1 = DefaultHandler);
PROVIDE(EXTI2 = DefaultHandler);
PROVIDE(EXTI3 = DefaultHandler);
PROVIDE(EXTI4 = DefaultHandler);
PROVIDE(DMA1_Channel1 = DefaultHandler);
PROVIDE(DMA1_Channel2 = DefaultHandler);
PROVIDE(DMA1_Channel3 = DefaultHandler);
PROVIDE(DMA1_Channel4 = DefaultHandler);
PROVIDE(DMA1_Channel5 = DefaultHandler);
PROVIDE(DMA1_Channel6 = DefaultHandler);
PROVIDE(DMA1_Channel7 = DefaultHandler);
PROVIDE(ADC1_2 = DefaultHandler);
PROVIDE(USB_HP_CAN_TX = DefaultHandler);
PROVIDE(USB_LP_CAN_RX0 = DefaultHandler);
PROVIDE(CAN_RX1 = DefaultHandler);
PROVIDE(CAN_SCE = DefaultHandler);
PROVIDE(EXTI9_5 = DefaultHandler);
PROVIDE(TIM1_BRK = DefaultHandler);
PROVIDE(TIM1_UP = DefaultHandler);
PROVIDE(TIM1_TRG_COM = DefaultHandler);
PROVIDE(TIM1_CC = DefaultHandler);
PROVIDE(TIM2 = DefaultHandler);
PROVIDE(TIM3 = DefaultHandler);
PROVIDE(TIM4 = DefaultHandler);
PROVIDE(I2C1_EV = DefaultHandler);
PROVIDE(I2C1_ER = DefaultHandler);
PROVIDE(I2C2_EV = DefaultHandler);
PROVIDE(I2C2_ER = DefaultHandler);
PROVIDE(SPI1 = DefaultHandler);
PROVIDE(SPI2 = DefaultHandler);
PROVIDE(USART1 = DefaultHandler);
PROVIDE(USART2 = DefaultHandler);
PROVIDE(USART3 = DefaultHandler);
PROVIDE(EXTI15_10 = DefaultHandler);
PROVIDE(RTCAlarm = DefaultHandler);
PROVIDE(USBWakeup = DefaultHandler);
PROVIDE(TIM8_BRK = DefaultHandler);
PROVIDE(TIM8_UP = DefaultHandler);
PROVIDE(TIM8_TRG_COM = DefaultHandler);
PROVIDE(TIM8_CC = DefaultHandler);
PROVIDE(ADC3 = DefaultHandler);
PROVIDE(FSMC = DefaultHandler);
PROVIDE(SDIO = DefaultHandler);
PROVIDE(TIM5 = DefaultHandler);
PROVIDE(SPI3 = DefaultHandler);
PROVIDE(UART4 = DefaultHandler);
PROVIDE(UART5 = DefaultHandler);
PROVIDE(TIM6 = DefaultHandler);
PROVIDE(TIM7 = DefaultHandler);
PROVIDE(DMA2_Channel1 = DefaultHandler);
PROVIDE(DMA2_Channel2 = DefaultHandler);
PROVIDE(DMA2_Channel3 = DefaultHandler);
PROVIDE(DMA2_Channel4_5 = DefaultHandler);