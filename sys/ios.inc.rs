vec![
    #[cfg(feature = "ARKit")] "ARKit",
    #[cfg(feature = "AVFAudio")] "AVFAudio",
    #[cfg(feature = "AVFoundation")] "AVFoundation",
    #[cfg(feature = "AVKit")] "AVKit",
    #[cfg(feature = "AVRouting")] "AVRouting",
    #[cfg(feature = "Accelerate")] "Accelerate",
    #[cfg(feature = "Accessibility")] "Accessibility",
    #[cfg(feature = "Accounts")] "Accounts",
    #[cfg(feature = "ActivityKit")] "ActivityKit",
    #[cfg(feature = "AdServices")] "AdServices",
    #[cfg(feature = "AdSupport")] "AdSupport",
    #[cfg(feature = "AddressBook")] "AddressBook",
    #[cfg(feature = "AddressBookUI")] "AddressBookUI",
    #[cfg(feature = "AppClip")] "AppClip",
    #[cfg(feature = "AppIntents")] "AppIntents",
    #[cfg(feature = "AppTrackingTransparency")] "AppTrackingTransparency",
    #[cfg(feature = "AssetsLibrary")] "AssetsLibrary",
    #[cfg(feature = "AudioToolbox")] "AudioToolbox",
    #[cfg(feature = "AudioUnit")] "AudioUnit",
    #[cfg(feature = "AuthenticationServices")] "AuthenticationServices",
    #[cfg(feature = "AutomaticAssessmentConfiguration")] "AutomaticAssessmentConfiguration",
    #[cfg(feature = "BackgroundAssets")] "BackgroundAssets",
    #[cfg(feature = "BackgroundTasks")] "BackgroundTasks",
    #[cfg(feature = "BusinessChat")] "BusinessChat",
    #[cfg(feature = "CFNetwork")] "CFNetwork",
    #[cfg(feature = "CallKit")] "CallKit",
    #[cfg(feature = "CarPlay")] "CarPlay",
    #[cfg(feature = "ClassKit")] "ClassKit",
    #[cfg(feature = "ClockKit")] "ClockKit",
    #[cfg(feature = "CloudKit")] "CloudKit",
    #[cfg(feature = "ColorSync")] "ColorSync",
    #[cfg(feature = "Contacts")] "Contacts",
    #[cfg(feature = "ContactsUI")] "ContactsUI",
    #[cfg(feature = "CoreAudio")] "CoreAudio",
    #[cfg(feature = "CoreAudioKit")] "CoreAudioKit",
    #[cfg(feature = "CoreBluetooth")] "CoreBluetooth",
    #[cfg(feature = "CoreData")] "CoreData",
    #[cfg(feature = "CoreFoundation")] "CoreFoundation",
    #[cfg(feature = "CoreGraphics")] "CoreGraphics",
    #[cfg(feature = "CoreHaptics")] "CoreHaptics",
    #[cfg(feature = "CoreImage")] "CoreImage",
    #[cfg(feature = "CoreLocation")] "CoreLocation",
    #[cfg(feature = "CoreLocationUI")] "CoreLocationUI",
    #[cfg(feature = "CoreMIDI")] "CoreMIDI",
    #[cfg(feature = "CoreML")] "CoreML",
    #[cfg(feature = "CoreMedia")] "CoreMedia",
    #[cfg(feature = "CoreMotion")] "CoreMotion",
    #[cfg(feature = "CoreNFC")] "CoreNFC",
    #[cfg(feature = "CoreServices")] "CoreServices",
    #[cfg(feature = "CoreSpotlight")] "CoreSpotlight",
    #[cfg(feature = "CoreTelephony")] "CoreTelephony",
    #[cfg(feature = "CoreText")] "CoreText",
    #[cfg(feature = "CoreTransferable")] "CoreTransferable",
    #[cfg(feature = "CoreVideo")] "CoreVideo",
    #[cfg(feature = "CryptoTokenKit")] "CryptoTokenKit",
    #[cfg(feature = "DataDetection")] "DataDetection",
    #[cfg(feature = "DeviceCheck")] "DeviceCheck",
    #[cfg(feature = "DeviceDiscoveryExtension")] "DeviceDiscoveryExtension",
    #[cfg(feature = "EventKit")] "EventKit",
    #[cfg(feature = "EventKitUI")] "EventKitUI",
    #[cfg(feature = "ExposureNotification")] "ExposureNotification",
    #[cfg(feature = "ExtensionFoundation")] "ExtensionFoundation",
    #[cfg(feature = "ExtensionKit")] "ExtensionKit",
    #[cfg(feature = "ExternalAccessory")] "ExternalAccessory",
    #[cfg(feature = "FileProvider")] "FileProvider",
    #[cfg(feature = "FileProviderUI")] "FileProviderUI",
    #[cfg(feature = "Foundation")] "Foundation",
    #[cfg(feature = "GLKit")] "GLKit",
    #[cfg(feature = "GSS")] "GSS",
    #[cfg(feature = "GameController")] "GameController",
    #[cfg(feature = "GameKit")] "GameKit",
    #[cfg(feature = "GameplayKit")] "GameplayKit",
    #[cfg(feature = "HealthKit")] "HealthKit",
    #[cfg(feature = "HealthKitUI")] "HealthKitUI",
    #[cfg(feature = "HomeKit")] "HomeKit",
    #[cfg(feature = "IOSurface")] "IOSurface",
    #[cfg(feature = "IdentityLookup")] "IdentityLookup",
    #[cfg(feature = "IdentityLookupUI")] "IdentityLookupUI",
    #[cfg(feature = "ImageCaptureCore")] "ImageCaptureCore",
    #[cfg(feature = "ImageIO")] "ImageIO",
    #[cfg(feature = "Intents")] "Intents",
    #[cfg(feature = "IntentsUI")] "IntentsUI",
    #[cfg(feature = "JavaScriptCore")] "JavaScriptCore",
    #[cfg(feature = "LinkPresentation")] "LinkPresentation",
    #[cfg(feature = "LocalAuthentication")] "LocalAuthentication",
    #[cfg(feature = "LocalAuthenticationEmbeddedUI")] "LocalAuthenticationEmbeddedUI",
    #[cfg(feature = "MLCompute")] "MLCompute",
    #[cfg(feature = "MapKit")] "MapKit",
    #[cfg(feature = "Matter")] "Matter",
    #[cfg(feature = "MatterSupport")] "MatterSupport",
    #[cfg(feature = "MediaAccessibility")] "MediaAccessibility",
    #[cfg(feature = "MediaPlayer")] "MediaPlayer",
    #[cfg(feature = "MediaSetup")] "MediaSetup",
    #[cfg(feature = "MediaToolbox")] "MediaToolbox",
    #[cfg(feature = "MessageUI")] "MessageUI",
    #[cfg(feature = "Messages")] "Messages",
    #[cfg(feature = "Metal")] "Metal",
    #[cfg(feature = "MetalFX")] "MetalFX",
    #[cfg(feature = "MetalKit")] "MetalKit",
    #[cfg(feature = "MetricKit")] "MetricKit",
    #[cfg(feature = "MobileCoreServices")] "MobileCoreServices",
    #[cfg(feature = "ModelIO")] "ModelIO",
    #[cfg(feature = "MultipeerConnectivity")] "MultipeerConnectivity",
    #[cfg(feature = "NaturalLanguage")] "NaturalLanguage",
    #[cfg(feature = "NearbyInteraction")] "NearbyInteraction",
    #[cfg(feature = "Network")] "Network",
    #[cfg(feature = "NetworkExtension")] "NetworkExtension",
    #[cfg(feature = "NewsstandKit")] "NewsstandKit",
    #[cfg(feature = "NotificationCenter")] "NotificationCenter",
    #[cfg(feature = "OSLog")] "OSLog",
    #[cfg(feature = "OpenAL")] "OpenAL",
    #[cfg(feature = "OpenGLES")] "OpenGLES",
    #[cfg(feature = "PDFKit")] "PDFKit",
    #[cfg(feature = "PHASE")] "PHASE",
    #[cfg(feature = "PassKit")] "PassKit",
    #[cfg(feature = "PencilKit")] "PencilKit",
    #[cfg(feature = "Photos")] "Photos",
    #[cfg(feature = "PhotosUI")] "PhotosUI",
    #[cfg(feature = "ProximityReader")] "ProximityReader",
    #[cfg(feature = "PushKit")] "PushKit",
    #[cfg(feature = "PushToTalk")] "PushToTalk",
    #[cfg(feature = "QuartzCore")] "QuartzCore",
    #[cfg(feature = "QuickLook")] "QuickLook",
    #[cfg(feature = "QuickLookThumbnailing")] "QuickLookThumbnailing",
    #[cfg(feature = "ReplayKit")] "ReplayKit",
    #[cfg(feature = "RoomPlan")] "RoomPlan",
    #[cfg(feature = "SafariServices")] "SafariServices",
    #[cfg(feature = "SafetyKit")] "SafetyKit",
    #[cfg(feature = "SceneKit")] "SceneKit",
    #[cfg(feature = "ScreenTime")] "ScreenTime",
    #[cfg(feature = "Security")] "Security",
    #[cfg(feature = "SensorKit")] "SensorKit",
    #[cfg(feature = "SharedWithYou")] "SharedWithYou",
    #[cfg(feature = "SharedWithYouCore")] "SharedWithYouCore",
    #[cfg(feature = "ShazamKit")] "ShazamKit",
    #[cfg(feature = "Social")] "Social",
    #[cfg(feature = "SoundAnalysis")] "SoundAnalysis",
    #[cfg(feature = "Speech")] "Speech",
    #[cfg(feature = "SpriteKit")] "SpriteKit",
    #[cfg(feature = "StoreKit")] "StoreKit",
    #[cfg(feature = "SwiftUI")] "SwiftUI",
    #[cfg(feature = "SystemConfiguration")] "SystemConfiguration",
    #[cfg(feature = "ThreadNetwork")] "ThreadNetwork",
    #[cfg(feature = "Twitter")] "Twitter",
    #[cfg(feature = "UIKit")] "UIKit",
    #[cfg(feature = "UniformTypeIdentifiers")] "UniformTypeIdentifiers",
    #[cfg(feature = "UserNotifications")] "UserNotifications",
    #[cfg(feature = "UserNotificationsUI")] "UserNotificationsUI",
    #[cfg(feature = "VideoSubscriberAccount")] "VideoSubscriberAccount",
    #[cfg(feature = "VideoToolbox")] "VideoToolbox",
    #[cfg(feature = "Vision")] "Vision",
    #[cfg(feature = "VisionKit")] "VisionKit",
    #[cfg(feature = "WatchConnectivity")] "WatchConnectivity",
    #[cfg(feature = "WebKit")] "WebKit",
    #[cfg(feature = "WidgetKit")] "WidgetKit",
    #[cfg(feature = "iAd")] "iAd",
]
