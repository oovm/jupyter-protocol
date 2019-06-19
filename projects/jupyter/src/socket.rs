use crate::connection_config::ConnectionConfig;
use crate::errors::Result;
use crate::wire::WireMessage;
use hmac::Mac;
use std::fmt::Debug;
use zmq;

pub(crate) enum SocketType {
    Shell,
    Control,
    IoPub,
    Heartbeat,
}

pub(crate) struct Socket(pub zmq::Socket);

impl Socket {
    pub fn new_shell(ctx: &zmq::Context, config: &ConnectionConfig) -> Result<Socket> {
        let socket = ctx.socket(zmq::REQ)?;
        let conn_str = Socket::connection_string(config, SocketType::Shell);
        socket.connect(&conn_str)?;

        Ok(Socket(socket))
    }

    pub fn new_control(ctx: &zmq::Context, config: &ConnectionConfig) -> Result<Socket> {
        let socket = ctx.socket(zmq::REQ)?;
        let conn_str = Socket::connection_string(config, SocketType::Control);
        socket.connect(&conn_str)?;

        Ok(Socket(socket))
    }

    pub fn new_iopub(ctx: &zmq::Context, config: &ConnectionConfig) -> Result<Socket> {
        let socket = ctx.socket(zmq::SUB)?;
        let conn_str = Socket::connection_string(config, SocketType::IoPub);
        socket.connect(&conn_str)?;
        socket.set_subscribe("".as_bytes())?;

        Ok(Socket(socket))
    }

    pub fn new_heartbeat(ctx: &zmq::Context, config: &ConnectionConfig) -> Result<Socket> {
        let socket = ctx.socket(zmq::REQ)?;
        let conn_str = Socket::connection_string(config, SocketType::Heartbeat);
        socket.connect(&conn_str)?;

        Ok(Socket(socket))
    }

    pub(crate) fn send_wire<M: Mac + Debug>(&self, wire: WireMessage<M>) -> Result<()> {
        let packets = wire.into_packets()?;
        let slices: Vec<_> = packets.iter().map(|v| v.as_slice()).collect();
        self.0.send_multipart(slices.as_slice(), 0)?;
        Ok(())
    }

    pub(crate) fn recv_wire<M: Mac + Debug>(&self, auth: M) -> Result<WireMessage<M>> {
        let raw_response = self.0.recv_multipart(0)?;
        WireMessage::from_raw_response(raw_response, auth.clone())
    }

    pub(crate) fn heartbeat(&self) -> Result<()> {
        let test = vec![];
        self.0.send(&test, 0)?;
        let _msg = self.0.recv_msg(0)?;
        Ok(())
    }

    fn connection_string(config: &ConnectionConfig, socket_type: SocketType) -> String {
        let port = match socket_type {
            SocketType::Shell => config.shell_port,
            SocketType::Control => config.control_port,
            SocketType::IoPub => config.iopub_port,
            SocketType::Heartbeat => config.hb_port,
        };

        format!(
            "{transport}://{ip}:{port}",
            transport = config.transport,
            ip = config.ip,
            port = port
        )
    }
}
