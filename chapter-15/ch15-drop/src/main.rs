struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data {}.", self.data);
    }
}

fn main() {
    //println!("Hello, world!");
    let c = CustomerSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomerSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomerSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomerSmartPointer created");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}
