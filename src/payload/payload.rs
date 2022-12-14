// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `payload.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct Payload {
    // message fields
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pub message: ::protobuf::SingularPtrField<Message>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Payload {
    fn default() -> &'a Payload {
        <Payload as ::protobuf::Message>::default_instance()
    }
}

impl Payload {
    pub fn new() -> Payload {
        ::std::default::Default::default()
    }

    // required bytes public_key = 1;


    pub fn get_public_key(&self) -> &[u8] {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // required bytes signature = 2;


    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // required .Message message = 3;


    pub fn get_message(&self) -> &Message {
        self.message.as_ref().unwrap_or_else(|| <Message as ::protobuf::Message>::default_instance())
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: Message) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut Message {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> Message {
        self.message.take().unwrap_or_else(|| Message::new())
    }
}

impl ::protobuf::Message for Payload {
    fn is_initialized(&self) -> bool {
        if self.public_key.is_none() {
            return false;
        }
        if self.signature.is_none() {
            return false;
        }
        if self.message.is_none() {
            return false;
        }
        for v in &self.message {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.public_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.public_key.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Payload {
        Payload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "public_key",
                |m: &Payload| { &m.public_key },
                |m: &mut Payload| { &mut m.public_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "signature",
                |m: &Payload| { &m.signature },
                |m: &mut Payload| { &mut m.signature },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message>>(
                "message",
                |m: &Payload| { &m.message },
                |m: &mut Payload| { &mut m.message },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Payload>(
                "Payload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Payload {
        static instance: ::protobuf::rt::LazyV2<Payload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Payload::new)
    }
}

impl ::protobuf::Clear for Payload {
    fn clear(&mut self) {
        self.public_key.clear();
        self.signature.clear();
        self.message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    blockchain_id: ::std::option::Option<Blockchain>,
    smart_contract_name: ::protobuf::SingularField<::std::string::String>,
    action: ::std::option::Option<Action>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // required .Blockchain blockchain_id = 1;


    pub fn get_blockchain_id(&self) -> Blockchain {
        self.blockchain_id.unwrap_or(Blockchain::HYPERLEDGER_FABRIC)
    }
    pub fn clear_blockchain_id(&mut self) {
        self.blockchain_id = ::std::option::Option::None;
    }

    pub fn has_blockchain_id(&self) -> bool {
        self.blockchain_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockchain_id(&mut self, v: Blockchain) {
        self.blockchain_id = ::std::option::Option::Some(v);
    }

    // required string smart_contract_name = 2;


    pub fn get_smart_contract_name(&self) -> &str {
        match self.smart_contract_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_smart_contract_name(&mut self) {
        self.smart_contract_name.clear();
    }

    pub fn has_smart_contract_name(&self) -> bool {
        self.smart_contract_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_smart_contract_name(&mut self, v: ::std::string::String) {
        self.smart_contract_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_smart_contract_name(&mut self) -> &mut ::std::string::String {
        if self.smart_contract_name.is_none() {
            self.smart_contract_name.set_default();
        }
        self.smart_contract_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_smart_contract_name(&mut self) -> ::std::string::String {
        self.smart_contract_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // required .Action action = 3;


    pub fn get_action(&self) -> Action {
        self.action.unwrap_or(Action::SET_DATA)
    }
    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: Action) {
        self.action = ::std::option::Option::Some(v);
    }

    // required bytes payload = 4;


    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if self.blockchain_id.is_none() {
            return false;
        }
        if self.smart_contract_name.is_none() {
            return false;
        }
        if self.action.is_none() {
            return false;
        }
        if self.payload.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.blockchain_id, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.smart_contract_name)?;
                },
                3 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.action, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.blockchain_id {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.smart_contract_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockchain_id {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&v))?;
        }
        if let Some(ref v) = self.smart_contract_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.action {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&v))?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Blockchain>>(
                "blockchain_id",
                |m: &Message| { &m.blockchain_id },
                |m: &mut Message| { &mut m.blockchain_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "smart_contract_name",
                |m: &Message| { &m.smart_contract_name },
                |m: &mut Message| { &mut m.smart_contract_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Action>>(
                "action",
                |m: &Message| { &m.action },
                |m: &mut Message| { &mut m.action },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "payload",
                |m: &Message| { &m.payload },
                |m: &mut Message| { &mut m.payload },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message>(
                "Message",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message {
        static instance: ::protobuf::rt::LazyV2<Message> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message::new)
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.blockchain_id = ::std::option::Option::None;
        self.smart_contract_name.clear();
        self.action = ::std::option::Option::None;
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Blockchain {
    HYPERLEDGER_FABRIC = 0,
    HYPERLEDGER_SAWTOOTH = 1,
    SUBSTRATE = 2,
    ETHEREUM = 3,
}

impl ::protobuf::ProtobufEnum for Blockchain {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Blockchain> {
        match value {
            0 => ::std::option::Option::Some(Blockchain::HYPERLEDGER_FABRIC),
            1 => ::std::option::Option::Some(Blockchain::HYPERLEDGER_SAWTOOTH),
            2 => ::std::option::Option::Some(Blockchain::SUBSTRATE),
            3 => ::std::option::Option::Some(Blockchain::ETHEREUM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Blockchain] = &[
            Blockchain::HYPERLEDGER_FABRIC,
            Blockchain::HYPERLEDGER_SAWTOOTH,
            Blockchain::SUBSTRATE,
            Blockchain::ETHEREUM,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Blockchain>("Blockchain", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Blockchain {
}

impl ::std::default::Default for Blockchain {
    fn default() -> Self {
        Blockchain::HYPERLEDGER_FABRIC
    }
}

impl ::protobuf::reflect::ProtobufValue for Blockchain {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Action {
    SET_DATA = 0,
    GET_DATA = 1,
}

impl ::protobuf::ProtobufEnum for Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Action> {
        match value {
            0 => ::std::option::Option::Some(Action::SET_DATA),
            1 => ::std::option::Option::Some(Action::GET_DATA),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Action] = &[
            Action::SET_DATA,
            Action::GET_DATA,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Action>("Action", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Action {
}

impl ::std::default::Default for Action {
    fn default() -> Self {
        Action::SET_DATA
    }
}

impl ::protobuf::reflect::ProtobufValue for Action {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rpayload.proto\"r\n\x07Payload\x12\x1f\n\npublic_key\x18\x01\x20\x02(\
    \x0cR\tpublicKeyB\0\x12\x1e\n\tsignature\x18\x02\x20\x02(\x0cR\tsignatur\
    eB\0\x12$\n\x07message\x18\x03\x20\x02(\x0b2\x08.MessageR\x07messageB\0:\
    \0\"\xb0\x01\n\x07Message\x122\n\rblockchain_id\x18\x01\x20\x02(\x0e2\
    \x0b.BlockchainR\x0cblockchainIdB\0\x120\n\x13smart_contract_name\x18\
    \x02\x20\x02(\tR\x11smartContractNameB\0\x12!\n\x06action\x18\x03\x20\
    \x02(\x0e2\x07.ActionR\x06actionB\0\x12\x1a\n\x07payload\x18\x04\x20\x02\
    (\x0cR\x07payloadB\0:\0*]\n\nBlockchain\x12\x16\n\x12HYPERLEDGER_FABRIC\
    \x10\0\x12\x18\n\x14HYPERLEDGER_SAWTOOTH\x10\x01\x12\r\n\tSUBSTRATE\x10\
    \x02\x12\x0c\n\x08ETHEREUM\x10\x03\x1a\0*&\n\x06Action\x12\x0c\n\x08SET_\
    DATA\x10\0\x12\x0c\n\x08GET_DATA\x10\x01\x1a\0B\0b\x06proto2\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
