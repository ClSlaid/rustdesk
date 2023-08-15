//! Generic implementation for clipboard protocol.

use std::{collections::HashSet, fs::Metadata};

use hbb_common::message_proto::CliprdrFileContentsResponse;

use crate::{clipboard_file::FileContentsRequestType, ClipboardFile};

/// error happens in running client
#[derive(Clone, Debug, Error)]
pub enum ClientError {
    /// error happens on initializing client
    InitError(String),
    /// internal error
    Internal(String),
    /// operation failure
    Fail(String),
}

/// Single element in ClientCapability
///
/// # Note
///
/// The ClientCapabilities PDU can actually be represented
/// as a list of ClientCapabilities, since we always set:
/// - CAPABILITY_SET_TYPE -> GENERAL
/// - CAPABILITY_SET_LENGTH -> 12
/// - CAPABILITY_VERSION -> version 2
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ClientCapability {
    /// client supports long format names and is enabled
    /// if is set, the communication must use long format names only
    LongFormatNames,
    /// file copy with FileContentsRequest and FileContentsResponse is enabled
    StreamFileClip,
    /// Indicates that any description of files to copy and paste MUST NOT
    /// include the source path of the file
    FileClipNoFilePaths,
    /// locking and unlocking of file system data on the clipboard is supported
    /// using Lock Clipboard Data PDU and Unlock Clipboard Data PDU
    LockClipData,
    /// Indicates support for transferring files that are larger than
    /// 4,294,967,295 bytes in size. If this flag is not set, then only files of
    /// size less than or equal to 4,294,967,295 bytes can be exchanged
    /// using the File Contents Request PDU and File Contents
    /// Response PDU.
    ///
    /// TL;DR: support u64 ranges and offsets in file transmission.
    HugeFileSupport,
}

/// able to be FormatDataResponse's payload
pub trait FormatDataPayload {
    fn to_payload(&self) -> Vec<u8>;
}

/// Trait to adapt system clipboard.
///
/// As for a generic clipboard, it should be able to:
///
/// - read text from clipboard
/// - read URI from clipboard and request data from URI
/// - write text to clipboard
/// - write URI to clipboard
///
/// Besides of accessing system clipboard, the client
/// should be able to communicate with the other client.
/// This requires the client to:
///
/// - read and send files in special MIME types
///
/// The client should also implement support for file writing
pub trait ClientContext {
    /// send notify to local msgbox
    fn notify_callback(&self, conn_id: u32, msg: String) -> Result<(), ClientError>;
    /// capability set
    fn capabilities() -> HashSet<ClientCapability>;

    /// create a clipboard client
    fn create() -> Result<Self, ClientError>;
    /// terminate monitor on system clipboard
    fn uninit(self) -> Result<(), ClientError>;
    /// clear all data in clipboard
    fn clear(&self) -> Result<(), ClientError>;

    /// merge received format list from server
    /// # Note
    /// This will update `self` but never do IO
    fn client_format_list(
        &mut self,
        clip_format_list: Vec<(i32, String)>,
    ) -> Result<ClipboardFile, ClientError>;

    /// merge received format list from client
    /// # Note
    /// This will update `self` but never do IO
    fn server_format_list(
        &mut self,
        clip_format_list: Vec<(i32, String)>,
    ) -> Result<ClipboardFile, ClientError>;
    /// response to format list request from server
    /// # Note
    /// This will update `self` but never do IO
    fn client_format_list_response(&self, success: bool) -> Result<ClipboardFile, ClientError>;
    /// request format data from server
    /// # Note
    /// This will update `self` but never do IO
    fn client_format_data_request(&self, format_id: i32) -> Result<ClipboardFile, ClientError>;
    /// response format data request from server
    /// # Note
    /// This will update `self` but should never do IO
    fn client_format_data_response(
        &mut self,
        format_data: impl FormatDataPayload,
    ) -> Result<ClipboardFile, ClientError>;
    /// request file contents
    fn client_file_contents_request(
        &mut self,
        request: FileContentsRequestType,
    ) -> Result<ClipboardFile, ClientError>;

    /// server file contents response
    pub fn server_clip_file(
        &self,
        resp: CliprdrFileContentsResponse,
    ) -> Result<CliprdrFileContentsResponse, ClientError>;
}
