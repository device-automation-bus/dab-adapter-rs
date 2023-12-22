use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type SharedMap = 
    HashMap<String, RequestTypes>;

#[derive(Debug)]
pub enum DabError {
    Err400(String),
    Err500(String),
    Err501(String),
}

#[derive(Clone)]
pub enum RequestTypes {	
    OperationsListRequest,
    ApplicationListRequest,
    ApplicationLaunchRequest,
    ApplicationLaunchWithContentRequest,
    ApplicationGetStateRequest,
    ApplicationExitRequest,
    DeviceInfoRequest,
    SystemRestartRequest,
    SystemSettingsListRequest,
    SystemSettingsGetRequest,
    SystemSettingsSetRequest,
    InputKeyListRequest,
    InputKeyPressRequest,
    InputLongKeyPressRequest,
    OutputImageRequest,
    HealthCheckGetRequest,
    VoiceListRequest,
    VoiceSetRequest,
    VoiceSendAudioRequest,
    VoiceSendTextRequest,
    VersionRequest,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    appId: Option<String>,
    force: Option<bool>,
    keyCode: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DabResponse {
    pub status: u16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiscoveryResponse {
    pub ip: String,
    pub deviceId: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceTelemetryStartResponse {
    pub status: u16,
    pub duration: u64,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Default, Serialize, Deserialize)]
pub enum NotificationLevel {
    #[default]
    info,
    warn,
    debug,
    trace,
    error,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Messages {
    pub timestamp: u64,
    pub level: NotificationLevel,
    pub ip: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryMessage {
    pub timestamp: u64,
    pub metric: String,
    pub value: u32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct HealthCheckRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub healthy: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct DeviceInfoRequest {}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum NetworkInterfaceType {
    #[default]
    Ethernet,
    Wifi,
    Bluetooth,
    Coax,
    Other,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum DisplayType {
    #[default]
    Native,
    External,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub connected: bool,
    pub macAddress: String,
    pub ipAddress: String,
    pub dns: Vec<String>,
    pub r#type: NetworkInterfaceType,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetDeviceInformationResponse {
    pub manufacturer: String,
    pub model: String,
    pub serialNumber: String,
    pub chipset: String,
    pub firmwareVersion: String,
    pub firmwareBuild: String,
    pub networkInterfaces: Vec<NetworkInterface>,
    pub displayType: DisplayType,
    pub screenWidthPixels: u32,
    pub screenHeightPixels: u32,
    pub uptimeSince: u64,
    pub deviceId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VersionRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct Version {
    pub versions: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopApplicationTelemetryRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopApplicationTelemetryResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartApplicationTelemetryRequest {
    pub appId: String,
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartApplicationTelemetryResponse {
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct OperationsListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListSupportedOperation {
    pub operations: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetVoiceSystemRequest {
    pub voiceSystem: VoiceSystem,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetVoiceSystemResponse {
    pub voiceSystem: VoiceSystem,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SendTextRequest {
    pub requestText: String,
    pub voiceSystem: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VoiceTextRequestResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VoiceListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListVoiceSystemsResponse {
    pub voiceSystems: Vec<VoiceSystem>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VoiceSystem {
    pub name: String,
    pub enabled: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SendAudioRequest {
    pub fileLocation: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct VoiceRequestResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SettingsGetRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct OutputResolution {
    pub width: u32,
    pub height: u32,
    pub frequency: f32,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum MatchContentFrameRate {
    #[default]
    EnabledAlways,
    EnabledSeamlessOnly,
    Disabled,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum HdrOutputMode {
    AlwaysHdr,
    HdrOnPlayback,
    #[default]
    DisableHdr,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum PictureMode {
    #[default]
    Standard,
    Dynamic,
    Movie,
    Sports,
    FilmMaker,
    Game,
    Auto,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum AudioOutputMode {
    #[default]
    Stereo,
    MultichannelPcm,
    PassThrough,
    Auto,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum AudioOutputSource {
    NativeSpeaker,
    Arc,
    EArc,
    Optical,
    Aux,
    Bluetooth,
    Auto,
    #[default]
    HDMI,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
pub enum VideoInputSource {
    Tuner,
    HDMI1,
    HDMI2,
    HDMI3,
    HDMI4,
    Composite,
    Component,
    #[default]
    Home,
    Cast,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct AudioVolume {
    pub min: u32,
    pub max: u32,
}
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListSystemSettingsRequest{

}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListSystemSettingsResponse {
    pub language: Vec<String>,
    pub outputResolution: Vec<OutputResolution>,
    pub memc: bool,
    pub cec: bool,
    pub lowLatencyMode: bool,
    pub matchContentFrameRate: Vec<MatchContentFrameRate>,
    pub hdrOutputMode: Vec<HdrOutputMode>,
    pub pictureMode: Vec<PictureMode>,
    pub audioOutputMode: Vec<AudioOutputMode>,
    pub audioOutputSource: Vec<AudioOutputSource>,
    pub videoInputSource: Vec<VideoInputSource>,
    pub audioVolume: AudioVolume,
    pub mute: bool,
    pub textToSpeech: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetSystemSettingsRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetSystemSettingsResponse {
    pub language: String,
    pub outputResolution: OutputResolution,
    pub memc: bool,
    pub cec: bool,
    pub lowLatencyMode: bool,
    pub matchContentFrameRate: MatchContentFrameRate,
    pub hdrOutputMode: HdrOutputMode,
    pub pictureMode: PictureMode,
    pub audioOutputMode: AudioOutputMode,
    pub audioOutputSource: AudioOutputSource,
    pub videoInputSource: VideoInputSource,
    pub audioVolume: u32,
    pub mute: bool,
    pub textToSpeech: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct SetSystemSettingsRequest {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub outputResolution: OutputResolution,
    #[serde(default)]
    pub memc: bool,
    #[serde(default)]
    pub cec: bool,
    #[serde(default)]
    pub lowLatencyMode: bool,
    #[serde(default)]
    pub matchContentFrameRate: MatchContentFrameRate,
    #[serde(default)]
    pub hdrOutputMode: HdrOutputMode,
    #[serde(default)]
    pub pictureMode: PictureMode,
    #[serde(default)]
    pub audioOutputMode: AudioOutputMode,
    #[serde(default)]
    pub audioOutputSource: AudioOutputSource,
    #[serde(default)]
    pub videoInputSource: VideoInputSource,
    #[serde(default)]
    pub audioVolume: u32,
    #[serde(default)]
    pub mute: bool,
    #[serde(default)]
    pub textToSpeech: bool,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct RestartRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct RestartResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetApplicationStateRequest {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct GetApplicationStateResponse {
    pub state: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentRequest {
    pub appId: String,
    pub contentId: String,
    pub parameters: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationWithContentResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationRequest {
    pub appId: String,
    pub parameters: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LaunchApplicationResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ApplicationListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct Application {
    pub appId: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ListApplicationsResponse {
    pub applications: Vec<Application>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ExitApplicationRequest {
    pub appId: String,
    pub background: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct ExitApplicationResponse {
    pub state: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct KeyListRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct KeyList {
    pub keyCodes: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct KeyPressRequest {
    pub keyCode: String,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct KeyPressResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LongKeyPressRequest {
    pub keyCode: String,
    pub durationMs: u32,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct LongKeyPressResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct CaptureScreenshotRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct CaptureScreenshotResponse {
    pub outputImage: String,
}

// Implement device-telemetry
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopDeviceTelemetryResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryRequest {
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartDeviceTelemetryResponse {
    pub duration: u64,
}

// Implement device-telemetry
#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopAppTelemetryRequest {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StopAppTelemetryResponse {}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartAppTelemetryRequest {
    pub app_id: String,
    pub duration: u64,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct StartAppTelemetryResponse {
    pub app_id: String,
    pub duration: u64,
}