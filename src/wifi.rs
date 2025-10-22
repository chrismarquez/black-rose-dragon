use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::wifi::AuthMethod;
use esp_idf_svc::wifi::ClientConfiguration;
use esp_idf_svc::wifi::Configuration;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripheral;
use log::{info, error};
use heapless::String;

pub struct Wifi<'a> {
    wifi: BlockingWifi<EspWifi<'a>>,
}

impl<'a> Wifi<'a> {
    pub fn new(
        modem: impl peripheral::Peripheral<P=esp_idf_svc::hal::modem::Modem> + 'a,
        sysloop: EspSystemEventLoop,
    ) -> Result<Self, esp_idf_svc::sys::EspError> {
        let wifi = BlockingWifi::wrap(
            EspWifi::new(modem, sysloop.clone(), None)?,
            sysloop,
        )?;

        Ok(Self { wifi })
    }

    pub fn connect(&mut self, ssid: &str, pass: &str) -> Result<(), esp_idf_svc::sys::EspError> {
        self.wifi.start()?;
        info!("Wifi started");
        
        let Ok(h_ssid) = heapless::String::<32>::try_from(ssid) else {
            panic!("Could not convert SSID to heapless string");
        };
        let Ok(h_pass) = heapless::String::<64>::try_from(pass) else { 
            panic!("Could not convert password to heapless string");
        };

        self.wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: h_ssid,
            password: h_pass,
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        }))?;
        
        let access_points = self.wifi.scan()?;
        for ap in access_points {
            info!("Found AP: {}", ap.ssid);
        }

        info!("Connecting to '{}' with password '{}'", ssid, pass);
        self.wifi.connect()?;
        info!("Waiting for DHCP lease...");
        self.wifi.wait_netif_up()?;
        info!("Connected!");

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), esp_idf_svc::sys::EspError> {
        self.wifi.disconnect()?;
        self.wifi.stop()?;
        Ok(())
    }
}
