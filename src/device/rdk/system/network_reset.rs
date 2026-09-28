use std::thread;
use std::time::Duration;

use crate::dab::structs::DabError;
use crate::dab::structs::NetworkResetRequest;
use crate::dab::structs::NetworkResetResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::rdk_request_with_params;
use crate::device::rdk::interface::service_is_available;
use crate::device::rdk::interface::RdkResponse;
use crate::device::rdk::interface::RdkResponseSimple;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: NetworkResetRequest) -> Result<String, DabError> {
    let mut ResponseOperator = NetworkResetResponse::default();
    // *** Fill in the fields of the struct NetworkResetResponse here ***

    for service in ["org.rdk.Network", "org.rdk.Wifi"] {
        if !service_is_available(service)? {
            return Err(DabError::Err501(format!(
                "{} is not available; network reset is not supported",
                service
            )));
        }
    }

    //#########org.rdk.Network.getInterfaces#########
    #[derive(Deserialize)]
    struct Interface {
        interface: String,
        enabled: bool,
    }

    #[derive(Deserialize)]
    struct GetInterfacesResult {
        interfaces: Vec<Interface>,
    }

    let rdkresponse: RdkResponse<GetInterfacesResult> =
        rdk_request("org.rdk.Network.getInterfaces")?;
    let interfaces = rdkresponse.result.interfaces;

    // The reset drops the network, so it runs after the response is out
    // (the spec asks for the response once the reset is initiated).
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        // Errors are only logged: the DAB response has already been sent.

        //#########org.rdk.Network.setIPSettings#########
        // Revert static IP configuration to DHCP. All parameters are required
        // by the Network plugin; the address fields are ignored with autoconfig.
        #[derive(Serialize)]
        struct SetIPSettingsParams {
            interface: String,
            ipversion: String,
            autoconfig: bool,
            ipaddr: String,
            netmask: String,
            gateway: String,
            primarydns: String,
            secondarydns: String,
        }

        for iface in &interfaces {
            let params = SetIPSettingsParams {
                interface: iface.interface.clone(),
                ipversion: "IPv4".into(),
                autoconfig: true,
                ipaddr: "".into(),
                netmask: "".into(),
                gateway: "".into(),
                primarydns: "".into(),
                secondarydns: "".into(),
            };
            if let Err(e) = rdk_request_with_params::<_, RdkResponseSimple>(
                "org.rdk.Network.setIPSettings",
                params,
            ) {
                println!("network-reset: setIPSettings {}: {:?}", iface.interface, e);
            }
        }

        //#########org.rdk.Wifi.clearSSID#########
        // Forget the saved SSIDs and their credentials; disconnects from Wi-Fi.
        if let Err(e) = rdk_request::<RdkResponseSimple>("org.rdk.Wifi.clearSSID") {
            println!("network-reset: clearSSID: {:?}", e);
        }

        //#########org.rdk.Network.setInterfaceEnabled#########
        // Bounce the enabled interfaces so they reconnect from scratch and
        // request a new DHCP lease. The disable is not persisted, so a failed
        // re-enable is undone by a reboot.
        #[derive(Serialize)]
        struct SetInterfaceEnabledParams {
            interface: String,
            enabled: bool,
            persist: bool,
        }

        for iface in interfaces.iter().filter(|i| i.enabled) {
            for enabled in [false, true] {
                let params = SetInterfaceEnabledParams {
                    interface: iface.interface.clone(),
                    enabled,
                    persist: enabled,
                };
                if let Err(e) = rdk_request_with_params::<_, RdkResponseSimple>(
                    "org.rdk.Network.setInterfaceEnabled",
                    params,
                ) {
                    println!(
                        "network-reset: setInterfaceEnabled {} {}: {:?}",
                        iface.interface, enabled, e
                    );
                }
            }
        }
    });

    // *******************************************************************
    Ok(serde_json::to_string(&ResponseOperator).unwrap())
}
