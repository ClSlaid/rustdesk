//! clipboard file PDU's abstracted implemetations

const CB_RESPONSE_OK: i32 = 0x0001;
const CB_RESPONSE_FAIL: i32 = 0x0002;

const CB_FILECONTENTS_SIZE: i32 = 0x0001;
const CB_FILECONTENTS_RANGE: i32 = 0x0001;

const CB_REQUEST_SIZE: i32 = 0x0008;

use hbb_common::protobuf::reflect::FileDescriptor;

use crate::ClipboardFile;
pub trait ClipboardPayload: Into<ClipboardFile> + Clone + Debug {}

#[derive(Debug, Clone)]
pub struct NotifyCallback {
    pub ty: String,
    pub title: String,
    pub text: String,
}

impl From<NotifyCallback> for ClipboardFile {
    fn from(value: NotifyCallback) -> Self {
        Self::NotifyCallback {
            r#type: value.ty,
            title: value.title,
            text: value.text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonitorReady {}

impl From<MonitorReady> for ClipboardFile {
    fn from(value: MonitorReady) -> Self {
        Self::MonitorReady
    }
}

#[derive(Debug, Clone)]
pub struct FormatList {
    /// format id and name pairs
    pub format_list: Vec<(i32, String)>, // don't really care about actual number and meaning of those formats
}

impl From<FormatList> for ClipboardFile {
    fn from(value: FormatList) -> Self {
        Self::FormatList {
            format_list: value.format_list,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatListResponse {
    pub success: bool,
}

impl From<FormatListResponse> for ClipboardFile {
    fn from(value: FormatListResponse) -> Self {
        let msg_flags = if value.success {
            CB_RESPONSE_OK
        } else {
            CB_RESPONSE_FAIL
        };
        Self::FormatListResponse { msg_flags }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDataRequest {
    pub format_id: i32,
}
impl From<FormatDataRequest> for ClipboardFile {
    fn from(value: FormatDataRequest) -> Self {
        Self::FormatDataRequest {
            requested_format_id: value.format_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDataResponse {
    /// if fail, got None
    /// if success, got Some
    pub format_data: Option<Vec<u8>>,
}
impl From<FormatDataResponse> for ClipboardFile {
    fn from(value: FromatDataResponse) -> Self {
        if let Some(format_data) = value.format_data {
            Self::FormatDataResponse {
                msg_flags: CB_RESPONSE_OK,
                format_data,
            }
        } else {
            Self::FormatDataResponse {
                msg_flags: CB_RESPONSE_FAIL,
                format_data: vec![],
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct FileContentsRequest {
    stream: u32,
    file_index: u32,
    req_type: FileContentsRequestType,
    clipdata_id: Option<u32>,
}

impl From<FileContentsRequest> for ClipboardFile {
    fn from(value: FileContentsRequest) -> Self {
        let stream_id = value.stream;
        let list_index = value.file_index;
        let (have_clip_data_id, clip_data_id) = if let Some(id) = value.clipdata_id {
            (true, id)
        } else {
            (false, 0)
        };

        let mut req = Self::FileContentsRequest {
            stream_id,
            list_index,
            have_clip_data_id,
            clip_data_id,
            ..Default::default()
        };

        match value.FileContentsRequestType {
            FileContentsRequestType::Size => {
                req.cb_requested = CB_REQUEST_SIZE;
                req.dw_flags = CB_FILECONTENTS_SIZE;
            }
            FileContentsRequestType::Range(offset, size) => {
                req.dw_flags = CB_FILECONTENTS_RANGE;
                let high = (offset >> 32) as u32;
                let low = (offset & u32::MAX) as u32;
                req.cb_requested = size;
                req.n_position_low = low;
                req.n_position_high = high;
            }
        };
        req
    }
}

#[derive(Debug, Clone)]
pub enum FileContentsRequestType {
    /// request file size
    Size,
    /// interpreted as `(offset, size)`
    /// request ranged data transmission from `offset` to `size`.
    Range(u64, u64),
}

#[derive(Debug, Clone)]
pub struct FileContentsResponse {
    pub steam: i32,
    pub payload: Option<FileContentsRequestType>,
}

impl From<FileContentsResponse> for ClipboardFile {
    fn from(value: FileContentsResponse) -> Self {
        let stream_id = value.stream;
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum FileContentsResponsePayload {
    /// requested file size
    Size(u64),
    /// requested file content
    /// TODO: replace with `bytes` to minimize clone
    Contents(Vec<u8>),
}
