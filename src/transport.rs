use std::{net::{Ipv4Addr, SocketAddr, UdpSocket}, os::fd::AsRawFd, usize};
use socket2::{Domain, Protocol, Socket, Type};
use libc;
const BUFFER_SIZE: usize = 1329; // one more than expected to see if we are getting too much data. 

pub fn execute_packet_fn_on_port(
    port: String,
    function: fn(usize, SocketAddr, &[u8; BUFFER_SIZE]) -> Result<(), Box<dyn std::error::Error>>
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::new(
        Domain::IPV4,
        Type::DGRAM,
        Some(Protocol::UDP)
    )?;

    let enable: libc::c_int = 1;
    unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_TIMESTAMPNS,
            &enable as *const _ as *const _,
            std::mem::size_of_val(&enable) as libc::socklen_t,
        );
    }
    
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap(); 
    
    socket.bind(&addr.into())?;
    let udp: std::net::UdpSocket = socket.into();

    let mut buf= [0u8; BUFFER_SIZE];

    loop {
        let (amt, src) = udp.recv_from(&mut buf).expect("Failed to receive data");
        function(amt, src, &buf)?;
    }
}