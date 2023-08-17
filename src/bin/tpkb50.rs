#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    // use cortex_m_semihosting::hprintln;
    type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;
    type HidDev = HIDClass<'static, UsbBusType>;
    use hal::{
        gpio::{
            alt::otg_fs::{Dm::PA11, Dp::PA12},
            PinState::Low,
        },
        otg_fs::{UsbBusType, USB},
        prelude::*,
        timer::Event,
    };
    use stm32f4xx_hal as hal;
    use tpkb50::{
        keyboard::Keyboard,
        keycodes::{KeyCode, MouseCode},
        keymatrix::KeyMatrix,
        trackpoint::{
            TrackPoint, RST as TP_RST, SCL as TP_SCL, SDA as TP_SDA,
            SFACTOR_HIGH as TP_SFACTOR_HIGH,
        },
    };
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{generator_prelude::SerializedDescriptor, KeyboardReport, MouseReport},
        hid_class::HIDClass,
    };
    const RSV_MSB1: u8 = MouseCode::BTN1 as u8;
    const RSV_MSB2: u8 = MouseCode::BTN2 as u8;
    const RSV_MSB3: u8 = MouseCode::BTN3 as u8;
    const RSV_WHUP: u8 = MouseCode::BTN4 as u8;
    const RSV_WHDN: u8 = MouseCode::BTN5 as u8;
    const RSV_WHLT: u8 = MouseCode::BTN6 as u8;
    const RSV_WHRT: u8 = MouseCode::BTN7 as u8;
    const RSV_TAB: u8 = KeyCode::Tab as u8;
    const RSV_SPACE: u8 = KeyCode::Space as u8;

    #[local]
    struct Local {
        keyboard: Keyboard,
        matrix: KeyMatrix,
        trackpoint: TrackPoint,
    }

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        hid_kb: HidDev,
        hid_ms: HidDev,
    }

    #[init(local = [
        EP_MEMORY: [u32; 1024] = [0; 1024],
        USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None
    ])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.MHz())
            .sysclk(48.MHz())
            .require_pll48clk()
            .freeze();
        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();
        let gpioc = ctx.device.GPIOC.split();

        let usb = USB {
            usb_global: ctx.device.OTG_FS_GLOBAL,
            usb_device: ctx.device.OTG_FS_DEVICE,
            usb_pwrclk: ctx.device.OTG_FS_PWRCLK,
            pin_dm: PA11(gpioa.pa11.into_alternate()),
            pin_dp: PA12(gpioa.pa12.into_alternate()),
            hclk: clocks.hclk(),
        };

        let p_rst: TP_RST = gpiob.pb7.into_push_pull_output_in_state(Low).erase();
        let p_clk: TP_SCL = gpiob.pb8.into_open_drain_output().erase();
        let p_data: TP_SDA = gpiob.pb9.into_open_drain_output().erase();
        let delay = ctx.core.SYST.delay(&clocks);

        let mut trackpoint = TrackPoint::new(p_clk, p_data, p_rst, delay);
        trackpoint.reset();
        trackpoint.set_sensitivity_factor(TP_SFACTOR_HIGH);
        // default remote mode, stream not work well as expected with tim exti.
        // trackpoint.set_stream_mode();

        *ctx.local.USB_BUS = Some(UsbBusType::new(usb, ctx.local.EP_MEMORY));
        let usb_bus = ctx.local.USB_BUS.as_ref().unwrap();

        let hid_kb = HIDClass::new(usb_bus, KeyboardReport::desc(), 10);
        let hid_ms = HIDClass::new(usb_bus, MouseReport::desc(), 10);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x2023, 0x0610))
            .manufacturer("Custom")
            .product("Trackpoint Keyboard")
            .serial_number("20221010")
            .device_class(0)
            .build();

        let mut timer = ctx.device.TIM3.counter_hz(&clocks);
        timer.start(1.kHz()).unwrap();
        timer.listen(Event::Update);

        let rows = [
            gpiob.pb6.into_pull_down_input().erase(),
            gpiob.pb5.into_pull_down_input().erase(),
            gpiob.pb4.into_pull_down_input().erase(),
            gpiob.pb3.into_pull_down_input().erase(),
        ];
        let cols = [
            gpiob.pb10.into_push_pull_output().erase(),
            gpioc.pc14.into_push_pull_output().erase(),
            gpiob.pb1.into_push_pull_output().erase(),
            gpiob.pb0.into_push_pull_output().erase(),
            gpioa.pa7.into_push_pull_output().erase(),
            gpioa.pa6.into_push_pull_output().erase(),
            gpioa.pa5.into_push_pull_output().erase(),
            gpioa.pa4.into_push_pull_output().erase(),
            gpioa.pa3.into_push_pull_output().erase(),
            gpioa.pa2.into_push_pull_output().erase(),
            gpioa.pa1.into_push_pull_output().erase(),
            gpioa.pa0.into_push_pull_output().erase(),
            gpioc.pc15.into_push_pull_output().erase(),
        ];
        let matrix = cortex_m::interrupt::free(move |_cs| KeyMatrix::new(rows, cols));

        (
            Shared {
                usb_dev,
                hid_kb,
                hid_ms,
            },
            Local {
                matrix,
                keyboard: Keyboard::new(),
                trackpoint,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = OTG_FS, priority = 3, shared = [usb_dev, hid_kb, hid_ms])]
    fn usb_tx(ctx: usb_tx::Context) {
        (ctx.shared.usb_dev, ctx.shared.hid_kb, ctx.shared.hid_ms).lock(
            |usb_dev, hid_kb, hid_ms| {
                if usb_dev.poll(&mut [hid_kb, hid_ms]) {}
            },
        );
    }

    #[task(binds = OTG_FS_WKUP, priority = 3, shared = [usb_dev, hid_kb, hid_ms])]
    fn usb_rx(ctx: usb_rx::Context) {
        (ctx.shared.usb_dev, ctx.shared.hid_kb, ctx.shared.hid_ms)
            .lock(|usb_dev, hid_kb, hid_ms| if usb_dev.poll(&mut [hid_kb, hid_ms]) {});
    }

    #[task(binds = TIM3, priority = 1, shared = [hid_kb, hid_ms], local=[
        matrix, keyboard, trackpoint,
        ms_btn: u8 = 0, ms_wheel: i8 = 0, ms_pan: i8 = 0
    ])]
    fn tick(ctx: tick::Context) {
        (ctx.shared.hid_kb, ctx.shared.hid_ms).lock(|hid_kb, hid_ms| {
            if let Some(kb_report) = ctx
                .local
                .keyboard
                .gen_report(&ctx.local.matrix.current_state())
            {
                hid_kb.push_input(&kb_report).ok();
                match kb_report.reserved {
                    // for mouse wheel key
                    RSV_WHUP => *ctx.local.ms_wheel = 1,
                    RSV_WHDN => *ctx.local.ms_wheel = -1,
                    RSV_WHLT => *ctx.local.ms_pan = -1,
                    RSV_WHRT => *ctx.local.ms_pan = 1,
                    btn @ (RSV_MSB1 | RSV_MSB2 | RSV_MSB3) => *ctx.local.ms_btn = btn,
                    _tap_key @ (RSV_TAB | RSV_SPACE) => {
                        // empirical time
                        cortex_m::asm::delay(500_000);

                        // send empty report, prevent repeat key
                        hid_kb
                            .push_input(&KeyboardReport {
                                modifier: 0,
                                reserved: 0,
                                leds: 0,
                                keycodes: [0; 6],
                            })
                            .ok();
                    }
                    _ => {
                        (*ctx.local.ms_btn, *ctx.local.ms_wheel, *ctx.local.ms_pan) = (0, 0, 0);
                    }
                };
            }
            let tp_data = &ctx.local.trackpoint.query_data_report();
            hid_ms
                .push_input(&MouseReport {
                    x: tp_data.x,
                    y: -tp_data.y,
                    buttons: tp_data.state & 7 | *ctx.local.ms_btn,
                    wheel: *ctx.local.ms_wheel,
                    pan: *ctx.local.ms_pan,
                })
                .ok();
        })
    }
}
