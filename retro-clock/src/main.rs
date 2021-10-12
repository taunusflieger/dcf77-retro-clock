#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]
#![no_main]

extern crate heapless;

//use crate::stm32f4xx_hal::i2c::I2c;
// use datetime_converter::DCF77DateTimeConverter;
//use dcf77_decoder::DCF77Decoder;
use panic_rtt_target as _;
use rtic::app;

//use time_display::display_error;

const DISP_I2C_ADDR: u8 = 0x70;
#[app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::gpio::{gpioa, gpioc, Edge, Input, Output, PullUp, PushPull, AF4};

    use stm32f4xx_hal::gpio::ExtiPin;
    use stm32f4xx_hal::gpio::GpioExt;
    use stm32f4xx_hal::gpio::PinExt;
    use stm32f4xx_hal::i2c::I2c;
    use stm32f4xx_hal::pac;
    use stm32f4xx_hal::pac::DWT;
    use stm32f4xx_hal::prelude::_stm32f4xx_hal_syscfg_SysCfgExt;
    use stm32f4xx_hal::rcc::{Clocks, RccExt};
    use stm32f4xx_hal::time::U32Ext;

    #[shared]
    struct Shared {
        val: u16,
        #[lock_free]
        synchronized: bool,
    }

    #[local]
    struct Local {
        dcf_pin: gpioa::PAn<Input<PullUp>>,
        debug_pin: gpioc::PCn<Output<PushPull>>,
    }

    /// Helper for setting up the clocks on the board
    pub fn setup_clocks(rcc: pac::RCC) -> Clocks {
        // Constrain clock registers
        let rcc = rcc.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(12.mhz())
            .require_pll48clk()
            .sysclk(168.mhz())
            .hclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())
            .freeze();

        assert!(clocks.is_pll48clk_valid());

        clocks
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let mut core = cx.core;
        let device = cx.device;

        // Initialize (enable) the monotonic timer (CYCCNT)
        core.DCB.enable_trace();
        // required on Cortex-M7 devices that software lock the DWT (e.g. STM32F7)
        // ooo
        DWT::unlock();
        core.DWT.enable_cycle_counter();

        // semantically, the monotonic timer is frozen at time "zero" during `init`
        // NOTE do *not* call `Instant::now` in this context; it will return a nonsense value
        //      let now = cx.start; // the start time of the system

        let clocks = setup_clocks(device.RCC);
        let mut syscfg = device.SYSCFG.constrain();
        let mut exti = device.EXTI;
        //let mut pwr = device.PWR;

        //let gpiob = device.GPIOB.split();
        //let scl = gpiob.pb6.into_alternate().set_open_drain();
        //let sda = gpiob.pb7.into_alternate().set_open_drain();
        //let i2c = I2c::new(device.I2C1, (scl, sda), 400.khz(), clocks);

        // Configure input pin for DCF77 signal as interrup source
        let gpioa = device.GPIOA.split();
        let mut pin = gpioa.pa6.into_pull_up_input().erase_number();
        pin.make_interrupt_source(&mut syscfg);
        pin.trigger_on_edge(&mut exti, Edge::RisingFalling);
        pin.enable_interrupt(&mut exti);

        // Use this pin for debugging decoded signal state with oscilloscope
        let gpioc = device.GPIOC.split();
        let output_pin = gpioc.pc6.into_push_pull_output().erase_number();

        // Schedule the blinking task
        // cx.schedule.blink(cx.start + CYCLE_HZ.cycles()).unwrap();

        //let mut timer = Timer::new(device.TIM2, &clocks).start_count_down(100.hz());
        //timer.listen(Event::TimeOut);
        // let rtc = Rtc::new(device.RTC, 255, 127, false, &mut pwr);
        rprintln!("Init successful");

        (
            Shared {
                val: 0,
                synchronized: false,
            },
            Local {
                dcf_pin: pin,
                debug_pin: output_pin,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = EXTI9_5, priority=2, local=[dcf_pin, debug_pin])]
    fn dcf77_signal_change(cx: dcf77_signal_change::Context) {
        let debug_pin = cx.local.debug_pin;

        cx.local.dcf_pin.clear_interrupt_pending_bit();
        if !cx.local.dcf_pin.check_interrupt() {
            return;
        }

        //let now = Instant::now();

        /*
              let res = cx
                  .resources
                  .decoder
                  .register_transition(dcf_pin.is_high(), now, debug_pin);
              if let Err(e) = res {
                  rprintln!("Err: {:?}", e);
              }
        */
    }
}
