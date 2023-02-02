use std::time::Duration;

// PeakTech 8005
const PT8005_BAUD: u32 = 9600;
const PT8005_INDICATOR_DATA: u8 = 0x0D;
const PT8005_TIMEOUT: u64 = 6000;

fn _list_available_ports(){
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

fn main() {
    // let port_peaktech = "/dev/tty.usbserial-0001";
    let port_peaktech = "COM3";
    let mut port = serialport::new(port_peaktech, PT8005_BAUD)
        .timeout(Duration::from_millis(PT8005_TIMEOUT))
        .open().expect("Failed to open port");

    loop {
        // read byte from serial port
        let mut serial_buf: Vec<u8> = vec![0; 1];
        port.read(serial_buf.as_mut_slice()).expect("Found no data!");
        // I do not wait for a sync-byte, I just want to read dBA/dBC data :)
        // check whether data indicator byte was found then read two more bytes
        if serial_buf[0] == PT8005_INDICATOR_DATA {
            let mut serial_buf_b1: Vec<u8> = vec![0; 1];
            port.read(serial_buf_b1.as_mut_slice()).expect("Found no data!");
            let mut serial_buf_b2: Vec<u8> = vec![0; 1];
            port.read(serial_buf_b2.as_mut_slice()).expect("Found no data!");
            // byte1 to float
            let value1: f32 = (serial_buf_b1[0] * 10).into();
            // byte 2 - extract bit 5-8
            let value2: f32 = (serial_buf_b2[0] >> 4).into();
            //byte 2 - extract bit 1-4
            let value3: f32 = (serial_buf_b2[0] & 15).into();
            // assemble output from parts
            let output: f32 = value1 + value2 + (value3 / 10.0);
            println!("Output {:?}", output );
        }
    }
}
