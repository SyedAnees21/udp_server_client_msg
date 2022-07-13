// use std::{thread,time};
use std::{net::UdpSocket,
          time::Duration,
          str,
          io::ErrorKind
        };
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize,Debug)]
struct SeqData {
    x: u8,
    y: u8,
    z: u8,
    packet_index: i32
}

#[derive(Deserialize,Debug)]
struct ClientResponse {
    packet_index: i32
}


fn main() {

    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    socket.set_read_timeout(Some(Duration::from_millis(20))).unwrap();
    let remote_adrr = "127.0.0.1:8000";
    
    let mut buf = [0;50];

    let data1 = SeqData {
        x: 1,
        y: 2,
        z: 3,
        packet_index: 1
    };

    let data2 = SeqData {
        x: 4,
        y: 5,
        z: 6,
        packet_index: 2
    };

    let data3 = SeqData {
        x: 7,
        y: 8,
        z: 9,
        packet_index: 3
    };

    let data_packets:Vec<SeqData>=vec![data1,data3,data2];
   
    loop {

        for i in 0..data_packets.len(){

            /*
            Starting to send data pakcets to client
             */
            let json_str = serde_json::to_string(&(data_packets[i])).unwrap();
            println!("Serialized structure for udp: {} of size {} ", json_str, json_str.len());

            socket.send_to(&json_str.as_bytes(), &remote_adrr).expect("Unable to send data!");


            /*
            // Recieving the responses from Client
            //  */
            let result = socket.recv_from(&mut buf);/*.unwrap_or_else(|_|
                {
               return (0, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)); 
               });
               let (bytes , src_addr) = result;
                */

            
            let (bytes , src_addr)= match result {
                                            Ok(res) => res,
                                            Err(e) => match e.kind(){
                                                                ErrorKind::TimedOut => {continue},
                                                                _ =>{
                                                                    println!("{:?}", e);
                                                                    continue;
                                                                }
                                          }
            };   
            
            // let (bytes , src_addr) = socket.recv_from(&mut buf).expect("unable to recieve");
            let msg_frm_client = str::from_utf8(&buf[..bytes])
                                            .expect("No message from client")
                                            .to_string();
            let response_from_client: ClientResponse = serde_json::from_str(&msg_frm_client.as_str()).expect("Unable to parse");
            
            /*
            Serving the Client response after each data packet transmission
             */
            if response_from_client.packet_index != 0 {
                let json_str = serde_json::to_string(&(data_packets[(response_from_client.packet_index) as usize])).unwrap();
                socket.send_to(&json_str.as_bytes(), src_addr).expect("Unable to send data!");
            }
        } 
    }
}


