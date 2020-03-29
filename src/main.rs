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
    let response = context.read_holding_registers(2353, 2).await?;
    println!("Sensor value for device {:?} is: {:?}", SLAVE_1, response);


    Ok(())
}
