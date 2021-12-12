enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

impl Message {
    fn call(&self) {
        //if let Message::Write(some_string) = "hello"{
        //    println!("{}", some_string);    
        }
        
    }


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let home2 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback2 = IpAddrEnum::V6(String::from("::1"));
    println!("Hello, world!");


    let m = Message::Write(String::from("hello"));
    m.call();
}
