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

//! Generated file from `GetAlleyInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetAlleyInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAlleyInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.CGKLMFHHPCC)
    pub CGKLMFHHPCC: ::protobuf::MessageField<super::BNPAEMJJCFG::BNPAEMJJCFG>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.NKNKOJKHDDF)
    pub NKNKOJKHDDF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.JAAMCLIBAIO)
    pub JAAMCLIBAIO: ::protobuf::MessageField<super::AEBKGEAGJCJ::AEBKGEAGJCJ>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.NMHDBEMILMA)
    pub NMHDBEMILMA: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.ABLBGHOJIBC)
    pub ABLBGHOJIBC: ::protobuf::MessageField<super::JCHNGGIIOCD::JCHNGGIIOCD>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.NLDEAJOMBIO)
    pub NLDEAJOMBIO: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.MMIDJEHMHMP)
    pub MMIDJEHMHMP: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.NEHCBGHEJHK)
    pub NEHCBGHEJHK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.OLDHNGGKABL)
    pub OLDHNGGKABL: ::std::vec::Vec<super::CKFJJBFEAPC::CKFJJBFEAPC>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.MMOMFLMMOII)
    pub MMOMFLMMOII: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetAlleyInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAlleyInfoScRsp {
    fn default() -> &'a GetAlleyInfoScRsp {
        <GetAlleyInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAlleyInfoScRsp {
    pub fn new() -> GetAlleyInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BNPAEMJJCFG::BNPAEMJJCFG>(
            "CGKLMFHHPCC",
            |m: &GetAlleyInfoScRsp| { &m.CGKLMFHHPCC },
            |m: &mut GetAlleyInfoScRsp| { &mut m.CGKLMFHHPCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NKNKOJKHDDF",
            |m: &GetAlleyInfoScRsp| { &m.NKNKOJKHDDF },
            |m: &mut GetAlleyInfoScRsp| { &mut m.NKNKOJKHDDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &GetAlleyInfoScRsp| { &m.level },
            |m: &mut GetAlleyInfoScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AEBKGEAGJCJ::AEBKGEAGJCJ>(
            "JAAMCLIBAIO",
            |m: &GetAlleyInfoScRsp| { &m.JAAMCLIBAIO },
            |m: &mut GetAlleyInfoScRsp| { &mut m.JAAMCLIBAIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMHDBEMILMA",
            |m: &GetAlleyInfoScRsp| { &m.NMHDBEMILMA },
            |m: &mut GetAlleyInfoScRsp| { &mut m.NMHDBEMILMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JCHNGGIIOCD::JCHNGGIIOCD>(
            "ABLBGHOJIBC",
            |m: &GetAlleyInfoScRsp| { &m.ABLBGHOJIBC },
            |m: &mut GetAlleyInfoScRsp| { &mut m.ABLBGHOJIBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "NLDEAJOMBIO",
            |m: &GetAlleyInfoScRsp| { &m.NLDEAJOMBIO },
            |m: &mut GetAlleyInfoScRsp| { &mut m.NLDEAJOMBIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMIDJEHMHMP",
            |m: &GetAlleyInfoScRsp| { &m.MMIDJEHMHMP },
            |m: &mut GetAlleyInfoScRsp| { &mut m.MMIDJEHMHMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NEHCBGHEJHK",
            |m: &GetAlleyInfoScRsp| { &m.NEHCBGHEJHK },
            |m: &mut GetAlleyInfoScRsp| { &mut m.NEHCBGHEJHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OLDHNGGKABL",
            |m: &GetAlleyInfoScRsp| { &m.OLDHNGGKABL },
            |m: &mut GetAlleyInfoScRsp| { &mut m.OLDHNGGKABL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MMOMFLMMOII",
            |m: &GetAlleyInfoScRsp| { &m.MMOMFLMMOII },
            |m: &mut GetAlleyInfoScRsp| { &mut m.MMOMFLMMOII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAlleyInfoScRsp| { &m.retcode },
            |m: &mut GetAlleyInfoScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAlleyInfoScRsp>(
            "GetAlleyInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAlleyInfoScRsp {
    const NAME: &'static str = "GetAlleyInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CGKLMFHHPCC)?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.NKNKOJKHDDF)?;
                },
                40 => {
                    self.NKNKOJKHDDF.push(is.read_uint32()?);
                },
                24 => {
                    self.level = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JAAMCLIBAIO)?;
                },
                48 => {
                    self.NMHDBEMILMA = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ABLBGHOJIBC)?;
                },
                98 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.NLDEAJOMBIO.insert(key, value);
                },
                80 => {
                    self.MMIDJEHMHMP = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.NEHCBGHEJHK)?;
                },
                32 => {
                    self.NEHCBGHEJHK.push(is.read_uint32()?);
                },
                74 => {
                    self.OLDHNGGKABL.push(is.read_message()?);
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.MMOMFLMMOII)?;
                },
                88 => {
                    self.MMOMFLMMOII.push(is.read_uint32()?);
                },
                64 => {
                    self.retcode = is.read_uint32()?;
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
        if let Some(v) = self.CGKLMFHHPCC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.NKNKOJKHDDF {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level);
        }
        if let Some(v) = self.JAAMCLIBAIO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NMHDBEMILMA != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NMHDBEMILMA);
        }
        if let Some(v) = self.ABLBGHOJIBC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for (k, v) in &self.NLDEAJOMBIO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.MMIDJEHMHMP != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.MMIDJEHMHMP);
        }
        for value in &self.NEHCBGHEJHK {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        for value in &self.OLDHNGGKABL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MMOMFLMMOII {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CGKLMFHHPCC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.NKNKOJKHDDF {
            os.write_uint32(5, *v)?;
        };
        if self.level != 0 {
            os.write_uint32(3, self.level)?;
        }
        if let Some(v) = self.JAAMCLIBAIO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.NMHDBEMILMA != 0 {
            os.write_uint32(6, self.NMHDBEMILMA)?;
        }
        if let Some(v) = self.ABLBGHOJIBC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        for (k, v) in &self.NLDEAJOMBIO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(98)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.MMIDJEHMHMP != 0 {
            os.write_uint32(10, self.MMIDJEHMHMP)?;
        }
        for v in &self.NEHCBGHEJHK {
            os.write_uint32(4, *v)?;
        };
        for v in &self.OLDHNGGKABL {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.MMOMFLMMOII {
            os.write_uint32(11, *v)?;
        };
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
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

    fn new() -> GetAlleyInfoScRsp {
        GetAlleyInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.CGKLMFHHPCC.clear();
        self.NKNKOJKHDDF.clear();
        self.level = 0;
        self.JAAMCLIBAIO.clear();
        self.NMHDBEMILMA = 0;
        self.ABLBGHOJIBC.clear();
        self.NLDEAJOMBIO.clear();
        self.MMIDJEHMHMP = 0;
        self.NEHCBGHEJHK.clear();
        self.OLDHNGGKABL.clear();
        self.MMOMFLMMOII.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAlleyInfoScRsp {
        static instance: ::protobuf::rt::Lazy<GetAlleyInfoScRsp> = ::protobuf::rt::Lazy::new();
        instance.get(GetAlleyInfoScRsp::new)
    }
}

impl ::protobuf::MessageFull for GetAlleyInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAlleyInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAlleyInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAlleyInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetAlleyInfoScRsp.proto\x1a\x11AEBKGEAGJCJ.proto\x1a\x11BNPAEMJJCF\
    G.proto\x1a\x11CKFJJBFEAPC.proto\x1a\x11JCHNGGIIOCD.proto\"\xb4\x04\n\
    \x11GetAlleyInfoScRsp\x12.\n\x0bCGKLMFHHPCC\x18\x0f\x20\x01(\x0b2\x0c.BN\
    PAEMJJCFGR\x0bCGKLMFHHPCC\x12\x20\n\x0bNKNKOJKHDDF\x18\x05\x20\x03(\rR\
    \x0bNKNKOJKHDDF\x12\x14\n\x05level\x18\x03\x20\x01(\rR\x05level\x12.\n\
    \x0bJAAMCLIBAIO\x18\r\x20\x01(\x0b2\x0c.AEBKGEAGJCJR\x0bJAAMCLIBAIO\x12\
    \x20\n\x0bNMHDBEMILMA\x18\x06\x20\x01(\rR\x0bNMHDBEMILMA\x12.\n\x0bABLBG\
    HOJIBC\x18\x0e\x20\x01(\x0b2\x0c.JCHNGGIIOCDR\x0bABLBGHOJIBC\x12E\n\x0bN\
    LDEAJOMBIO\x18\x0c\x20\x03(\x0b2#.GetAlleyInfoScRsp.NLDEAJOMBIOEntryR\
    \x0bNLDEAJOMBIO\x12\x20\n\x0bMMIDJEHMHMP\x18\n\x20\x01(\rR\x0bMMIDJEHMHM\
    P\x12\x20\n\x0bNEHCBGHEJHK\x18\x04\x20\x03(\rR\x0bNEHCBGHEJHK\x12.\n\x0b\
    OLDHNGGKABL\x18\t\x20\x03(\x0b2\x0c.CKFJJBFEAPCR\x0bOLDHNGGKABL\x12\x20\
    \n\x0bMMOMFLMMOII\x18\x0b\x20\x03(\rR\x0bMMOMFLMMOII\x12\x18\n\x07retcod\
    e\x18\x08\x20\x01(\rR\x07retcode\x1a>\n\x10NLDEAJOMBIOEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\r\
    R\x05value:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::AEBKGEAGJCJ::file_descriptor().clone());
            deps.push(super::BNPAEMJJCFG::file_descriptor().clone());
            deps.push(super::CKFJJBFEAPC::file_descriptor().clone());
            deps.push(super::JCHNGGIIOCD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetAlleyInfoScRsp::generated_message_descriptor_data());
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