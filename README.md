# bno085
Inspired by [bno080](https://github.com/tstellanova/bno080) library but rebuilt from scratch and different (although also not optimal) architecture.
Allows for more control which helps due to our BNO085 being a bit unstable and we need to handle resets:

```
    let rpi_interface = rppal::i2c::I2c::new().unwrap();
    let interface = I2CInterface::new(rpi_interface);
    let mut driver = BnoDriver::new(interface);
    driver.setup();
    driver.soft_reset().unwrap();

    loop {
       match driver.receive_packet() {
        Ok(res) => match res {
            bno085::bno_packet::BnoPacket::ChannelExec(ce) => match ce {
                bno085::bno_packet::ChannelExecutableData::ResetComplete => {
                    // Enable reports after reset
                    driver.enable_report(bno_constants::SENSOR_REPORTID_ACCEL, 50, 49).unwrap();
                    driver.enable_report(bno_constants::SENSOR_REPORTID_GYRO_CALIBRATED, 50, 49).unwrap();
                },
                bno085::bno_packet::ChannelExecutableData::Unknown(_) => {},
            },
            bno085::bno_packet::BnoPacket::SensorReports(reports) => {
                println!("Reports: {:?}", reports.len());
            },
            _ => {}
        },
        Err(err) => println!("Error {:?}", err)
       }
        sleep(Duration::from_millis(50));
    }

```
