use time::Time;
use chrono::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_6;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc, time::Duration};

    use tokio_modbus::client::{
        rtu,
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    };
    use tokio_modbus::prelude::*;
    use tokio_serial::{Serial, SerialPortSettings};

    const SLAVE_1: Slave = Slave(0x01);
    
    #[derive(Debug)]
    struct SerialConfig {
        path: String,
        settings: SerialPortSettings,
    }

    impl NewContext for SerialConfig {
        fn new_context(&self) -> Pin<Box<dyn Future<Output = Result<Context, Error>>>> {
            let serial = Serial::from_path(&self.path, &self.settings);
            Box::pin(async {
                let port = serial?;
                rtu::connect(port).await
            })
        }
    }

    let serial_config = SerialConfig {
        path: "/dev/ttyAMA0".into(),
        settings: SerialPortSettings {
            baud_rate: 9600,
            timeout: Duration::from_millis(100),
            ..Default::default()
        },
    };
    println!("Configuration: {:?}", serial_config);

    // A shared, reconnectable context is not actually needed in this
    // simple example. Nevertheless we use it here to demonstrate how
    // it works.
    let shared_context = Rc::new(RefCell::new(SharedContext::new(
        None, // no initial context, i.e. not connected
        Box::new(serial_config),
    )));

    reconnect_shared_context(&shared_context).await?;

    assert!(shared_context.borrow().is_connected());
    println!("Connected");

    println!("Reading a sensor value from {:?}", SLAVE_1);
    let context = shared_context.borrow().share_context().unwrap();
    let mut context = context.borrow_mut();
    context.set_slave(SLAVE_1);

    let response = context.read_holding_registers(2193, 2).await?;
    println!("OHCompressor1: {:?}", convert_to_float(&response));

    let context = shared_context.borrow().share_context().unwrap();
    let mut context = context.borrow_mut();
    context.set_slave(SLAVE_1);

    let response = context.read_holding_registers(2193, 2).await?;
    println!("OHCompressor1: {:?}", convert_to_float(&response));

    context.set_slave(SLAVE_1);

    let response = context.read_holding_registers(2353, 2).await?;
    println!("OHDomesticWaterAuxilary: {:?}", convert_to_float(&response));


    Ok(())
}

fn convert_to_float(data: &[u16]) -> f32 {
    let mut buf: [u8; 4] = [0; 4];

    LittleEndian::write_u16(&mut buf, data[0]);
    LittleEndian::write_u16(&mut buf[2..4], data[1]);

    LittleEndian::read_f32(&buf)
}

fn convert_to_int(data: &[u16]) -> u16 {
    data[0]
}

fn convert_to_byte(data: &[u16]) -> u8 {
    (data[0] & 0xff) as u8
}

fn convert_to_time(data: &[u16]) -> Time {
    Time::try_from_hms(
        (data[0] >> 8) as u8,
        (data[0] & 0xff) as u8,
        0)
        .expect("coucou")
}

fn convert_to_datetime(data: &[u16]) -> DateTime<Utc> {
    // !!! careful, this will panic in case of bad value
    Utc.ymd(
        (1900 + data[5]) as i32,
        data[4] as u32,
        data[3] as u32)
        .and_hms(
            data[2] as u32,
            data[1] as u32,
            data[0] as u32)
}

fn convert_to_string(data: &[u16]) -> String {
    let mut buf = String::new();

    for i in 0..data.len() {
        let word = data[i];
        buf.push_str(&ISO_8859_6.decode(
            &[(word & 0xff) as u8, (word >> 8) as u8],DecoderTrap::Replace).expect("Unable to decode string")[..]);

    }

    buf
}

#[test]
fn to_time() {
    let t = convert_to_time(&[1056]);
    assert_eq!(t.hour(), 4);
    assert_eq!(t.minute(), 32);
    assert_eq!(t.second(), 0);
}

#[test]
fn to_datetime() {
    let dt = convert_to_datetime(&[1,2,3, 4, 5, 120]);
    assert_eq!(dt.year(), 2020);
    assert_eq!(dt.month(), 5);
    assert_eq!(dt.day(), 4);
    assert_eq!(dt.hour(), 3);
    assert_eq!(dt.minute(), 2);
    assert_eq!(dt.second(), 1);
}

#[test]
fn to_floats() {
    assert_eq!(convert_to_float(&[29226, 17567]), 1275.5676);
}


#[test]
fn to_int() {
    assert_eq!(convert_to_int(&[29226]), 29226);
}

#[test]
fn to_byte() {
    assert_eq!(convert_to_byte(&[1057]), 33);
}

#[test]
fn to_string() {
    assert_eq!(convert_to_string(&[0x4349]), "IC");
}