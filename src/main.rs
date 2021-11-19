#![allow(
non_snake_case,
non_upper_case_globals,
dead_code,
non_camel_case_types,
unused_must_use,
unused_unsafe,
improper_ctypes,
unused_doc_comments
)]

mod APDUResponse;

extern crate pcsc;

use pcsc::*;
use std::io::Write;
use std::{io, str};

fn main() {
    loop_console();
}

fn get_card() -> Card {
    let ctx = Context::establish(Scope::User).expect("Context");
    let mut readers_buf = [0; 2048];
    let mut readers = ctx
        .list_readers(&mut readers_buf)
        .expect("failed to list readers");
    let reader = readers.next().ok_or(()).expect("no readers are connected");
    println!("Using reader: {:?}", reader);

    ctx.connect(reader, ShareMode::Shared, Protocols::ANY)
        .expect("failed to connect to card")
}

static select_main_AID_command: &str =
    "00 A4 04 00 10 A0 00 00 00 77 01 08 00 07 00 00 FE 00 00 01 00";

static select_DF: &str = "00 A4 00 0C 00";
static select_DF_5000: &str = "00 A4 02 0C 02 50 00";
static read_bytes: &str = "00 B0 00 00 00";

fn listTransparentPersonalData() {
    let mut trans_rf: [u8; 7] = [0x00, 0xA4, 0x02, 0x0C, 0x02, 0x50, 0x00]; // <- DF 5000, increment last byte to get EFs 5001-50FF;
    let c = get_card();
    let apdu_select_main_AID_command = &*decode_hex(select_main_AID_command).unwrap();
    let apdu_select_df = &*decode_hex(select_DF).unwrap();
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    c.transmit(apdu_select_main_AID_command, &mut rapdu_buf);
    c.transmit(apdu_select_df, &mut rapdu_buf);

    for i in 0u8..15 {
        trans_rf[trans_rf.len() - 1] = i;
        let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
        c.transmit(&trans_rf, &mut rapdu_buf)
            .expect("failed to transmit APDU to card");
        let rapdu = c
            .transmit(&*decode_hex(read_bytes).unwrap(), &mut rapdu_buf)
            .expect("failed to transmit APDU to card");
        let [sw1, sw2] = [rapdu[rapdu.len() - 2], rapdu[rapdu.len() - 1]];
        print!("[");
        for char in rapdu {
            print!("{:02X?}, ", char);
        }
        print!("] ");
        io::stdout().flush().unwrap();
        for char in &rapdu[..rapdu.len() - 2] {
            print!("{}", *char as char);
        }
        unsafe {
            print!(
                " SW1={} SW2={}: {:?}",
                sw1,
                sw2,
                APDUResponse::translate_response(sw1, sw2)
            );
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    let s2: &str = &*s.replace(" ", "").replace("\n", "");
    if s2.len() % 2 != 0 {
        return Err("string ".to_owned() + s + " does not divide into bytes");
    }

    (0..s2.len())
        .step_by(2)
        .map(|i| {
            let r = u8::from_str_radix(&s2[i..i + 2], 16);
            if r.is_err() {
                Err("Parsing of ".to_owned() + s + " failed")
            } else {
                Ok(r.unwrap())
            }
        })
        .collect()
}

fn loop_console() {
    let mut card = get_card();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match &*input.trim().to_ascii_lowercase() {
            "exit" => {
                std::process::exit(0);
            }

            "card" => {
                card = get_card();
                println!("card refreshed");
            }

            "select main aid" => {
                println!(">>{}", select_main_AID_command);
                let apdu = &*decode_hex(select_main_AID_command).unwrap();
                run_and_output_command(&card, apdu);
            }

            "read bytes" => {
                println!(">>{}", read_bytes);
                let apdu = &*decode_hex(read_bytes).unwrap();
                run_and_output_command(&card, apdu);
            }

            "select df" => {
                println!(">>{}", select_DF);
                let apdu = &*decode_hex(select_DF).unwrap();
                run_and_output_command(&card, apdu);
            }

            "select df 5000" => {
                println!(">>{}", select_DF_5000);
                let apdu = &*decode_hex(select_DF_5000).unwrap();
                run_and_output_command(&card, apdu);
            }

            "get id transparent personal data" => {
                listTransparentPersonalData();
            }

            _ => {
                let apdu = &*decode_hex(input.trim()).expect("Inconvertible.");
                run_and_output_command(&card, apdu);
            }
        }
    }
}

fn run_and_output_command(card: &Card, apdu: &[u8]) {
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];

    let rapdu = card
        .transmit(apdu, &mut rapdu_buf)
        .expect("failed to transmit APDU to card");

    let [sw1, sw2] = [rapdu[rapdu.len() - 2], rapdu[rapdu.len() - 1]];
    print!("<< [");
    for c in rapdu {
        print!("{:02X?}, ", c);
    }
    print!("] ");
    io::stdout().flush().unwrap();
    for c in &rapdu[..rapdu.len() - 2] {
        print!("{}", *c as char);
    }
    unsafe {
        print!(
            " SW1={} SW2={}: {:?}",
            sw1,
            sw2,
            APDUResponse::translate_response(sw1, sw2)
        );
    }
    print!("\n");
    io::stdout().flush().unwrap();
}
