#![no_mangle]

use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use std::collections::HashMap;
use whoami::{lang, realname, username};
use wmi::{COMLibrary, Variant, WMIConnection, WMIDateTime};

#[no_mangle]
pub extern "system" fn DllMain(
    _hinst_dll: *mut winapi::ctypes::c_void,
    fdw_reason: u32,
    _lpv_reserved: *mut winapi::ctypes::c_void,
) -> i32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            main();
        }
        DLL_PROCESS_DETACH => {}
        _ => {}
    }
    1
}

pub fn main() {
    user_enum();
    let _ = system_enum();
    //network_enum();
}

pub fn user_enum() {
    println!("\n[*] Current user informations:");
    println!("Username: {}", username());
    println!("Real name: {}", realname());
    println!("Language: {:?}", lang().collect::<Vec<String>>());
}

pub fn system_enum() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[*] System informations:");
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMv2", COMLibrary::new()?)?;
    let os_results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(
        "SELECT CSName, Caption, BuildNumber, OSArchitecture FROM Win32_OperatingSystem",
    )?;
    for os_result in os_results {
        if let Some(csname) = os_result.get("CSName") {
            if let Variant::String(ref cs_name) = csname {
                println!("Computer Name: {cs_name}");
            }
        }
        if let Some(os_caption) = os_result.get("Caption") {
            if let Variant::String(ref caption) = os_caption {
                println!("OS: {caption}");
            }
        }
        if let Some(os_build) = os_result.get("BuildNumber") {
            if let Variant::String(ref build) = os_build {
                println!("Build: {build}");
            }
        }
        if let Some(os_arch) = os_result.get("OSArchitecture") {
            if let Variant::String(ref arch) = os_arch {
                println!("OS Architecture: {arch}");
            }
        }
    }

    println!("\n[*] Users informations:");
    println!(
        "{:<25} {:<35} {:<20} {:<35}\n",
        "Username", "Full username", "Domain", "SID"
    );
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMv2", COMLibrary::new()?)?;
    let users_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT Name, Caption, Domain, SID FROM Win32_UserAccount")?;

    let mut print_username: String;
    let mut print_caption: String;
    let mut print_domain: String;
    let mut print_sid: String;
    for users_result in users_results {
        print_username = String::from("");
        print_caption = String::from("");
        print_domain = String::from("");
        print_sid = String::from("");

        if let Some(user_username) = users_result.get("Name") {
            if let Variant::String(ref user) = user_username {
                print_username.push_str(user);
            }
        }
        if let Some(user_caption) = users_result.get("Caption") {
            if let Variant::String(ref caption) = user_caption {
                print_caption.push_str(caption);
            }
        }
        if let Some(user_domain) = users_result.get("Domain") {
            if let Variant::String(ref domain) = user_domain {
                print_domain.push_str(domain);
            }
        }
        if let Some(user_sid) = users_result.get("SID") {
            if let Variant::String(ref sid) = user_sid {
                print_sid.push_str(sid);
            }
        }

        println!(
            "{:<25} {:<35} {:<20} {:<35}",
            print_username, print_caption, print_domain, print_sid
        );
    }

    println!("\n[*] AV Enumeration:");
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", COMLibrary::new()?)?;

    let av_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT displayName FROM AntiVirusProduct")?;
    for av_result in av_results {
        if let Some(display_name) = av_result.get("displayName") {
            if let Variant::String(ref name) = display_name {
                println!("{}", name);
            }
        } else {
            println!("No av found!")
        }
    }

    Ok(())
}

pub fn network_enum() {
    println!("\n[*] Network interfaces:");
    // TO DO
}
