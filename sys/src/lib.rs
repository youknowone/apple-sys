//! apple-sys main module
//! auto-generated by "python 'configure.py'"
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]

#[cfg(feature = "AGL")]
pub mod AGL {
    include!(concat!(env!("OUT_DIR"), "/AGL.rs"));
}
#[cfg(feature = "AVFAudio")]
pub mod AVFAudio {
    include!(concat!(env!("OUT_DIR"), "/AVFAudio.rs"));
}
#[cfg(feature = "AVFoundation")]
pub mod AVFoundation {
    include!(concat!(env!("OUT_DIR"), "/AVFoundation.rs"));
}
#[cfg(feature = "AVKit")]
pub mod AVKit {
    include!(concat!(env!("OUT_DIR"), "/AVKit.rs"));
}
#[cfg(feature = "Accelerate")]
pub mod Accelerate {
    include!(concat!(env!("OUT_DIR"), "/Accelerate.rs"));
}
#[cfg(feature = "Accessibility")]
pub mod Accessibility {
    include!(concat!(env!("OUT_DIR"), "/Accessibility.rs"));
}
#[cfg(feature = "Accounts")]
pub mod Accounts {
    include!(concat!(env!("OUT_DIR"), "/Accounts.rs"));
}
#[cfg(feature = "AdServices")]
pub mod AdServices {
    include!(concat!(env!("OUT_DIR"), "/AdServices.rs"));
}
#[cfg(feature = "AdSupport")]
pub mod AdSupport {
    include!(concat!(env!("OUT_DIR"), "/AdSupport.rs"));
}
#[cfg(feature = "AddressBook")]
pub mod AddressBook {
    include!(concat!(env!("OUT_DIR"), "/AddressBook.rs"));
}
#[cfg(feature = "AppKit")]
pub mod AppKit {
    include!(concat!(env!("OUT_DIR"), "/AppKit.rs"));
}
#[cfg(feature = "AppTrackingTransparency")]
pub mod AppTrackingTransparency {
    include!(concat!(env!("OUT_DIR"), "/AppTrackingTransparency.rs"));
}
#[cfg(feature = "AppleScriptKit")]
pub mod AppleScriptKit {
    include!(concat!(env!("OUT_DIR"), "/AppleScriptKit.rs"));
}
#[cfg(feature = "AppleScriptObjC")]
pub mod AppleScriptObjC {
    include!(concat!(env!("OUT_DIR"), "/AppleScriptObjC.rs"));
}
#[cfg(feature = "ApplicationServices")]
pub mod ApplicationServices {
    include!(concat!(env!("OUT_DIR"), "/ApplicationServices.rs"));
}
#[cfg(feature = "AudioToolbox")]
pub mod AudioToolbox {
    include!(concat!(env!("OUT_DIR"), "/AudioToolbox.rs"));
}
#[cfg(feature = "AudioUnit")]
pub mod AudioUnit {
    include!(concat!(env!("OUT_DIR"), "/AudioUnit.rs"));
}
#[cfg(feature = "AudioVideoBridging")]
pub mod AudioVideoBridging {
    include!(concat!(env!("OUT_DIR"), "/AudioVideoBridging.rs"));
}
#[cfg(feature = "AuthenticationServices")]
pub mod AuthenticationServices {
    include!(concat!(env!("OUT_DIR"), "/AuthenticationServices.rs"));
}
#[cfg(feature = "AutomaticAssessmentConfiguration")]
pub mod AutomaticAssessmentConfiguration {
    include!(concat!(
        env!("OUT_DIR"),
        "/AutomaticAssessmentConfiguration.rs"
    ));
}
#[cfg(feature = "Automator")]
pub mod Automator {
    include!(concat!(env!("OUT_DIR"), "/Automator.rs"));
}
#[cfg(feature = "BackgroundTasks")]
pub mod BackgroundTasks {
    include!(concat!(env!("OUT_DIR"), "/BackgroundTasks.rs"));
}
#[cfg(feature = "BusinessChat")]
pub mod BusinessChat {
    include!(concat!(env!("OUT_DIR"), "/BusinessChat.rs"));
}
#[cfg(feature = "CFNetwork")]
pub mod CFNetwork {
    include!(concat!(env!("OUT_DIR"), "/CFNetwork.rs"));
}
#[cfg(feature = "CHIP")]
pub mod CHIP {
    include!(concat!(env!("OUT_DIR"), "/CHIP.rs"));
}
#[cfg(feature = "CalendarStore")]
pub mod CalendarStore {
    include!(concat!(env!("OUT_DIR"), "/CalendarStore.rs"));
}
#[cfg(feature = "CallKit")]
pub mod CallKit {
    include!(concat!(env!("OUT_DIR"), "/CallKit.rs"));
}
#[cfg(feature = "Carbon")]
pub mod Carbon {
    include!(concat!(env!("OUT_DIR"), "/Carbon.rs"));
}
#[cfg(feature = "ClassKit")]
pub mod ClassKit {
    include!(concat!(env!("OUT_DIR"), "/ClassKit.rs"));
}
#[cfg(feature = "CloudKit")]
pub mod CloudKit {
    include!(concat!(env!("OUT_DIR"), "/CloudKit.rs"));
}
#[cfg(feature = "Cocoa")]
pub mod Cocoa {
    include!(concat!(env!("OUT_DIR"), "/Cocoa.rs"));
}
#[cfg(feature = "Collaboration")]
pub mod Collaboration {
    include!(concat!(env!("OUT_DIR"), "/Collaboration.rs"));
}
#[cfg(feature = "ColorSync")]
pub mod ColorSync {
    include!(concat!(env!("OUT_DIR"), "/ColorSync.rs"));
}
#[cfg(feature = "Contacts")]
pub mod Contacts {
    include!(concat!(env!("OUT_DIR"), "/Contacts.rs"));
}
#[cfg(feature = "ContactsUI")]
pub mod ContactsUI {
    include!(concat!(env!("OUT_DIR"), "/ContactsUI.rs"));
}
#[cfg(feature = "CoreAudio")]
pub mod CoreAudio {
    include!(concat!(env!("OUT_DIR"), "/CoreAudio.rs"));
}
#[cfg(feature = "CoreAudioKit")]
pub mod CoreAudioKit {
    include!(concat!(env!("OUT_DIR"), "/CoreAudioKit.rs"));
}
#[cfg(feature = "CoreBluetooth")]
pub mod CoreBluetooth {
    include!(concat!(env!("OUT_DIR"), "/CoreBluetooth.rs"));
}
#[cfg(feature = "CoreData")]
pub mod CoreData {
    include!(concat!(env!("OUT_DIR"), "/CoreData.rs"));
}
#[cfg(feature = "CoreFoundation")]
pub mod CoreFoundation {
    include!(concat!(env!("OUT_DIR"), "/CoreFoundation.rs"));
}
#[cfg(feature = "CoreGraphics")]
pub mod CoreGraphics {
    include!(concat!(env!("OUT_DIR"), "/CoreGraphics.rs"));
}
#[cfg(feature = "CoreHaptics")]
pub mod CoreHaptics {
    include!(concat!(env!("OUT_DIR"), "/CoreHaptics.rs"));
}
#[cfg(feature = "CoreImage")]
pub mod CoreImage {
    include!(concat!(env!("OUT_DIR"), "/CoreImage.rs"));
}
#[cfg(feature = "CoreLocation")]
pub mod CoreLocation {
    include!(concat!(env!("OUT_DIR"), "/CoreLocation.rs"));
}
#[cfg(feature = "CoreMIDI")]
pub mod CoreMIDI {
    include!(concat!(env!("OUT_DIR"), "/CoreMIDI.rs"));
}
#[cfg(feature = "CoreML")]
pub mod CoreML {
    include!(concat!(env!("OUT_DIR"), "/CoreML.rs"));
}
#[cfg(feature = "CoreMedia")]
pub mod CoreMedia {
    include!(concat!(env!("OUT_DIR"), "/CoreMedia.rs"));
}
#[cfg(feature = "CoreMediaIO")]
pub mod CoreMediaIO {
    include!(concat!(env!("OUT_DIR"), "/CoreMediaIO.rs"));
}
#[cfg(feature = "CoreMotion")]
pub mod CoreMotion {
    include!(concat!(env!("OUT_DIR"), "/CoreMotion.rs"));
}
#[cfg(feature = "CoreServices")]
pub mod CoreServices {
    include!(concat!(env!("OUT_DIR"), "/CoreServices.rs"));
}
#[cfg(feature = "CoreSpotlight")]
pub mod CoreSpotlight {
    include!(concat!(env!("OUT_DIR"), "/CoreSpotlight.rs"));
}
#[cfg(feature = "CoreTelephony")]
pub mod CoreTelephony {
    include!(concat!(env!("OUT_DIR"), "/CoreTelephony.rs"));
}
#[cfg(feature = "CoreText")]
pub mod CoreText {
    include!(concat!(env!("OUT_DIR"), "/CoreText.rs"));
}
#[cfg(feature = "CoreVideo")]
pub mod CoreVideo {
    include!(concat!(env!("OUT_DIR"), "/CoreVideo.rs"));
}
#[cfg(feature = "CoreWLAN")]
pub mod CoreWLAN {
    include!(concat!(env!("OUT_DIR"), "/CoreWLAN.rs"));
}
#[cfg(feature = "CryptoTokenKit")]
pub mod CryptoTokenKit {
    include!(concat!(env!("OUT_DIR"), "/CryptoTokenKit.rs"));
}
#[cfg(feature = "DVDPlayback")]
pub mod DVDPlayback {
    include!(concat!(env!("OUT_DIR"), "/DVDPlayback.rs"));
}
#[cfg(feature = "DataDetection")]
pub mod DataDetection {
    include!(concat!(env!("OUT_DIR"), "/DataDetection.rs"));
}
#[cfg(feature = "DeviceCheck")]
pub mod DeviceCheck {
    include!(concat!(env!("OUT_DIR"), "/DeviceCheck.rs"));
}
#[cfg(feature = "DirectoryService")]
pub mod DirectoryService {
    include!(concat!(env!("OUT_DIR"), "/DirectoryService.rs"));
}
#[cfg(feature = "DiscRecording")]
pub mod DiscRecording {
    include!(concat!(env!("OUT_DIR"), "/DiscRecording.rs"));
}
#[cfg(feature = "DiscRecordingUI")]
pub mod DiscRecordingUI {
    include!(concat!(env!("OUT_DIR"), "/DiscRecordingUI.rs"));
}
#[cfg(feature = "DiskArbitration")]
pub mod DiskArbitration {
    include!(concat!(env!("OUT_DIR"), "/DiskArbitration.rs"));
}
#[cfg(feature = "EventKit")]
pub mod EventKit {
    include!(concat!(env!("OUT_DIR"), "/EventKit.rs"));
}
#[cfg(feature = "ExceptionHandling")]
pub mod ExceptionHandling {
    include!(concat!(env!("OUT_DIR"), "/ExceptionHandling.rs"));
}
#[cfg(feature = "ExecutionPolicy")]
pub mod ExecutionPolicy {
    include!(concat!(env!("OUT_DIR"), "/ExecutionPolicy.rs"));
}
#[cfg(feature = "ExposureNotification")]
pub mod ExposureNotification {
    include!(concat!(env!("OUT_DIR"), "/ExposureNotification.rs"));
}
#[cfg(feature = "ExternalAccessory")]
pub mod ExternalAccessory {
    include!(concat!(env!("OUT_DIR"), "/ExternalAccessory.rs"));
}
#[cfg(feature = "FWAUserLib")]
pub mod FWAUserLib {
    include!(concat!(env!("OUT_DIR"), "/FWAUserLib.rs"));
}
#[cfg(feature = "FileProvider")]
pub mod FileProvider {
    include!(concat!(env!("OUT_DIR"), "/FileProvider.rs"));
}
#[cfg(feature = "FileProviderUI")]
pub mod FileProviderUI {
    include!(concat!(env!("OUT_DIR"), "/FileProviderUI.rs"));
}
#[cfg(feature = "FinderSync")]
pub mod FinderSync {
    include!(concat!(env!("OUT_DIR"), "/FinderSync.rs"));
}
#[cfg(feature = "ForceFeedback")]
pub mod ForceFeedback {
    include!(concat!(env!("OUT_DIR"), "/ForceFeedback.rs"));
}
#[cfg(feature = "Foundation")]
pub mod Foundation {
    include!(concat!(env!("OUT_DIR"), "/Foundation.rs"));
}
#[cfg(feature = "GLKit")]
pub mod GLKit {
    include!(concat!(env!("OUT_DIR"), "/GLKit.rs"));
}
#[cfg(feature = "GLUT")]
pub mod GLUT {
    include!(concat!(env!("OUT_DIR"), "/GLUT.rs"));
}
#[cfg(feature = "GSS")]
pub mod GSS {
    include!(concat!(env!("OUT_DIR"), "/GSS.rs"));
}
#[cfg(feature = "GameController")]
pub mod GameController {
    include!(concat!(env!("OUT_DIR"), "/GameController.rs"));
}
#[cfg(feature = "GameKit")]
pub mod GameKit {
    include!(concat!(env!("OUT_DIR"), "/GameKit.rs"));
}
#[cfg(feature = "GameplayKit")]
pub mod GameplayKit {
    include!(concat!(env!("OUT_DIR"), "/GameplayKit.rs"));
}
#[cfg(feature = "Hypervisor")]
pub mod Hypervisor {
    include!(concat!(env!("OUT_DIR"), "/Hypervisor.rs"));
}
#[cfg(feature = "ICADevices")]
pub mod ICADevices {
    include!(concat!(env!("OUT_DIR"), "/ICADevices.rs"));
}
#[cfg(feature = "IMServicePlugIn")]
pub mod IMServicePlugIn {
    include!(concat!(env!("OUT_DIR"), "/IMServicePlugIn.rs"));
}
#[cfg(feature = "IOBluetooth")]
pub mod IOBluetooth {
    include!(concat!(env!("OUT_DIR"), "/IOBluetooth.rs"));
}
#[cfg(feature = "IOBluetoothUI")]
pub mod IOBluetoothUI {
    include!(concat!(env!("OUT_DIR"), "/IOBluetoothUI.rs"));
}
#[cfg(feature = "IOKit")]
pub mod IOKit {
    include!(concat!(env!("OUT_DIR"), "/IOKit.rs"));
}
#[cfg(feature = "IOSurface")]
pub mod IOSurface {
    include!(concat!(env!("OUT_DIR"), "/IOSurface.rs"));
}
#[cfg(feature = "IOUSBHost")]
pub mod IOUSBHost {
    include!(concat!(env!("OUT_DIR"), "/IOUSBHost.rs"));
}
#[cfg(feature = "IdentityLookup")]
pub mod IdentityLookup {
    include!(concat!(env!("OUT_DIR"), "/IdentityLookup.rs"));
}
#[cfg(feature = "ImageCaptureCore")]
pub mod ImageCaptureCore {
    include!(concat!(env!("OUT_DIR"), "/ImageCaptureCore.rs"));
}
#[cfg(feature = "ImageIO")]
pub mod ImageIO {
    include!(concat!(env!("OUT_DIR"), "/ImageIO.rs"));
}
#[cfg(feature = "InputMethodKit")]
pub mod InputMethodKit {
    include!(concat!(env!("OUT_DIR"), "/InputMethodKit.rs"));
}
#[cfg(feature = "InstallerPlugins")]
pub mod InstallerPlugins {
    include!(concat!(env!("OUT_DIR"), "/InstallerPlugins.rs"));
}
#[cfg(feature = "InstantMessage")]
pub mod InstantMessage {
    include!(concat!(env!("OUT_DIR"), "/InstantMessage.rs"));
}
#[cfg(feature = "Intents")]
pub mod Intents {
    include!(concat!(env!("OUT_DIR"), "/Intents.rs"));
}
#[cfg(feature = "IntentsUI")]
pub mod IntentsUI {
    include!(concat!(env!("OUT_DIR"), "/IntentsUI.rs"));
}
#[cfg(feature = "JavaNativeFoundation")]
pub mod JavaNativeFoundation {
    include!(concat!(env!("OUT_DIR"), "/JavaNativeFoundation.rs"));
}
#[cfg(feature = "JavaRuntimeSupport")]
pub mod JavaRuntimeSupport {
    include!(concat!(env!("OUT_DIR"), "/JavaRuntimeSupport.rs"));
}
#[cfg(feature = "JavaScriptCore")]
pub mod JavaScriptCore {
    include!(concat!(env!("OUT_DIR"), "/JavaScriptCore.rs"));
}
#[cfg(feature = "Kerberos")]
pub mod Kerberos {
    include!(concat!(env!("OUT_DIR"), "/Kerberos.rs"));
}
#[cfg(feature = "KernelManagement")]
pub mod KernelManagement {
    include!(concat!(env!("OUT_DIR"), "/KernelManagement.rs"));
}
#[cfg(feature = "LDAP")]
pub mod LDAP {
    include!(concat!(env!("OUT_DIR"), "/LDAP.rs"));
}
#[cfg(feature = "LatentSemanticMapping")]
pub mod LatentSemanticMapping {
    include!(concat!(env!("OUT_DIR"), "/LatentSemanticMapping.rs"));
}
#[cfg(feature = "LinkPresentation")]
pub mod LinkPresentation {
    include!(concat!(env!("OUT_DIR"), "/LinkPresentation.rs"));
}
#[cfg(feature = "LocalAuthentication")]
pub mod LocalAuthentication {
    include!(concat!(env!("OUT_DIR"), "/LocalAuthentication.rs"));
}
#[cfg(feature = "LocalAuthenticationEmbeddedUI")]
pub mod LocalAuthenticationEmbeddedUI {
    include!(concat!(
        env!("OUT_DIR"),
        "/LocalAuthenticationEmbeddedUI.rs"
    ));
}
#[cfg(feature = "MLCompute")]
pub mod MLCompute {
    include!(concat!(env!("OUT_DIR"), "/MLCompute.rs"));
}
#[cfg(feature = "MailKit")]
pub mod MailKit {
    include!(concat!(env!("OUT_DIR"), "/MailKit.rs"));
}
#[cfg(feature = "MapKit")]
pub mod MapKit {
    include!(concat!(env!("OUT_DIR"), "/MapKit.rs"));
}
#[cfg(feature = "MediaAccessibility")]
pub mod MediaAccessibility {
    include!(concat!(env!("OUT_DIR"), "/MediaAccessibility.rs"));
}
#[cfg(feature = "MediaLibrary")]
pub mod MediaLibrary {
    include!(concat!(env!("OUT_DIR"), "/MediaLibrary.rs"));
}
#[cfg(feature = "MediaPlayer")]
pub mod MediaPlayer {
    include!(concat!(env!("OUT_DIR"), "/MediaPlayer.rs"));
}
#[cfg(feature = "MediaToolbox")]
pub mod MediaToolbox {
    include!(concat!(env!("OUT_DIR"), "/MediaToolbox.rs"));
}
#[cfg(feature = "Metal")]
pub mod Metal {
    include!(concat!(env!("OUT_DIR"), "/Metal.rs"));
}
#[cfg(feature = "MetalKit")]
pub mod MetalKit {
    include!(concat!(env!("OUT_DIR"), "/MetalKit.rs"));
}
#[cfg(feature = "MetalPerformanceShaders")]
pub mod MetalPerformanceShaders {
    include!(concat!(env!("OUT_DIR"), "/MetalPerformanceShaders.rs"));
}
#[cfg(feature = "MetalPerformanceShadersGraph")]
pub mod MetalPerformanceShadersGraph {
    include!(concat!(env!("OUT_DIR"), "/MetalPerformanceShadersGraph.rs"));
}
#[cfg(feature = "MetricKit")]
pub mod MetricKit {
    include!(concat!(env!("OUT_DIR"), "/MetricKit.rs"));
}
#[cfg(feature = "ModelIO")]
pub mod ModelIO {
    include!(concat!(env!("OUT_DIR"), "/ModelIO.rs"));
}
#[cfg(feature = "MultipeerConnectivity")]
pub mod MultipeerConnectivity {
    include!(concat!(env!("OUT_DIR"), "/MultipeerConnectivity.rs"));
}
#[cfg(feature = "NaturalLanguage")]
pub mod NaturalLanguage {
    include!(concat!(env!("OUT_DIR"), "/NaturalLanguage.rs"));
}
#[cfg(feature = "NearbyInteraction")]
pub mod NearbyInteraction {
    include!(concat!(env!("OUT_DIR"), "/NearbyInteraction.rs"));
}
#[cfg(feature = "NetFS")]
pub mod NetFS {
    include!(concat!(env!("OUT_DIR"), "/NetFS.rs"));
}
#[cfg(feature = "Network")]
pub mod Network {
    include!(concat!(env!("OUT_DIR"), "/Network.rs"));
}
#[cfg(feature = "NetworkExtension")]
pub mod NetworkExtension {
    include!(concat!(env!("OUT_DIR"), "/NetworkExtension.rs"));
}
#[cfg(feature = "NotificationCenter")]
pub mod NotificationCenter {
    include!(concat!(env!("OUT_DIR"), "/NotificationCenter.rs"));
}
#[cfg(feature = "OSAKit")]
pub mod OSAKit {
    include!(concat!(env!("OUT_DIR"), "/OSAKit.rs"));
}
#[cfg(feature = "OSLog")]
pub mod OSLog {
    include!(concat!(env!("OUT_DIR"), "/OSLog.rs"));
}
#[cfg(feature = "OpenAL")]
pub mod OpenAL {
    include!(concat!(env!("OUT_DIR"), "/OpenAL.rs"));
}
#[cfg(feature = "OpenCL")]
pub mod OpenCL {
    include!(concat!(env!("OUT_DIR"), "/OpenCL.rs"));
}
#[cfg(feature = "OpenDirectory")]
pub mod OpenDirectory {
    include!(concat!(env!("OUT_DIR"), "/OpenDirectory.rs"));
}
#[cfg(feature = "OpenGL")]
pub mod OpenGL {
    include!(concat!(env!("OUT_DIR"), "/OpenGL.rs"));
}
#[cfg(feature = "PCSC")]
pub mod PCSC {
    include!(concat!(env!("OUT_DIR"), "/PCSC.rs"));
}
#[cfg(feature = "PDFKit")]
pub mod PDFKit {
    include!(concat!(env!("OUT_DIR"), "/PDFKit.rs"));
}
#[cfg(feature = "PHASE")]
pub mod PHASE {
    include!(concat!(env!("OUT_DIR"), "/PHASE.rs"));
}
#[cfg(feature = "ParavirtualizedGraphics")]
pub mod ParavirtualizedGraphics {
    include!(concat!(env!("OUT_DIR"), "/ParavirtualizedGraphics.rs"));
}
#[cfg(feature = "PassKit")]
pub mod PassKit {
    include!(concat!(env!("OUT_DIR"), "/PassKit.rs"));
}
#[cfg(feature = "PencilKit")]
pub mod PencilKit {
    include!(concat!(env!("OUT_DIR"), "/PencilKit.rs"));
}
#[cfg(feature = "Photos")]
pub mod Photos {
    include!(concat!(env!("OUT_DIR"), "/Photos.rs"));
}
#[cfg(feature = "PhotosUI")]
pub mod PhotosUI {
    include!(concat!(env!("OUT_DIR"), "/PhotosUI.rs"));
}
#[cfg(feature = "PreferencePanes")]
pub mod PreferencePanes {
    include!(concat!(env!("OUT_DIR"), "/PreferencePanes.rs"));
}
#[cfg(feature = "PushKit")]
pub mod PushKit {
    include!(concat!(env!("OUT_DIR"), "/PushKit.rs"));
}
#[cfg(feature = "Quartz")]
pub mod Quartz {
    include!(concat!(env!("OUT_DIR"), "/Quartz.rs"));
}
#[cfg(feature = "QuartzCore")]
pub mod QuartzCore {
    include!(concat!(env!("OUT_DIR"), "/QuartzCore.rs"));
}
#[cfg(feature = "QuickLook")]
pub mod QuickLook {
    include!(concat!(env!("OUT_DIR"), "/QuickLook.rs"));
}
#[cfg(feature = "QuickLookThumbnailing")]
pub mod QuickLookThumbnailing {
    include!(concat!(env!("OUT_DIR"), "/QuickLookThumbnailing.rs"));
}
#[cfg(feature = "QuickLookUI")]
pub mod QuickLookUI {
    include!(concat!(env!("OUT_DIR"), "/QuickLookUI.rs"));
}
#[cfg(feature = "ReplayKit")]
pub mod ReplayKit {
    include!(concat!(env!("OUT_DIR"), "/ReplayKit.rs"));
}
#[cfg(feature = "SafariServices")]
pub mod SafariServices {
    include!(concat!(env!("OUT_DIR"), "/SafariServices.rs"));
}
#[cfg(feature = "SceneKit")]
pub mod SceneKit {
    include!(concat!(env!("OUT_DIR"), "/SceneKit.rs"));
}
#[cfg(feature = "ScreenCaptureKit")]
pub mod ScreenCaptureKit {
    include!(concat!(env!("OUT_DIR"), "/ScreenCaptureKit.rs"));
}
#[cfg(feature = "ScreenSaver")]
pub mod ScreenSaver {
    include!(concat!(env!("OUT_DIR"), "/ScreenSaver.rs"));
}
#[cfg(feature = "ScreenTime")]
pub mod ScreenTime {
    include!(concat!(env!("OUT_DIR"), "/ScreenTime.rs"));
}
#[cfg(feature = "ScriptingBridge")]
pub mod ScriptingBridge {
    include!(concat!(env!("OUT_DIR"), "/ScriptingBridge.rs"));
}
#[cfg(feature = "Security")]
pub mod Security {
    include!(concat!(env!("OUT_DIR"), "/Security.rs"));
}
#[cfg(feature = "SecurityFoundation")]
pub mod SecurityFoundation {
    include!(concat!(env!("OUT_DIR"), "/SecurityFoundation.rs"));
}
#[cfg(feature = "SecurityInterface")]
pub mod SecurityInterface {
    include!(concat!(env!("OUT_DIR"), "/SecurityInterface.rs"));
}
#[cfg(feature = "SensorKit")]
pub mod SensorKit {
    include!(concat!(env!("OUT_DIR"), "/SensorKit.rs"));
}
#[cfg(feature = "ServiceManagement")]
pub mod ServiceManagement {
    include!(concat!(env!("OUT_DIR"), "/ServiceManagement.rs"));
}
#[cfg(feature = "ShazamKit")]
pub mod ShazamKit {
    include!(concat!(env!("OUT_DIR"), "/ShazamKit.rs"));
}
#[cfg(feature = "Social")]
pub mod Social {
    include!(concat!(env!("OUT_DIR"), "/Social.rs"));
}
#[cfg(feature = "SoundAnalysis")]
pub mod SoundAnalysis {
    include!(concat!(env!("OUT_DIR"), "/SoundAnalysis.rs"));
}
#[cfg(feature = "Speech")]
pub mod Speech {
    include!(concat!(env!("OUT_DIR"), "/Speech.rs"));
}
#[cfg(feature = "SpriteKit")]
pub mod SpriteKit {
    include!(concat!(env!("OUT_DIR"), "/SpriteKit.rs"));
}
#[cfg(feature = "StoreKit")]
pub mod StoreKit {
    include!(concat!(env!("OUT_DIR"), "/StoreKit.rs"));
}
#[cfg(feature = "SwiftUI")]
pub mod SwiftUI {
    include!(concat!(env!("OUT_DIR"), "/SwiftUI.rs"));
}
#[cfg(feature = "SyncServices")]
pub mod SyncServices {
    include!(concat!(env!("OUT_DIR"), "/SyncServices.rs"));
}
#[cfg(feature = "SystemConfiguration")]
pub mod SystemConfiguration {
    include!(concat!(env!("OUT_DIR"), "/SystemConfiguration.rs"));
}
#[cfg(feature = "SystemExtensions")]
pub mod SystemExtensions {
    include!(concat!(env!("OUT_DIR"), "/SystemExtensions.rs"));
}
#[cfg(feature = "TWAIN")]
pub mod TWAIN {
    include!(concat!(env!("OUT_DIR"), "/TWAIN.rs"));
}
#[cfg(feature = "Tcl")]
pub mod Tcl {
    include!(concat!(env!("OUT_DIR"), "/Tcl.rs"));
}
#[cfg(feature = "UniformTypeIdentifiers")]
pub mod UniformTypeIdentifiers {
    include!(concat!(env!("OUT_DIR"), "/UniformTypeIdentifiers.rs"));
}
#[cfg(feature = "UserNotifications")]
pub mod UserNotifications {
    include!(concat!(env!("OUT_DIR"), "/UserNotifications.rs"));
}
#[cfg(feature = "UserNotificationsUI")]
pub mod UserNotificationsUI {
    include!(concat!(env!("OUT_DIR"), "/UserNotificationsUI.rs"));
}
#[cfg(feature = "VideoDecodeAcceleration")]
pub mod VideoDecodeAcceleration {
    include!(concat!(env!("OUT_DIR"), "/VideoDecodeAcceleration.rs"));
}
#[cfg(feature = "VideoSubscriberAccount")]
pub mod VideoSubscriberAccount {
    include!(concat!(env!("OUT_DIR"), "/VideoSubscriberAccount.rs"));
}
#[cfg(feature = "VideoToolbox")]
pub mod VideoToolbox {
    include!(concat!(env!("OUT_DIR"), "/VideoToolbox.rs"));
}
#[cfg(feature = "Virtualization")]
pub mod Virtualization {
    include!(concat!(env!("OUT_DIR"), "/Virtualization.rs"));
}
#[cfg(feature = "Vision")]
pub mod Vision {
    include!(concat!(env!("OUT_DIR"), "/Vision.rs"));
}
#[cfg(feature = "WebKit")]
pub mod WebKit {
    include!(concat!(env!("OUT_DIR"), "/WebKit.rs"));
}
#[cfg(feature = "WidgetKit")]
pub mod WidgetKit {
    include!(concat!(env!("OUT_DIR"), "/WidgetKit.rs"));
}
#[cfg(feature = "iTunesLibrary")]
pub mod iTunesLibrary {
    include!(concat!(env!("OUT_DIR"), "/iTunesLibrary.rs"));
}
#[cfg(feature = "vmnet")]
pub mod vmnet {
    include!(concat!(env!("OUT_DIR"), "/vmnet.rs"));
}
