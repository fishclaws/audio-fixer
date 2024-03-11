use std::fmt::format;
use std::process::Command;

use rodio::*;
use rodio::cpal::Host;
use rodio::cpal::traits::{HostTrait,DeviceTrait};
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple};
  

fn main() {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    let mut found_device: String = "".to_owned();
    let mut dev_name: String = "".to_owned();
    for device in devices{
        let dev:rodio::Device = device.into();
        dev_name = dev.name().unwrap();
        let paren_start = dev_name.chars().position(|c| c == '(').unwrap();
        let paren_end = dev_name.chars().position(|c| c == ')').unwrap();
        let ss1 = dev_name.chars();
        let ss: String = ss1.skip(paren_start + 1).take(paren_end - paren_start - 1).collect();
        let index = ss.find(" Hands-Free AG Audio");
        if let Some(found_index) = index {
            found_device = ss.chars().take(found_index).collect();
            break;
        }
   }
   if !found_device.is_empty() {
    println!("\nFound device {}", found_device);
        switch_from_to(found_device, dev_name, host)
   } else {
    println!("Couldn't find Device");
   }
}

fn switch_from_to(found_device: String, dev_name: String, host: Host) {
    switch_to(dev_name.clone());
    let devices2: std::iter::Filter<Devices, fn(&Device) -> bool> = host.output_devices().unwrap();
    for device in devices2 {
        let dev:rodio::Device = device.into();
        let corrected_device = dev.name().unwrap();
        if dev_name == corrected_device {
            continue;
        }
        let index = corrected_device.find(found_device.as_str());
        if let Some(found_index) = index {
            switch_to(corrected_device)
        }
    }
}

fn switch_to(corrected_device: String) {
    println!("Found corrected device {}", corrected_device);
    let output = Command::new("Powershell")
        .args(&["/C", "Get-AudioDevice", "-list", "|", "Select", "ID,", "Name", "|", "Where", "Name", "-eq", &format!("\"{}\"", corrected_device), "|", "ft", "-hidetableheaders"])
        .output()
        .expect("failed to execute process");
    for out in String::from_utf8(output.stdout).iter() {
        println!("{}", out);
        let c = out;
        let ids: Vec<&str> = c.split(" ").collect();
        let id = ids[0].strip_prefix("\r\n");
        if let Some(id2) = id {
            println!("{}", id2);
            let output2 = Command::new("Powershell")
            .args(&["/C", "Set-AudioDevice", "-ID", &format!("\"{}\"", id2)])
            .output()
            .expect("failed to execute process");
            for out2 in String::from_utf8(output2.stdout).iter() {
                println!("{}", out2);
            }
        }
    }
}
