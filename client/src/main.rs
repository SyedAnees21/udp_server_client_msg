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
    let mut c_index:i32 = 0;
    let mut packets_list: Vec<SeqData> = Vec::new();

    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    socket.connect("127.0.0.1:8888").expect("Could not connect to server");

    let mut buf= [0; 40];
    
    for _i in 0..3{
        socket.recv(&mut buf).expect("Could not get the datagram");
        let json_str = (str::from_utf8(&buf).expect("unable to parse")).trim_matches('\0').to_string();
        
        let recieved_data:SeqData = serde_json::from_str(json_str.as_str()).unwrap();
        println!("{:?}", &recieved_data);
        
        c_index = packet_validation(&recieved_data,c_index, &socket);
        packets_list.push(recieved_data);
        // println!("{}",c_index);
    }
    println!("{} {} {}", packets_list[0].x, packets_list[0].y, packets_list[0].z);
    println!("All Packets {:#?}", packets_list.iter());

}

fn packet_validation(recieved:&SeqData,  mut index:i32, sock:&UdpSocket) -> i32 {
    
    // if recieved.packet_index - index == 1 {
    //     index = recieved.packet_index;
    //     println!("Packet: {} recieved successfuly",index);
    // }else if recieved.packet_index - index == 0 {
    //     println!("duplicate packet recieved");
    // }else {
    //     println!("Number of packet lost: {}", recieved.packet_index-index);
    // }

    match  recieved.packet_index - index {
        1 => {
            index = recieved.packet_index;
            let response = ClientResponse{packet_index:0};
            let res_json = serde_json::to_string(&response).expect("Could not parse response");

            sock.send(res_json.as_bytes()).unwrap();
        },
        0 => {
            println!("Packet {}  recieved twice", recieved.packet_index);
    
        },
        _ => {
            let response = ClientResponse{packet_index:recieved.packet_index-index};
            let res_json = serde_json::to_string(&response).expect("Could not parse response");

            sock.send(res_json.as_bytes()).unwrap();
        }
    }
    index
}