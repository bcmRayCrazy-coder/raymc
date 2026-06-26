use bluest::{Adapter, AdvertisingDevice, Device};
use iced::futures::StreamExt;

pub enum BluetoothError {
    AdapterNotSetup,
    AdapterNotFound,
    AdapterUnavailable,
    ScanFailed,
    ConnectFailed,
    DisconnectFailed
}

pub struct BluetoothManager {
    adapter: Option<Adapter>,
    device_list: Vec<AdvertisingDevice>,
}

impl BluetoothManager {
    pub async fn setup(&mut self) -> Result<(), BluetoothError> {
        let adapter = Adapter::default()
            .await
            .ok_or(BluetoothError::AdapterNotFound)?;
        adapter
            .wait_available()
            .await
            .map_err(|_| BluetoothError::AdapterUnavailable)?;
        self.adapter = Some(adapter);
        Ok(())
    }

    pub async fn scan(&mut self) -> Result<(), BluetoothError> {
        if let Some(adapter) = self.adapter.as_ref() {
            let mut result = adapter
                .scan(&[])
                .await
                .map_err(|_| BluetoothError::ScanFailed)?;

            self.device_list.clear();
            while let Some(device) = result.next().await {
                self.device_list.push(device.clone());
            }

            Ok(())
        } else {
            Err(BluetoothError::AdapterNotSetup)
        }
    }

    pub async fn connect(&mut self, device: &Device) -> Result<(), BluetoothError> {
        if let Some(adapter) = self.adapter.as_ref() {
            match adapter.connect_device(device).await {
                Ok(_) => Ok(()),
                Err(_) => Err(BluetoothError::ConnectFailed),
            }
        } else {
            Err(BluetoothError::AdapterNotSetup)
        }
    }

    pub async fn disconnect(&mut self, device: &Device) -> Result<(), BluetoothError> {
        if let Some(adapter) = self.adapter.as_ref() {
            match adapter.disconnect_device(device).await {
                Ok(_) => Ok(()),
                Err(_) => Err(BluetoothError::DisconnectFailed),
            }
        } else {
            Err(BluetoothError::AdapterNotSetup)
        }
    }
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self {
            adapter: None,
            device_list: Vec::new(),
        }
    }
}
