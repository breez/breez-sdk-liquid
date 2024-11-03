// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub revision: i64,
    #[prost(float, tag = "3")]
    pub schema_version: f32,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRecordRequest {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    #[prost(uint32, tag = "2")]
    pub request_time: u32,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SetRecordReply {
    #[prost(enumeration = "SetRecordStatus", tag = "1")]
    pub status: i32,
    #[prost(int64, tag = "2")]
    pub new_revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChangesRequest {
    #[prost(int64, tag = "1")]
    pub since_revision: i64,
    #[prost(uint32, tag = "2")]
    pub request_time: u32,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChangesReply {
    #[prost(message, repeated, tag = "1")]
    pub changes: ::prost::alloc::vec::Vec<Record>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackChangesRequest {
    #[prost(uint32, tag = "1")]
    pub request_time: u32,
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetRecordStatus {
    Success = 0,
    Conflict = 1,
}
impl SetRecordStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Conflict => "CONFLICT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "CONFLICT" => Some(Self::Conflict),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod syncer_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SyncerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SyncerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SyncerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SyncerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SyncerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn set_record(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRecordRequest>,
        ) -> std::result::Result<tonic::Response<super::SetRecordReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sync.Syncer/SetRecord");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sync.Syncer", "SetRecord"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChangesRequest>,
        ) -> std::result::Result<tonic::Response<super::ListChangesReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sync.Syncer/ListChanges");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sync.Syncer", "ListChanges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn track_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::TrackChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Record>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sync.Syncer/TrackChanges");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("sync.Syncer", "TrackChanges"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
