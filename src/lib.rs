use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use colorized::*;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use std::collections::HashMap;
use whoami::{lang, realname, username};
use wmi::{COMLibrary, Variant, WMIConnection};

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
    let _ = user_enum();
    let _ = system_enum();
    let _ = service_enum();
    network_enum();
}

pub fn user_enum() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "[*] Current user:".color(Colors::BrightCyanFg));
    println!("Username: {}", username());
    println!("Real name: {}", realname());
    println!("Language: {:?}", lang().collect::<Vec<String>>());

    println!("\n{}", "[*] Other users:".color(Colors::BrightCyanFg));
    println!(
        "{:<25} {:<35} {:<20} {:<35}",
        "Username", "Full username", "Domain", "SID"
    );
    println!(
        "{:<25} {:<35} {:<20} {:<35}",
        "--------", "-------------", "------", "---"
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

    Ok(())
}

pub fn system_enum() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "[*] System:".color(Colors::BrightCyanFg));
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

    println!("\n{}", "[*] AV Enumeration:".color(Colors::BrightCyanFg));
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", COMLibrary::new()?)?;
    let av_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT displayName FROM AntiVirusProduct")?;
    for av_result in av_results {
        if let Some(display_name) = av_result.get("displayName") {
            if let Variant::String(ref name) = display_name {
                println!("{}", name.color(Colors::BrightRedFg));
            }
        } else {
            println!("No av found!")
        }
    }

    println!("\n{}", "[*] Windows patches".color(Colors::BrightCyanFg));
    println!("{:<20} {:<15} {:<20}", "Computer", "HotFixID", "Type");
    println!("{:<20} {:<15} {:<20}", "--------", "--------", "----");
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMv2", COMLibrary::new()?)?;
    let patches_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT CSName, HotFixID, Description FROM Win32_QuickFixEngineering")?;

    let mut print_hotcsname: String;
    let mut print_hotfixid: String;
    let mut print_hotdesc: String;
    for patches_result in patches_results {
        print_hotcsname = String::from("");
        print_hotfixid = String::from("");
        print_hotdesc = String::from("");

        if let Some(hotfix_csname) = patches_result.get("CSName") {
            if let Variant::String(ref csname) = hotfix_csname {
                print_hotcsname.push_str(csname);
            }
        }

        if let Some(hotfix_hotfixid) = patches_result.get("HotFixID") {
            if let Variant::String(ref hotfixid) = hotfix_hotfixid {
                print_hotfixid.push_str(hotfixid);
            }
        }

        if let Some(hotfix_desc) = patches_result.get("Description") {
            if let Variant::String(ref desc) = hotfix_desc {
                print_hotdesc.push_str(desc);
            }
        }

        println!(
            "{:<20} {:<15} {:<20}",
            print_hotcsname, print_hotfixid, print_hotdesc
        );
    }

    Ok(())
}

pub fn service_enum() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "[*] Current services:".color(Colors::BrightCyanFg));
    println!("-> Checking unquoted path:");
    println!("\t{:<35} {}", "Service name", "Service path");
    println!("\t{:<35} {}", "------------", "------------");
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMv2", COMLibrary::new()?)?;
    let services_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT Name, PathName FROM Win32_Service")?;

    let mut print_scname: String;
    let mut print_scpath: String;
    for services_result in services_results {
        print_scname = String::from("");
        print_scpath = String::from("");

        if let Some(sc_name) = services_result.get("Name") {
            if let Variant::String(ref name) = sc_name {
                print_scname.push_str(name);
            }
        }

        if let Some(sc_path) = services_result.get("PathName") {
            if let Variant::String(ref path) = sc_path {
                print_scpath.push_str(path);
            }
        }

        if print_scpath.contains(" ") {
            if !print_scpath.to_lowercase().contains("system32") {
                println!(
                    "\t{:<35} {}",
                    print_scname,
                    print_scpath.color(Colors::BrightYellowFg)
                );
            }
        }
    }

    Ok(())
}

pub fn network_enum() {
    println!(
        "\n{}",
        "[*] Network interfaces:".color(Colors::BrightCyanFg)
    );
    println!(
        "{:<15} {:<15} {:<15}",
        "Interface name", "IP address", "Netmask"
    );
    println!(
        "{:<15} {:<15} {:<15}",
        "--------------", "----------", "-------"
    );
    let network_interfaces = NetworkInterface::show().unwrap();

    let mut print_interface: String;
    let mut print_ip: String;
    let mut print_netmask: String;
    for interface in network_interfaces.iter() {
        print_interface = interface.name.to_owned();
        print_ip = interface.addr.get(1).unwrap().ip().to_string();
        print_netmask = interface
            .addr
            .get(1)
            .unwrap()
            .netmask()
            .unwrap()
            .to_string();

        if !print_interface.to_lowercase().contains("loopback") {
            println!(
                "{:<15} {:<15} {:<15}",
                print_interface, print_ip, print_netmask
            );
        }
    }
}
