// https://www.st.com/en/microcontrollers-microprocessors/stm32f103/documentation.html

#include <stdint.h>
 
/* Defines a weak (override permitted) alias pointing to Default_handler.
 * Overriding example:
 * 
 * #[unsafe(no_mangle)]
 * pub extern "C" fn NMI_handler() -> ! {
 *  loop { }
 * }
*/ 
#define WEAK_ALIAS __attribute__((weak, alias("Default_handler")))
// Specifies a new data section to be used by the linker
#define VECTOR_SECTION __attribute__((section(".vector_table")))

typedef void (*vector_entry)(void);

// Boundaries defined by linker script
extern uint32_t _stack_start;
extern uint32_t _etext;
extern uint32_t _sdata;
extern uint32_t _edata;
extern uint32_t _sbss;
extern uint32_t _ebss;

int main(void);

// Fixed vector entries
void Reset_handler(void);
void NMI_handler(void) WEAK_ALIAS;
void HardFault_handler(void) WEAK_ALIAS;

// Settable vector entries
void MemManage_handler(void) WEAK_ALIAS;
void BusFault_handler(void) WEAK_ALIAS;
void UsageFault_handler(void) WEAK_ALIAS;
void SVCall_handler(void) WEAK_ALIAS;
void DebugMonitor_handler(void) WEAK_ALIAS;
void PendSV_handler(void) WEAK_ALIAS;
void SysTick_handler(void) WEAK_ALIAS;
void WWDG_handler(void) WEAK_ALIAS;
void PVD_handler(void) WEAK_ALIAS;
void TAMPER_handler(void) WEAK_ALIAS;
void RTC_handler(void) WEAK_ALIAS;
void FLASH_handler(void) WEAK_ALIAS;
void RCC_handler(void) WEAK_ALIAS;
void EXTI0_handler(void) WEAK_ALIAS;
void EXTI1_handler(void) WEAK_ALIAS;
void EXTI2_handler(void) WEAK_ALIAS;
void EXTI3_handler(void) WEAK_ALIAS;
void EXTI4_handler(void) WEAK_ALIAS;
void DMA1_Channel1_handler(void) WEAK_ALIAS;
void DMA1_Channel2_handler(void) WEAK_ALIAS;
void DMA1_Channel3_handler(void) WEAK_ALIAS;
void DMA1_Channel4_handler(void) WEAK_ALIAS;
void DMA1_Channel5_handler(void) WEAK_ALIAS;
void DMA1_Channel6_handler(void) WEAK_ALIAS;
void DMA1_Channel7_handler(void) WEAK_ALIAS;
void ADC1_2_handler(void) WEAK_ALIAS;
void USB_HP_CAN_TX_handler(void) WEAK_ALIAS;
void USB_LP_CAN_RX0_handler(void) WEAK_ALIAS;
void CAN_RX1_handler(void) WEAK_ALIAS;
void CAN_SCE_handler(void) WEAK_ALIAS;
void EXTI9_5_handler(void) WEAK_ALIAS;
void TIM1_BRK_handler(void) WEAK_ALIAS;
void TIM1_UP_handler(void) WEAK_ALIAS;
void TIM1_TRG_COM_handler(void) WEAK_ALIAS;
void TIM1_CC_handler(void) WEAK_ALIAS;
void TIM2_handler(void) WEAK_ALIAS;
void TIM3_handler(void) WEAK_ALIAS;
void TIM4_handler(void) WEAK_ALIAS;
void I2C1_EV_handler(void) WEAK_ALIAS;
void I2C1_ER_handler(void) WEAK_ALIAS;
void I2C2_EV_handler(void) WEAK_ALIAS;
void I2C2_ER_handler(void) WEAK_ALIAS;
void SPI1_handler(void) WEAK_ALIAS;
void SPI2_handler(void) WEAK_ALIAS;
void USART1_handler(void) WEAK_ALIAS;
void USART2_handler(void) WEAK_ALIAS;
void USART3_handler(void) WEAK_ALIAS;
void EXTI15_10_handler(void) WEAK_ALIAS;
void RTCAlarm_handler(void) WEAK_ALIAS;
void USBWakeup_handler(void) WEAK_ALIAS;
void TIM8_BRK_handler(void) WEAK_ALIAS;
void TIM8_UP_handler(void) WEAK_ALIAS;
void TIM8_TRG_COM_handler(void) WEAK_ALIAS;
void TIM8_CC_handler(void) WEAK_ALIAS;
void ADC3_handler(void) WEAK_ALIAS;
void FSMC_handler(void) WEAK_ALIAS;
void SDIO_handler(void) WEAK_ALIAS;
void TIM5_handler(void) WEAK_ALIAS;
void SPI3_handler(void) WEAK_ALIAS;
void UART4_handler(void) WEAK_ALIAS;
void UART5_handler(void) WEAK_ALIAS;
void TIM6_handler(void) WEAK_ALIAS;
void TIM7_handler(void) WEAK_ALIAS;
void DMA2_Channel1_handler(void) WEAK_ALIAS;
void DMA2_Channel2_handler(void) WEAK_ALIAS;
void DMA2_Channel3_handler(void) WEAK_ALIAS;
void DMA2_Channel4_5_handler(void) WEAK_ALIAS;

// Array of function pointers for handling each interrupt type defined by the
// reference manual (see Table 63)
const vector_entry vectors[] VECTOR_SECTION = {
    // must cast to pointer representation of an integer, then to a function
    // pointer
    (vector_entry)(uintptr_t)&_stack_start,
    Reset_handler,
    NMI_handler,
    HardFault_handler,
    MemManage_handler,
    BusFault_handler,
    UsageFault_handler,
    0,
    0,
    0,
    0,
    SVCall_handler,
    DebugMonitor_handler,
    0,
    PendSV_handler,
    SysTick_handler,
    WWDG_handler,
    PVD_handler,
    TAMPER_handler,
    RTC_handler,
    FLASH_handler,
    RCC_handler,
    EXTI0_handler,
    EXTI1_handler,
    EXTI2_handler,
    EXTI3_handler,
    EXTI4_handler,
    DMA1_Channel1_handler,
    DMA1_Channel2_handler,
    DMA1_Channel3_handler,
    DMA1_Channel4_handler,
    DMA1_Channel5_handler,
    DMA1_Channel6_handler,
    DMA1_Channel7_handler,
    ADC1_2_handler,
    USB_HP_CAN_TX_handler,
    USB_LP_CAN_RX0_handler,
    CAN_RX1_handler,
    CAN_SCE_handler,
    EXTI9_5_handler,
    TIM1_BRK_handler,
    TIM1_UP_handler,
    TIM1_TRG_COM_handler,
    TIM1_CC_handler,
    TIM2_handler,
    TIM3_handler,
    TIM4_handler,
    I2C1_EV_handler,
    I2C1_ER_handler,
    I2C2_EV_handler,
    I2C2_ER_handler,
    SPI1_handler,
    SPI2_handler,
    USART1_handler,
    USART2_handler,
    USART3_handler,
    EXTI15_10_handler,
    RTCAlarm_handler,
    USBWakeup_handler,
    TIM8_BRK_handler,
    TIM8_UP_handler,
    TIM8_TRG_COM_handler,
    TIM8_CC_handler,
    ADC3_handler,
    FSMC_handler,
    SDIO_handler,
    TIM5_handler,
    SPI3_handler,
    UART4_handler,
    UART5_handler,
    TIM6_handler,
    TIM7_handler,
    DMA2_Channel1_handler,
    DMA2_Channel2_handler,
    DMA2_Channel3_handler,
    DMA2_Channel4_5_handler,
};

void Default_handler(void) {
  while (1)
    ;
}

void Reset_handler(void) {
  // Copy .data section to SRAM
  {
    uint32_t size = (uint32_t)&_edata - (uint32_t)&_sdata;
    uint32_t *pDst = (uint32_t *)&_sdata; // SRAM
    uint32_t *pSrc = (uint32_t *)&_etext; // FLASH

    uint32_t num_words = size / sizeof(uint32_t);
    for (uint32_t i = 0; i < num_words; i++) {
      *pDst++ = *pSrc++;
    }
  }

  // Zero out the .bss section to zero in SRAM
  {
    uint32_t size = (uint32_t)&_ebss - (uint32_t)&_sbss;
    uint32_t *pDst = (uint32_t *)&_sbss; // SRAM

    uint32_t num_words = size / sizeof(uint32_t);
    for (uint32_t i = 0; i < num_words; i++) {
      *pDst++ = 0;
    }
  }

  // Call main
  main();
}