use std::{fmt, net::IpAddr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseCode {
    Successful = 0,
    Initializing = 1,
    InternalError = 2,
    InvalidRequest = 3,
    InvalidParameter = 4,
    Guarded = 5,
    TimeOut = 6,
    FirmwareUpdating = 99,
    AccessError = 100,
    OtherErrors = 101,
    WrongUserName = 102,
    WrongPassword = 103,
    AccountExpired = 104,
    AccountDisconnected = 105,
    AccountLimitReached = 106,
    ServerMaintenance = 107,
    InvalidAccount = 108,
    LicenseError = 109,
    ReadOnlyMode = 110,
    MaxStations = 111,
    AccessDenied = 112,
    NeedSpecifyPlaylist = 113,
    NeedCreatePlaylist = 114,
    SimultaneousLoginsLimit = 115,
    LinkingInProgress = 200,
    UnlinkingInProgress = 201,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ResponseCode::Successful => "Successful",
            ResponseCode::Initializing => "Initializing",
            ResponseCode::InternalError => "Internal Error",
            ResponseCode::InvalidRequest => "Invalid Request",
            ResponseCode::InvalidParameter => "Invalid Parameter",
            ResponseCode::Guarded => "Guarded (Operation not allowed)",
            ResponseCode::TimeOut => "Request Timed Out",
            ResponseCode::FirmwareUpdating => "Firmware Updating",
            ResponseCode::AccessError => "Access Error",
            ResponseCode::OtherErrors => "Other Error",
            ResponseCode::WrongUserName => "Wrong Username",
            ResponseCode::WrongPassword => "Wrong Password",
            ResponseCode::AccountExpired => "Account Expired",
            ResponseCode::AccountDisconnected => "Account Disconnected",
            ResponseCode::AccountLimitReached => "Account Limit Reached",
            ResponseCode::ServerMaintenance => "Server Under Maintenance",
            ResponseCode::InvalidAccount => "Invalid Account",
            ResponseCode::LicenseError => "License Error",
            ResponseCode::ReadOnlyMode => "Read-Only Mode",
            ResponseCode::MaxStations => "Maximum Stations Reached",
            ResponseCode::AccessDenied => "Access Denied",
            ResponseCode::NeedSpecifyPlaylist => "Playlist Must Be Specified",
            ResponseCode::NeedCreatePlaylist => "Playlist Must Be Created",
            ResponseCode::SimultaneousLoginsLimit => "Simultaneous Logins Limit Reached",
            ResponseCode::LinkingInProgress => "Linking In Progress",
            ResponseCode::UnlinkingInProgress => "Unlinking In Progress",
        };
        write!(f, "{}", description)
    }
}

impl From<u32> for ResponseCode {
    fn from(code: u32) -> Self {
        match code {
            0 => ResponseCode::Successful,
            1 => ResponseCode::Initializing,
            2 => ResponseCode::InternalError,
            3 => ResponseCode::InvalidRequest,
            4 => ResponseCode::InvalidParameter,
            5 => ResponseCode::Guarded,
            6 => ResponseCode::TimeOut,
            99 => ResponseCode::FirmwareUpdating,
            100 => ResponseCode::AccessError,
            101 => ResponseCode::OtherErrors,
            102 => ResponseCode::WrongUserName,
            103 => ResponseCode::WrongPassword,
            104 => ResponseCode::AccountExpired,
            105 => ResponseCode::AccountDisconnected,
            106 => ResponseCode::AccountLimitReached,
            107 => ResponseCode::ServerMaintenance,
            108 => ResponseCode::InvalidAccount,
            109 => ResponseCode::LicenseError,
            110 => ResponseCode::ReadOnlyMode,
            111 => ResponseCode::MaxStations,
            112 => ResponseCode::AccessDenied,
            113 => ResponseCode::NeedSpecifyPlaylist,
            114 => ResponseCode::NeedCreatePlaylist,
            115 => ResponseCode::SimultaneousLoginsLimit,
            200 => ResponseCode::LinkingInProgress,
            201 => ResponseCode::UnlinkingInProgress,
            _ => ResponseCode::OtherErrors,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YamahaDevice {
    pub ip: IpAddr,
    pub name: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DeviceInfo {
    pub model_name: String,
    pub destination: String,
    pub device_id: String,
    pub system_id: String,
    pub system_version: f32,
    pub api_version: f32,
    pub netmodule_generation: u32,
    pub netmodule_version: String,
    pub netmodule_checksum: String,
    pub serial_number: String,
    pub operation_mode: String,
    pub update_error_code: String,
    pub net_module_num: u32,
    pub update_data_type: u32,
    pub analytics_info: AnalyticsInfo,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AnalyticsInfo {
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ZoneStatus {
    pub power: String,
    pub sleep: u32,
    pub volume: u32,
    pub mute: bool,
    pub max_volume: u32,
    pub input: String,
    pub input_text: String,
    pub distribution_enable: bool,
    pub sound_program: String,
    pub surr_decoder_type: String,
    pub pure_direct: bool,
    pub enhancer: bool,
    pub tone_control: ToneControl,
    pub dialogue_level: u32,
    pub dialogue_lift: u32,
    pub subwoofer_volume: u32,
    pub link_control: String,
    pub link_audio_delay: String,
    pub disable_flags: u32,
    pub contents_display: bool,
    pub actual_volume: ActualVolume,
    pub party_enable: bool,
    pub extra_bass: bool,
    pub adaptive_drc: bool,
    pub dts_dialogue_control: u32,
    pub adaptive_dsp_level: bool,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ToneControl {
    pub mode: String,
    pub bass: u32,
    pub treble: u32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ActualVolume {
    pub mode: String,
    pub value: f32,
    pub unit: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ZoneProgramList {
    pub sound_program_list: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SignalInfo {
    pub audio: AudioSignal,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct AudioSignal {
    pub error: u32,
    pub format: String,
    pub fs: String,
    pub bitrate: u32,
    pub bit: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DeviceFeatures {
    pub system: System,
    pub zone: Vec<Zone>,
    pub tuner: Tuner,
    pub netusb: NetUsb,
    pub distribution: Distribution,
    pub ccs: Ccs,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct System {
    pub func_list: Vec<String>,
    pub zone_num: i32,
    pub input_list: Vec<SystemInput>,
    pub bluetooth: Option<SystemBluetooth>,
    pub web_control_url: Option<String>,
    pub party_volume_list: Option<Vec<String>>,
    pub hdmi_standby_through_list: Option<Vec<String>>,
    pub works_with_sonos: Option<WorksWithSonos>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SystemInput {
    pub id: String,
    pub distribution_enable: bool,
    pub rename_enable: bool,
    pub account_enable: bool,
    pub play_info_type: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SystemBluetooth {
    pub update_cancelable: bool,
    pub tx_connectivity_type_max: Option<i32>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct WorksWithSonos {
    pub zone: Vec<SonosZone>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct SonosZone {
    pub id: String,
    pub input_list: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Zone {
    pub id: String,
    pub zone_b: Option<bool>,
    pub func_list: Vec<String>,
    pub input_list: Vec<String>,
    pub sound_program_list: Option<Vec<String>>,
    pub surr_decoder_type_list: Option<Vec<String>>,
    pub tone_control_mode_list: Option<Vec<String>>,
    pub link_control_list: Option<Vec<String>>,
    pub link_audio_delay_list: Option<Vec<String>>,
    #[serde(default)]
    pub range_step: Vec<RangeStep>,
    pub scene_num: Option<i32>,
    pub cursor_list: Option<Vec<String>>,
    pub menu_list: Option<Vec<String>>,
    pub actual_volume_mode_list: Option<Vec<String>>,
    pub ccs_supported: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct RangeStep {
    pub id: String,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Tuner {
    pub func_list: Vec<String>,
    pub range_step: Vec<TunerRangeStep>,
    pub preset: TunerPreset,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct TunerRangeStep {
    pub id: String,
    pub min: i32,
    pub max: i32,
    pub step: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct TunerPreset {
    pub r#type: String,
    pub num: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsb {
    pub func_list: Vec<String>,
    pub preset: NetUsbPreset,
    pub recent_info: NetUsbRecentInfo,
    pub play_queue: NetUsbQueue,
    pub mc_playlist: NetUsbMcPlaylist,
    pub net_radio_type: String,
    pub tidal: Option<NetUsbTidal>,
    pub qobuz: Option<NetUsbQobuz>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbPreset {
    pub num: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbRecentInfo {
    pub num: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbQueue {
    pub size: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbMcPlaylist {
    pub size: i32,
    pub num: i32,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbTidal {
    pub mode: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbQobuz {
    pub login_type: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Distribution {
    pub version: f32,
    pub compatible_client: Vec<i32>,
    pub client_max: i32,
    pub server_zone_list: Vec<String>,
    pub mc_surround: Option<McSurround>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct McSurround {
    pub version: f32,
    pub func_list: Vec<String>,
    pub master_role: McRole,
    pub slave_role: McRole,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct McRole {
    pub surround_pair: Option<bool>,
    pub stereo_pair: Option<bool>,
    pub subwoofer_pair: Option<bool>,
    pub surround_pair_l_or_r: Option<bool>,
    pub surround_pair_lr: Option<bool>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Ccs {
    pub supported: bool,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NetUsbPlayInfo {
    pub input: String,
    pub play_queue_type: Option<String>,
    pub playback: String,
    pub repeat: String,
    pub shuffle: String,

    #[serde(default)]
    pub repeat_available: Option<Vec<String>>,
    #[serde(default)]
    pub shuffle_available: Option<Vec<String>>,

    pub play_time: i32,
    pub total_time: i32,

    pub artist: String,
    pub album: String,
    pub track: String,

    pub albumart_url: String,
    pub albumart_id: i32,

    pub usb_devicetype: String,

    #[serde(default)]
    pub auto_stopped: Option<bool>,

    pub attribute: u32,
}
