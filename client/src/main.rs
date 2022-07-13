use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str;

#[derive(Deserialize,Debug)]
struct SeqData {
    x: u8,
    y: u8,
    z: u8,
    packet_index: i32
}

#[derive(Serialize,Debug)]
struct ClientResponse {
    packet_index: i32
}

fn main() {
    
    let mut packets_list: Vec<SeqData> = Vec::new();

    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    socket.connect("127.0.0.1:8888").expect("Could not connect to server");

    let mut buf= [0; 40];

    loop {
        
        let mut c_index:i32 = 0;

        for i in 0..=1 {

            socket.recv(&mut buf).expect("Could not get the datagram");
            let json_str = (str::from_utf8(&buf).expect("unable to parse")).trim_matches('\0').to_string();
            let recieved_data:SeqData = serde_json::from_str(json_str.as_str()).unwrap();
            
            /*
            pushing the data on to the vector 
            */
            packets_list.push(recieved_data);

            /*
            validation call to verify the packets in the list recieved
             */
            c_index = packet_validation( c_index, &socket , &mut packets_list , i); 
        }
        println!("All Packets {:#?}", packets_list.iter());
     }

    
}



fn packet_validation( mut index:i32, sock:&UdpSocket,  list:&mut Vec<SeqData>, i:usize) -> i32 {
    
    match  list[i].packet_index - index {
    
        1 => {                                                           //if all OKAY!
            /*Acknowleding the server packets are intact!*/ 
            index = list[i].packet_index;
                
             let response = ClientResponse{packet_index:0};
             let res_json = serde_json::to_string(&response).expect("Could not parse response");

             sock.send(res_json.as_bytes()).unwrap();
        },
        0 => {                                                           //if the same packet is recieved twice
            println!("Packet {}  recieved twice", list[i].packet_index);
    
        },
        _ => {                                                           //if the certain packet is missing
            /*Requesting the server for the missing packet */
            let mut buf= [0; 40];
    
            let response = ClientResponse{packet_index:list[i].packet_index - index};
            let res_json = serde_json::to_string(&response).expect("Could not parse response");

            sock.send(res_json.as_bytes()).unwrap();
            let len =sock.recv(&mut buf).expect("Could not get the datagram");
            let json_str = (str::from_utf8(&buf[..len]).expect("unable to parse")).to_string();

            let data: SeqData= serde_json::from_str(json_str.as_str()).unwrap();
            list.push(data);
        }
    }
    index
}