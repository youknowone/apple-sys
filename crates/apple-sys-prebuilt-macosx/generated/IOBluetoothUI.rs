#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::IOBluetooth::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type IOBluetoothDeviceSelectorControllerRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothPairingControllerRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothServiceBrowserControllerRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothServiceBrowserControllerOptions = u32;
unsafe extern "C" {
    pub fn IOBluetoothValidateHardwareWithDescription(
        cancelButtonTitle: CFStringRef,
        descriptionText: CFStringRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOBluetoothGetPairingController() -> IOBluetoothPairingControllerRef;
}
unsafe extern "C" {
    pub fn IOBluetoothGetDeviceSelectorController() -> IOBluetoothDeviceSelectorControllerRef;
}
