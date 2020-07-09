use futures::io;
use futures::prelude::*;

fn main() -> io::Result<()> {
    smol::run(async {
        let settings = mio_serial::SerialPortSettings::default();
        let mut port = mio_serial::Serial::from_path("/dev/ttyACM0", &settings)?;
        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let stdin = smol::reader(std::io::stdin());
        let mut stdout = smol::writer(std::io::stdout());

        let mut io = smol::Async::new(port)?;
        let (port_r, mut port_w) = (&mut io).split();

        future::try_join(io::copy(stdin, &mut port_w), io::copy(port_r, &mut stdout)).await?;

        Ok(())
    })
}
