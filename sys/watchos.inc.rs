vec![
    #[cfg(feature = "AVFAudio")] "AVFAudio",
    #[cfg(feature = "AVFoundation")] "AVFoundation",
    #[cfg(feature = "AVKit")] "AVKit",
    #[cfg(feature = "Accelerate")] "Accelerate",
    #[cfg(feature = "Accessibility")] "Accessibility",
    #[cfg(feature = "AppIntents")] "AppIntents",
    #[cfg(feature = "AuthenticationServices")] "AuthenticationServices",
    #[cfg(feature = "CallKit")] "CallKit",
    #[cfg(feature = "ClockKit")] "ClockKit",
    #[cfg(feature = "CloudKit")] "CloudKit",
    #[cfg(feature = "ColorSync")] "ColorSync",
    #[cfg(feature = "Contacts")] "Contacts",
    #[cfg(feature = "CoreAudio")] "CoreAudio",
    #[cfg(feature = "CoreBluetooth")] "CoreBluetooth",
    #[cfg(feature = "CoreData")] "CoreData",
    #[cfg(feature = "CoreFoundation")] "CoreFoundation",
    #[cfg(feature = "CoreGraphics")] "CoreGraphics",
    #[cfg(feature = "CoreLocation")] "CoreLocation",
    #[cfg(feature = "CoreLocationUI")] "CoreLocationUI",
    #[cfg(feature = "CoreMIDI")] "CoreMIDI",
    #[cfg(feature = "CoreML")] "CoreML",
    #[cfg(feature = "CoreMedia")] "CoreMedia",
    #[cfg(feature = "CoreMotion")] "CoreMotion",
    #[cfg(feature = "CoreServices")] "CoreServices",
    #[cfg(feature = "CoreText")] "CoreText",
    #[cfg(feature = "CoreTransferable")] "CoreTransferable",
    #[cfg(feature = "CoreVideo")] "CoreVideo",
    #[cfg(feature = "CryptoTokenKit")] "CryptoTokenKit",
    #[cfg(feature = "DataDetection")] "DataDetection",
    #[cfg(feature = "DeviceCheck")] "DeviceCheck",
    #[cfg(feature = "EventKit")] "EventKit",
    #[cfg(feature = "ExtensionFoundation")] "ExtensionFoundation",
    #[cfg(feature = "ExtensionKit")] "ExtensionKit",
    #[cfg(feature = "Foundation")] "Foundation",
    #[cfg(feature = "GameKit")] "GameKit",
    #[cfg(feature = "HealthKit")] "HealthKit",
    #[cfg(feature = "HomeKit")] "HomeKit",
    #[cfg(feature = "ImageIO")] "ImageIO",
    #[cfg(feature = "Intents")] "Intents",
    #[cfg(feature = "LocalAuthentication")] "LocalAuthentication",
    #[cfg(feature = "MapKit")] "MapKit",
    #[cfg(feature = "Matter")] "Matter",
    #[cfg(feature = "MediaPlayer")] "MediaPlayer",
    #[cfg(feature = "MobileCoreServices")] "MobileCoreServices",
    #[cfg(feature = "NaturalLanguage")] "NaturalLanguage",
    #[cfg(feature = "NearbyInteraction")] "NearbyInteraction",
    #[cfg(feature = "Network")] "Network",
    #[cfg(feature = "NetworkExtension")] "NetworkExtension",
    #[cfg(feature = "OSLog")] "OSLog",
    #[cfg(feature = "PassKit")] "PassKit",
    #[cfg(feature = "PhotosUI")] "PhotosUI",
    #[cfg(feature = "PushKit")] "PushKit",
    #[cfg(feature = "SafetyKit")] "SafetyKit",
    #[cfg(feature = "SceneKit")] "SceneKit",
    #[cfg(feature = "Security")] "Security",
    #[cfg(feature = "ShazamKit")] "ShazamKit",
    #[cfg(feature = "SoundAnalysis")] "SoundAnalysis",
    #[cfg(feature = "SpriteKit")] "SpriteKit",
    #[cfg(feature = "StoreKit")] "StoreKit",
    #[cfg(feature = "SwiftUI")] "SwiftUI",
    #[cfg(feature = "UIKit")] "UIKit",
    #[cfg(feature = "UniformTypeIdentifiers")] "UniformTypeIdentifiers",
    #[cfg(feature = "UserNotifications")] "UserNotifications",
    #[cfg(feature = "UserNotificationsUI")] "UserNotificationsUI",
    #[cfg(feature = "WatchConnectivity")] "WatchConnectivity",
    #[cfg(feature = "WatchKit")] "WatchKit",
    #[cfg(feature = "WidgetKit")] "WidgetKit",
]
