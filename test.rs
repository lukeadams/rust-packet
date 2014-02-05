extern mod pktutil;

use std::io::net::ip::Ipv4Addr;
use std::io::net::ip;

use pktutil::decode_packet;
use pktutil::{UdpPacket,TcpPacket};
use pktutil::{EthernetHeader,Ipv4Header,UdpHeader,TcpFlags,TcpHeader};

use pktutil::{UserDatagram,TCP};
use pktutil::{Ethertype_IP};

#[test]
fn test_decode_udp_packet() {
    let dns_pkt: &[u8] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0x31, 0x27, 0x33, 0x40, 0x00, 0x40, 0x11, 0x8f, 0x37, 0xc0, 0xa8, 0x01, 0x02, 0xc0, 0xa8, 0x01, 0xff, 0xbc, 0xad, 0x7e, 0x9c, 0x00, 0x1d, 0xc9, 0xd9, 0x4d, 0x2d, 0x53, 0x45, 0x41, 0x52, 0x43, 0x48, 0x20, 0x2a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a];

    // 0000   ff ff ff ff ff ff 30 85 a9 40 09 35 08 00 45 00  ......0..@.5..E.
    // 0010   00 31 27 33 40 00 40 11 8f 37 c0 a8 01 02 c0 a8  .1'3@.@..7......
    // 0020   01 ff bc ad 7e 9c 00 1d c9 d9|4d 2d 53 45 41 52  ....~.....M-SEAR
    // 0030   43 48 20 2a 20 48 54 54 50 2f 31 2e 31 0d 0a     CH * HTTP/1.1..

    let expected_payload = [0x4d, 0x2d, 0x53, 0x45, 0x41, 0x52, 0x43, 0x48, 0x20, 0x2a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a];

    let pkt = decode_packet(dns_pkt);
    
    match pkt {
        UdpPacket(eth_hdr, ip_hdr,  udp_hdr,  payload) => {
            let dst_mac = ~[0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
            let src_mac = ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35];
            assert_eq!(eth_hdr.dst_mac, dst_mac);
            assert_eq!(eth_hdr.src_mac, src_mac);
            assert_eq!(eth_hdr.ethertype, Ethertype_IP);

            assert_eq!(ip_hdr.version, 4);
            assert_eq!(ip_hdr.diff_services, 0x00);
            assert_eq!(ip_hdr.total_len, 49);
            assert_eq!(ip_hdr.id, 0x2733);
            assert_eq!(ip_hdr.flags, 0x02);            
            assert_eq!(ip_hdr.frag_offset, 0);
            assert_eq!(ip_hdr.ttl, 64);
            assert_eq!(ip_hdr.protocol, UserDatagram);
            assert_eq!(ip_hdr.checksum, 0x8f37);
            assert_eq!(ip_hdr.src_ip, Ipv4Addr(192, 168, 1, 2));
            assert_eq!(ip_hdr.dst_ip, Ipv4Addr(192, 168, 1, 255));
            
            assert_eq!(udp_hdr.src_port, 48301);
            assert_eq!(udp_hdr.dst_port, 32412);
            assert_eq!(udp_hdr.length, 29);
            assert_eq!(udp_hdr.checksum, 0xc9d9);

            assert_eq!(0, ip_hdr.checksum());
            assert_eq!(0, udp_hdr.ipv4_checksum(ip_hdr.src_ip, ip_hdr.dst_ip, payload));
            
            assert_eq!(payload.len(), 21);
            assert_eq!(payload, expected_payload);
        }, 
        g => { println!("{:?}", g); fail!("wrong packet type to start out with"); }
    }
}

#[test]
fn test_decode_udp_packet2() {
    let dns_pkt: &[u8] = [0x2c, 0xb0, 0x5d, 0x35, 0x46, 0xae, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0x38, 0x12, 0x73, 0x40, 0x00, 0x40, 0x11, 0x56, 0x88, 0xc0, 0xa8, 0x01, 0x02, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x3e, 0x00, 0x35, 0x00, 0x24, 0x55, 0x52, 0xf2, 0x5d, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x6c, 0x04, 0x79, 0x69, 0x6d, 0x67, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01];

    // 0000   2c b0 5d 35 46 ae 30 85 a9 40 09 35 08 00 45 00  ,.]5F.0..@.5..E.
    // 0010   00 38 12 73 40 00 40 11 56 88 c0 a8 01 02 08 08  .8.s@.@.V.......
    // 0020   08 08 3e 3e 00 35 00 24 55 52|f2 5d 01 00 00 01  ..>>.5.$UR.]....
    // 0030   00 00 00 00 00 00 01 6c 04 79 69 6d 67 03 63 6f  .......l.yimg.co
    // 0040   6d 00 00 01 00 01                                m.....
    
    let expected_payload = [0xf2, 0x5d, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x6c, 0x04, 0x79, 0x69, 0x6d, 0x67, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01];

    let pkt = decode_packet(dns_pkt);
    
    match pkt {
        UdpPacket(eth_hdr, ip_hdr,  udp_hdr,  payload) => {
            let dst_mac = ~[0x2c, 0xb0, 0x5d, 0x35, 0x46, 0xae];
            let src_mac = ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35];
            assert_eq!(eth_hdr.dst_mac, dst_mac);
            assert_eq!(eth_hdr.src_mac, src_mac);
            assert_eq!(eth_hdr.ethertype, Ethertype_IP);

            assert_eq!(ip_hdr.version, 4);
            assert_eq!(ip_hdr.diff_services, 0x00);
            assert_eq!(ip_hdr.total_len, 56);
            assert_eq!(ip_hdr.id, 0x1273);
            assert_eq!(ip_hdr.flags, 0x02);            
            assert_eq!(ip_hdr.frag_offset, 0);
            assert_eq!(ip_hdr.ttl, 64);
            assert_eq!(ip_hdr.protocol, UserDatagram);
            assert_eq!(ip_hdr.checksum, 0x5688);
            assert_eq!(ip_hdr.src_ip, Ipv4Addr(192, 168, 1, 2));
            assert_eq!(ip_hdr.dst_ip, Ipv4Addr(8, 8, 8, 8));
            
            assert_eq!(udp_hdr.src_port, 15934);
            assert_eq!(udp_hdr.dst_port, 53);
            assert_eq!(udp_hdr.length, 36);
            assert_eq!(udp_hdr.checksum, 0x5552);

            assert_eq!(0, ip_hdr.checksum());
            assert_eq!(0, udp_hdr.ipv4_checksum(ip_hdr.src_ip, ip_hdr.dst_ip, payload));
            
            assert_eq!(payload.len(), 28);
            assert_eq!(payload, expected_payload);
        }, 
        g => { println!("{:?}", g); fail!("wrong packet type to start out with"); }
    }
}

#[test]
fn test_decode_tcp_packet() {
    let tcp_pkt: &[u8] = [0xd0, 0xe7, 0x82, 0x7b, 0x3d, 0x8c, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0xa7, 0xef, 0xbd, 0x40, 0x00, 0x40, 0x06, 0xc7, 0x3c, 0xc0, 0xa8, 0x01, 0x02, 0xc0, 0xa8, 0x01, 0x04, 0xdc, 0x08, 0x1f, 0x48, 0xee, 0x98, 0x7f, 0xb1, 0x1e, 0xed, 0x95, 0xba, 0x80, 0x18, 0x01, 0x61, 0x66, 0x03, 0x00, 0x00, 0x01, 0x01, 0x08, 0x0a, 0x04, 0xaa, 0xe7, 0x42, 0x00, 0x53, 0xd4, 0xbf, 0x47, 0x45, 0x54, 0x20, 0x2f, 0x73, 0x73, 0x64, 0x70, 0x2f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2d, 0x64, 0x65, 0x73, 0x63, 0x2e, 0x78, 0x6d, 0x6c, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a, 0x55, 0x73, 0x65, 0x72, 0x2d, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x3a, 0x20, 0x55, 0x50, 0x6e, 0x50, 0x2f, 0x31, 0x2e, 0x30, 0x20, 0x44, 0x4c, 0x4e, 0x41, 0x44, 0x4f, 0x43, 0x2f, 0x31, 0x2e, 0x35, 0x30, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x75, 0x6d, 0x2f, 0x31, 0x2e, 0x30, 0x2e, 0x34, 0x2e, 0x31, 0x31, 0x0d, 0x0a, 0x48, 0x6f, 0x73, 0x74, 0x3a, 0x20, 0x31, 0x39, 0x32, 0x2e, 0x31, 0x36, 0x38, 0x2e, 0x31, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x30, 0x38, 0x0d, 0x0a, 0x0d, 0x0a];

    // 0000   d0 e7 82 7b 3d 8c 30 85 a9 40 09 35 08 00 45 00  ...{=.0..@.5..E.
    // 0010   00 a7 ef bd 40 00 40 06 c7 3c c0 a8 01 02 c0 a8  ....@.@..<......
    // 0020   01 04 dc 08 1f 48 ee 98 7f b1 1e ed 95 ba 80 18  .....H..........
    // 0030   01 61 66 03 00 00 01 01 08 0a 04 aa e7 42 00 53  .af..........B.S
    // 0040   d4 bf 47 45 54 20 2f 73 73 64 70 2f 64 65 76 69  ..GET /ssdp/devi
    // 0050   63 65 2d 64 65 73 63 2e 78 6d 6c 20 48 54 54 50  ce-desc.xml HTTP
    // 0060   2f 31 2e 31 0d 0a 55 73 65 72 2d 41 67 65 6e 74  /1.1..User-Agent
    // 0070   3a 20 55 50 6e 50 2f 31 2e 30 20 44 4c 4e 41 44  : UPnP/1.0 DLNAD
    // 0080   4f 43 2f 31 2e 35 30 20 50 6c 61 74 69 6e 75 6d  OC/1.50 Platinum
    // 0090   2f 31 2e 30 2e 34 2e 31 31 0d 0a 48 6f 73 74 3a  /1.0.4.11..Host:
    // 00a0   20 31 39 32 2e 31 36 38 2e 31 2e 34 3a 38 30 30   192.168.1.4:800
    // 00b0   38 0d 0a 0d 0a                                   8.... 

    // 474554202f737364702f6465766963652d646573632e786d6c20485454502f312e310d0a557365722d4167656e743a2055506e502f312e3020444c4e41444f432f312e353020506c6174696e756d2f312e302e342e31310d0a486f73743a203139322e3136382e312e343a383030380d0a0d0a
    let expected_payload = [0x47, 0x45, 0x54, 0x20, 0x2f, 0x73, 0x73, 0x64, 0x70, 0x2f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2d, 0x64, 0x65, 0x73, 0x63, 0x2e, 0x78, 0x6d, 0x6c, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a, 0x55, 0x73, 0x65, 0x72, 0x2d, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x3a, 0x20, 0x55, 0x50, 0x6e, 0x50, 0x2f, 0x31, 0x2e, 0x30, 0x20, 0x44, 0x4c, 0x4e, 0x41, 0x44, 0x4f, 0x43, 0x2f, 0x31, 0x2e, 0x35, 0x30, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x75, 0x6d, 0x2f, 0x31, 0x2e, 0x30, 0x2e, 0x34, 0x2e, 0x31, 0x31, 0x0d, 0x0a, 0x48, 0x6f, 0x73, 0x74, 0x3a, 0x20, 0x31, 0x39, 0x32, 0x2e, 0x31, 0x36, 0x38, 0x2e, 0x31, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x30, 0x38, 0x0d, 0x0a, 0x0d, 0x0a];
    let pkt = decode_packet(tcp_pkt);
    
    match pkt {
        TcpPacket(eth_hdr, ip_hdr,  tcp_hdr,  payload) => {
            let dst_mac = ~[0xd0, 0xe7, 0x82, 0x7b, 0x3d, 0x8c];
            let src_mac = ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35];
            assert_eq!(eth_hdr.dst_mac, dst_mac);
            assert_eq!(eth_hdr.src_mac, src_mac);
            assert_eq!(eth_hdr.ethertype, Ethertype_IP);

            assert_eq!(ip_hdr.version, 4);
            assert_eq!(ip_hdr.diff_services, 0x00);
            assert_eq!(ip_hdr.ecn, 0x00);
            assert_eq!(ip_hdr.total_len, 167);
            assert_eq!(ip_hdr.id, 0xefbd);
            assert_eq!(ip_hdr.flags, 0x02);            
            assert_eq!(ip_hdr.frag_offset, 0);
            assert_eq!(ip_hdr.ttl, 64);
            assert_eq!(ip_hdr.protocol, TCP);
            assert_eq!(ip_hdr.checksum, 0xc73c);
            assert_eq!(ip_hdr.src_ip, Ipv4Addr(192, 168, 1, 2));
            assert_eq!(ip_hdr.dst_ip, Ipv4Addr(192, 168, 1, 4));
            
            assert_eq!(tcp_hdr.src_port, 56328);
            assert_eq!(tcp_hdr.dst_port, 8008);
            assert_eq!(tcp_hdr.seq_num, 4002971569);
            assert_eq!(tcp_hdr.ack_num, 518886842);
            assert_eq!(tcp_hdr.data_offset, 8);

            assert_eq!(tcp_hdr.flags.ns, false);
            assert_eq!(tcp_hdr.flags.cwr, false);
            assert_eq!(tcp_hdr.flags.ece, false);
            assert_eq!(tcp_hdr.flags.urg, false);
            assert_eq!(tcp_hdr.flags.ack, true);
            assert_eq!(tcp_hdr.flags.psh, true);
            assert_eq!(tcp_hdr.flags.rst, false);
            assert_eq!(tcp_hdr.flags.syn, false);
            assert_eq!(tcp_hdr.flags.fin, false);

            assert_eq!(tcp_hdr.window_size, 353);
            assert_eq!(tcp_hdr.checksum, 0x6603);
            assert_eq!(tcp_hdr.urgent_ptr, 0x0000);
            assert_eq!(tcp_hdr.options, ~[1u8, 1u8, 8u8, 10u8, 4u8, 170u8, 231u8, 66u8, 0u8, 83u8, 212u8, 191u8]);
            
            assert_eq!(payload.len(), 115);
            assert_eq!(payload, expected_payload);
        }, 
        g => { println!("{:?}", g); fail!("wrong packet type to start out with"); }
    }
}

#[test]
fn test_encode_udp() {
    let expected: ~[u8] = ~[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0x31, 0x27, 0x33, 0x40, 0x00, 0x40, 0x11, 0x8f, 0x37, 0xc0, 0xa8, 0x01, 0x02, 0xc0, 0xa8, 0x01, 0xff, 0xbc, 0xad, 0x7e, 0x9c, 0x00, 0x1d, 0xc9, 0xd9, 0x4d, 0x2d, 0x53, 0x45, 0x41, 0x52, 0x43, 0x48, 0x20, 0x2a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a];

    let payload: &[u8] = [0x4d, 0x2d, 0x53, 0x45, 0x41, 0x52, 0x43, 0x48, 0x20, 0x2a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a];

    let eth_hdr = EthernetHeader{
        dst_mac:   ~[0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        src_mac:   ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35],
        ethertype: Ethertype_IP,
    };

    let mut udp_hdr = UdpHeader{
        src_port:  48301,
        dst_port:  32412,
        length:    00,
        checksum:  0x0000, // remove this cheat
    };

    udp_hdr.length = (payload.len() + 8) as u16;

    let mut ip_hdr = Ipv4Header{
        version:       4,
        diff_services: 0x00,
        ecn:           0x00,
        total_len:     00,
        id:            0x2733,
        flags:         0x02,
        frag_offset:   0,
        ttl:           64,
        checksum:      0x0000, // remove this cheat
        src_ip:        Ipv4Addr(192, 168, 1, 2),
        dst_ip:        Ipv4Addr(192, 168, 1, 255),
        ihl:           5,
        protocol:      UserDatagram,
        options:       ~[],
    };

    ip_hdr.total_len = 20u16 + udp_hdr.len() as u16 + payload.len() as u16;
    
    ip_hdr.checksum = ip_hdr.checksum();
    assert_eq!(ip_hdr.checksum, 0x8f37); // fix
    
    udp_hdr.checksum = udp_hdr.ipv4_checksum(ip_hdr.src_ip, ip_hdr.dst_ip, payload);
    assert_eq!(udp_hdr.checksum, 0xc9d9);
    // (left: `51678u16`, right: `51673u16`)

    let mut res_bytes = eth_hdr.as_bytes();
    res_bytes.push_all(ip_hdr.as_bytes());
    res_bytes.push_all(udp_hdr.as_bytes());
    res_bytes.push_all(payload);

    assert_eq!(res_bytes, expected); // make expected &[u8] and then what here?
}


#[test]
fn test_encode_udp2() {
    let expected: ~[u8] = ~[ 0x2c, 0xb0, 0x5d, 0x35, 0x46, 0xae, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0x38, 0x12, 0x73, 0x40, 0x00, 0x40, 0x11, 0x56, 0x88, 0xc0, 0xa8, 0x01, 0x02, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x3e, 0x00, 0x35, 0x00, 0x24, 0x55, 0x52, 0xf2, 0x5d, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x6c, 0x04, 0x79, 0x69, 0x6d, 0x67, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01 ];
    let payload: &[u8] = [ 0xf2, 0x5d, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x6c, 0x04, 0x79, 0x69, 0x6d, 0x67, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01];

    let eth_hdr = EthernetHeader{
        dst_mac:   ~[0x2c, 0xb0, 0x5d, 0x35, 0x46, 0xae],
        src_mac:   ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35],
        ethertype: Ethertype_IP,
    };

    let mut udp_hdr = UdpHeader{
        src_port:  15934,
        dst_port:  53,
        length:    36,
        checksum:  0x0000, // remove this cheat
    };

    udp_hdr.length = (payload.len() + 8) as u16;

    let mut ip_hdr = Ipv4Header{
        version:       4,
        diff_services: 0x00,
        ecn:           0x00,
        total_len:     56,
        id:            0x1273,
        flags:         0x02,
        frag_offset:   0,
        ttl:           64,
        checksum:      0x0000, // remove this cheat
        src_ip:        Ipv4Addr(192, 168, 1, 2),
        dst_ip:        Ipv4Addr(8, 8, 8, 8),
        ihl:           5,
        protocol:      UserDatagram,
        options:       ~[],
    };

    ip_hdr.total_len = 20u16 + udp_hdr.len() as u16 + payload.len() as u16;
    
    ip_hdr.checksum = ip_hdr.checksum();
    assert_eq!(ip_hdr.checksum, 0x5688);

    udp_hdr.checksum = udp_hdr.ipv4_checksum(ip_hdr.src_ip, ip_hdr.dst_ip, payload);
    assert_eq!(udp_hdr.checksum, 0x5552);

    let mut res_bytes = eth_hdr.as_bytes();
    res_bytes.push_all(ip_hdr.as_bytes());
    res_bytes.push_all(udp_hdr.as_bytes());
    res_bytes.push_all(payload);

    assert_eq!(res_bytes, expected); // make expected &[u8] and then what here?
}
/*
#[test]
fn test_encode_tcp() {
    let expected: ~[u8] = ~[0xd0, 0xe7, 0x82, 0x7b, 0x3d, 0x8c, 0x30, 0x85, 0xa9, 0x40, 0x09, 0x35, 0x08, 0x00, 0x45, 0x00, 0x00, 0xa7, 0xef, 0xbd, 0x40, 0x00, 0x40, 0x06, 0xc7, 0x3c, 0xc0, 0xa8, 0x01, 0x02, 0xc0, 0xa8, 0x01, 0x04, 0xdc, 0x08, 0x1f, 0x48, 0xee, 0x98, 0x7f, 0xb1, 0x1e, 0xed, 0x95, 0xba, 0x80, 0x18, 0x01, 0x61, 0x66, 0x03, 0x00, 0x00, 0x01, 0x01, 0x08, 0x0a, 0x04, 0xaa, 0xe7, 0x42, 0x00, 0x53, 0xd4, 0xbf, 0x47, 0x45, 0x54, 0x20, 0x2f, 0x73, 0x73, 0x64, 0x70, 0x2f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2d, 0x64, 0x65, 0x73, 0x63, 0x2e, 0x78, 0x6d, 0x6c, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a, 0x55, 0x73, 0x65, 0x72, 0x2d, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x3a, 0x20, 0x55, 0x50, 0x6e, 0x50, 0x2f, 0x31, 0x2e, 0x30, 0x20, 0x44, 0x4c, 0x4e, 0x41, 0x44, 0x4f, 0x43, 0x2f, 0x31, 0x2e, 0x35, 0x30, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x75, 0x6d, 0x2f, 0x31, 0x2e, 0x30, 0x2e, 0x34, 0x2e, 0x31, 0x31, 0x0d, 0x0a, 0x48, 0x6f, 0x73, 0x74, 0x3a, 0x20, 0x31, 0x39, 0x32, 0x2e, 0x31, 0x36, 0x38, 0x2e, 0x31, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x30, 0x38, 0x0d, 0x0a, 0x0d, 0x0a];

    let payload: ~[u8] = ~[0x47, 0x45, 0x54, 0x20, 0x2f, 0x73, 0x73, 0x64, 0x70, 0x2f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2d, 0x64, 0x65, 0x73, 0x63, 0x2e, 0x78, 0x6d, 0x6c, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a, 0x55, 0x73, 0x65, 0x72, 0x2d, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x3a, 0x20, 0x55, 0x50, 0x6e, 0x50, 0x2f, 0x31, 0x2e, 0x30, 0x20, 0x44, 0x4c, 0x4e, 0x41, 0x44, 0x4f, 0x43, 0x2f, 0x31, 0x2e, 0x35, 0x30, 0x20, 0x50, 0x6c, 0x61, 0x74, 0x69, 0x6e, 0x75, 0x6d, 0x2f, 0x31, 0x2e, 0x30, 0x2e, 0x34, 0x2e, 0x31, 0x31, 0x0d, 0x0a, 0x48, 0x6f, 0x73, 0x74, 0x3a, 0x20, 0x31, 0x39, 0x32, 0x2e, 0x31, 0x36, 0x38, 0x2e, 0x31, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x30, 0x38, 0x0d, 0x0a, 0x0d, 0x0a];

    let eth_hdr = EthernetHeader{
        dst_mac: ~[0xd0, 0xe7, 0x82, 0x7b, 0x3d, 0x8c],
        src_mac: ~[0x30, 0x85, 0xa9, 0x40, 0x09, 0x35],
        ethertype: Ethertype_IP,
    };

    let mut ip_hdr = Ipv4Header{
        version:       4,
        diff_services: 0x00,
        ecn:           0x00,
        total_len:     167,
        id:            0xefbd,
        flags:         0x02,
        frag_offset:   0,
        ttl:           64,
        checksum:      0x0000, // remove this cheat
        src_ip:        Ipv4Addr(192, 168, 1, 2),
        dst_ip:        Ipv4Addr(192, 168, 1, 4),
        ihl:           5,
        protocol:      TCP,
        options:       ~[],
    };

    ip_hdr.length = 20 + payload.len();
    
    ip_hdr.checksum = ip_hdr.checksum();

    let tcp_flags = TcpFlags{
        ns: false,
        cwr: false,
        ece: false,
        urg: false,
        ack: true,
        psh: true,
        rst: false,
        syn: false,
        fin: false,
    };

    let tcp_hdr = TcpHeader{
        src_port:     56328 as ip::Port,
        dst_port:     8008 as ip::Port,
        seq_num:       4002971569,
        ack_num:       518886842,
        data_offset:   8,
        flags:        tcp_flags,
        window_size:  353,
        checksum:     0x6603,
        urgent_ptr:   0x0000,
        options:      ~[1u8, 1u8, 8u8, 10u8, 4u8, 170u8, 231u8, 66u8, 0u8, 83u8, 212u8, 191u8],
    };

    let mut res_bytes = eth_hdr.as_bytes();
    res_bytes.push_all(ip_hdr.as_bytes());
    res_bytes.push_all(tcp_hdr.as_bytes());
    res_bytes.push_all(payload);

    assert_eq!(res_bytes, expected); // make expected &[u8] and then what here?
}
*/