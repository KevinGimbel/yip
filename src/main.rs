/*
 yip - simple, zero-dependency, caller-IP responder server
    Copyright (C) 2020  Kevin Gimbel

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Result, Write};
use std::env;

fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap().to_string();
    let ip: Vec<&str> = addr.split(":").collect();
    let _r = stream.write_fmt(format_args!("{}", ip[0]));
    stream.shutdown(Shutdown::Both).expect("Failed to shutdown stream");
}

fn main() -> Result<()> {
    // Read port from env
    let port = env::var("PORT").unwrap_or_default().parse().unwrap_or(8111);
    // Create TCP listener
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
