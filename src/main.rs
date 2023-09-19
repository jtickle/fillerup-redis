extern crate rand;
extern crate redis;

use crate::redis::Commands;
use crate::rand::Rng;

use std::collections::HashMap;

//static KEYSIZE:i32 = 128;
static VALSIZELOW:i32 = 1048576;
static VALSIZEHIGH:i32 = 268435456;

static ASCIILEN:i32 = 94;
static ASCIILOW:i32 = 33;

fn genrandstr(len: i32, rng: &mut rand::rngs::ThreadRng) -> String {
    let data = (0..len)
        .map(|_| u8::try_from(rng.gen_range(0..ASCIILEN) + ASCIILOW).unwrap())
        .collect::<Vec<u8>>();

    return String::from_utf8(data).unwrap();
}

fn genlongstr(len: i32, rng: &mut rand::rngs::ThreadRng) -> String {
    let n: u8 = rng.gen_range(0..ASCIILEN).try_into().expect("Could not fix an i32 into a u8");

    let data = (0..len)
        .map(|i| u8::try_from(((i32::from(n) + i) % ASCIILEN) + ASCIILOW).unwrap())
        .collect::<Vec<u8>>();

    return String::from_utf8(data).unwrap();
}

fn connect() -> redis::Connection {
    redis::Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn display_mem_info(con: &mut redis::Connection) {
    let output: String = redis::cmd("INFO")
        .query(con)
        .expect("failed to execute INFO");

    let data: HashMap<&str,&str> = output
        .split("\r\n")
        .filter(|line| line.contains(":"))
        .map(|line| {
            let v :Vec<&str> = line.split(":").collect::<Vec<&str>>().try_into().unwrap();
            (v[0],v[1])
        }).collect();

    // At this point, data is a HashMap of what you'd normally get out of INFO

    println!("Memory usage: {}", data["used_memory_human"]);
    println!("Evictions: {}", data["evicted_keys"]);
}

fn main() {
    let mut con = connect();
    let mut rng = rand::thread_rng();

    let mut n = 0;

    loop {
        let k: String = format!("{}", n);
        let l: i32 = rng.gen_range(VALSIZELOW..VALSIZEHIGH);
        let v: String = genlongstr(l, &mut rng);
        let _ : () = con.set(k.clone(), v)
            .expect(&format!("Failed to set key {}", k));

        println!("Wrote key {} with length {}", k, l);
        display_mem_info(&mut con);
        n += 1;
    }
}
