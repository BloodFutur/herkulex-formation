#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use drs_0x01::Rotation::Clockwise;

#[allow(unused_imports)]
use panic_halt;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::Rcc;
use stm32f1xx_hal::rcc::RccExt;

extern crate drs_0x01;
extern crate herkulex_drs_0x01_stm32f1xx;

use stm32f1xx_hal::{
    pac,
    serial::{Config, Serial},
};

use herkulex_drs_0x01_stm32f1xx::*;
use herkulex_drs_0x01_stm32f1xx::communication::Communication;
use herkulex_drs_0x01_stm32f1xx::motors::Motors;

#[entry]
fn main() -> ! {
// Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp: Peripherals = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for

    // us: First get a handle to the RCC peripheral:
    let rcc: Rcc = dp.RCC.constrain();
    // Now we have access to the RCC's registers. The GPIOC can be enabled in
    // RCC_APB2ENR (Prog. Ref. Manual 8.3.7), therefore we must pass this
    // register to the `split` function.
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin C13
    // is usually connected to the Bluepills onboard LED.

    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();

    // let sys_clock = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let clocks_serial = rcc.cfgr.freeze(&mut flash.acr);

    // USART1 on Pins A9 and A10
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);

    let pin_rx = gpioa.pa10;

    let serial = Serial::usart1(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()), // baud rate defined in herkulex doc : 115200
        clocks_serial.clone(),
        // &mut rcc.apb2,
    );

    // separate into tx and rx channels
    let (mut tx, rx) = serial.split();
    let mut delay = cp.SYST.delay(&clocks_serial);



    // La communication est une communication USART utilisant 2 ports : RX et TX
    // - TX est le port utilisé pour transmettre des informations
    // - RX est le port utilise pour recevoir des informations
    let communication = Communication::new(&mut tx, rx);
    // hprintln!("Communication créée");

    let motors = Motors::new(communication);
    // hprintln!("Motors créé");

    let motorA = motors.new_motor(0x0A);
    // id = 0xA
    // hprintln!("MotorA créé");

    let motor2 = motors.new_motor(0x02);
    // id = 2
    // hprintln!("Motor2 créé");
    
    let motorB = motors.new_motor(0x0B);
    let motor1 = motors.new_motor(0x01);
    let motor6 = motors.new_motor(0x06);

    motorA.reboot();
    motor2.reboot();
    motor1.reboot();
    motorB.reboot();
    motor6.reboot();
    // hprintln!("servos redémarrés");
    delay.delay_ms(400_u16);



    // Afin de faire tourner les servos il est nécessaire d'activer le torque
    // Dans le cas contraire les servos ne tourneront pas
    motorA.enable_torque();
    motor2.enable_torque();
    motor1.enable_torque();
    motorB.enable_torque();
    motor6.enable_torque();
    // hprintln!("servos torque activé");
    delay.delay_ms(100_u16);

    motorA.clear_errors();
    motor2.clear_errors();
    motor1.clear_errors();
    motorB.clear_errors();
    motor6.clear_errors();
    delay.delay_ms(400_u16);

    loop {
        // Récupère la température des deux servomoteurs et l'affiche dans la console de debug
        // let t0 = motorA.get_temperature();
        // hprintln!("temp servo0 : {:?}", t0);
        // let t2 = motor2.get_temperature();
        // hprintln!("temp servo2 : {:?}", t2);


        motor1.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);
        motor2.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);

        motor6.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);

        motorA.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);

        motorB.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);

        motor1.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);
        motor2.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);

        motor6.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);

        motorA.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);

        motorB.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);

    }
}
