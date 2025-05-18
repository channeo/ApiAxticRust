use std::env;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS:String = set_address();
    pub static ref PORT : u16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()    
    
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse().unwrap()
}

// -> mục đích để ghi nhớ địa chỉ và port của server, chúng ta sẽ sử dụng trong các file khác mà không cần phải gọi lại hàm set_address() và set_port() nhiều lần