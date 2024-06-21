// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `GachaInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GachaInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GachaInfo {
    // message fields
    // @@protoc_insertion_point(field:GachaInfo.end_time)
    pub end_time: i64,
    // @@protoc_insertion_point(field:GachaInfo.begin_time)
    pub begin_time: i64,
    // @@protoc_insertion_point(field:GachaInfo.up_info)
    pub up_info: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GachaInfo.featured)
    pub featured: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GachaInfo.history_url)
    pub history_url: ::std::string::String,
    // @@protoc_insertion_point(field:GachaInfo.detail_url)
    pub detail_url: ::std::string::String,
    // @@protoc_insertion_point(field:GachaInfo.gacha_ceiling)
    pub gacha_ceiling: ::protobuf::MessageField<super::GachaCeiling::GachaCeiling>,
    // @@protoc_insertion_point(field:GachaInfo.gacha_id)
    pub gacha_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GachaInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GachaInfo {
    fn default() -> &'a GachaInfo {
        <GachaInfo as ::protobuf::Message>::default_instance()
    }
}

impl GachaInfo {
    pub fn new() -> GachaInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &GachaInfo| { &m.end_time },
            |m: &mut GachaInfo| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &GachaInfo| { &m.begin_time },
            |m: &mut GachaInfo| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "up_info",
            |m: &GachaInfo| { &m.up_info },
            |m: &mut GachaInfo| { &mut m.up_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "featured",
            |m: &GachaInfo| { &m.featured },
            |m: &mut GachaInfo| { &mut m.featured },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "history_url",
            |m: &GachaInfo| { &m.history_url },
            |m: &mut GachaInfo| { &mut m.history_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "detail_url",
            |m: &GachaInfo| { &m.detail_url },
            |m: &mut GachaInfo| { &mut m.detail_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GachaCeiling::GachaCeiling>(
            "gacha_ceiling",
            |m: &GachaInfo| { &m.gacha_ceiling },
            |m: &mut GachaInfo| { &mut m.gacha_ceiling },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_id",
            |m: &GachaInfo| { &m.gacha_id },
            |m: &mut GachaInfo| { &mut m.gacha_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GachaInfo>(
            "GachaInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GachaInfo {
    const NAME: &'static str = "GachaInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.end_time = is.read_int64()?;
                },
                40 => {
                    self.begin_time = is.read_int64()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.up_info)?;
                },
                80 => {
                    self.up_info.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.featured)?;
                },
                32 => {
                    self.featured.push(is.read_uint32()?);
                },
                58 => {
                    self.history_url = is.read_string()?;
                },
                18 => {
                    self.detail_url = is.read_string()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gacha_ceiling)?;
                },
                72 => {
                    self.gacha_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.end_time != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.end_time);
        }
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.begin_time);
        }
        for value in &self.up_info {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.featured {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if !self.history_url.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.history_url);
        }
        if !self.detail_url.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.detail_url);
        }
        if let Some(v) = self.gacha_ceiling.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.gacha_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.gacha_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.end_time != 0 {
            os.write_int64(3, self.end_time)?;
        }
        if self.begin_time != 0 {
            os.write_int64(5, self.begin_time)?;
        }
        for v in &self.up_info {
            os.write_uint32(10, *v)?;
        };
        for v in &self.featured {
            os.write_uint32(4, *v)?;
        };
        if !self.history_url.is_empty() {
            os.write_string(7, &self.history_url)?;
        }
        if !self.detail_url.is_empty() {
            os.write_string(2, &self.detail_url)?;
        }
        if let Some(v) = self.gacha_ceiling.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.gacha_id != 0 {
            os.write_uint32(9, self.gacha_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GachaInfo {
        GachaInfo::new()
    }

    fn clear(&mut self) {
        self.end_time = 0;
        self.begin_time = 0;
        self.up_info.clear();
        self.featured.clear();
        self.history_url.clear();
        self.detail_url.clear();
        self.gacha_ceiling.clear();
        self.gacha_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GachaInfo {
        static instance: GachaInfo = GachaInfo {
            end_time: 0,
            begin_time: 0,
            up_info: ::std::vec::Vec::new(),
            featured: ::std::vec::Vec::new(),
            history_url: ::std::string::String::new(),
            detail_url: ::std::string::String::new(),
            gacha_ceiling: ::protobuf::MessageField::none(),
            gacha_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GachaInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GachaInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GachaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GachaInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fGachaInfo.proto\x1a\x12GachaCeiling.proto\"\x89\x02\n\tGachaInfo\
    \x12\x19\n\x08end_time\x18\x03\x20\x01(\x03R\x07endTime\x12\x1d\n\nbegin\
    _time\x18\x05\x20\x01(\x03R\tbeginTime\x12\x17\n\x07up_info\x18\n\x20\
    \x03(\rR\x06upInfo\x12\x1a\n\x08featured\x18\x04\x20\x03(\rR\x08featured\
    \x12\x1f\n\x0bhistory_url\x18\x07\x20\x01(\tR\nhistoryUrl\x12\x1d\n\ndet\
    ail_url\x18\x02\x20\x01(\tR\tdetailUrl\x122\n\rgacha_ceiling\x18\x01\x20\
    \x01(\x0b2\r.GachaCeilingR\x0cgachaCeiling\x12\x19\n\x08gacha_id\x18\t\
    \x20\x01(\rR\x07gachaIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::GachaCeiling::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GachaInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
