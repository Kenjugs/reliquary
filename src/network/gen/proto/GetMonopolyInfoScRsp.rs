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

//! Generated file from `GetMonopolyInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetMonopolyInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetMonopolyInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.ICMBDLMGDBO)
    pub ICMBDLMGDBO: ::protobuf::MessageField<super::BMICCBDOCGG::BMICCBDOCGG>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.IBJOOCKCFHF)
    pub IBJOOCKCFHF: ::protobuf::MessageField<super::NHMKOHAACCC::NHMKOHAACCC>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.ABKOHABEMMF)
    pub ABKOHABEMMF: ::protobuf::MessageField<super::HHPGOCKNJCN::HHPGOCKNJCN>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.OCJONDAINGB)
    pub OCJONDAINGB: ::protobuf::MessageField<super::AAOIINGJMGB::AAOIINGJMGB>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.BCNOIFKDEFO)
    pub BCNOIFKDEFO: ::protobuf::MessageField<super::FIPPOBBBBJJ::FIPPOBBBBJJ>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.DMBOMNEILKG)
    pub DMBOMNEILKG: ::protobuf::MessageField<super::KCLICGHKKPH::KCLICGHKKPH>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.POJBLJNDABI)
    pub POJBLJNDABI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.PCOEFKDDBOE)
    pub PCOEFKDDBOE: ::protobuf::MessageField<super::OEEFPPBEAOF::OEEFPPBEAOF>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.OBBICMBHBMO)
    pub OBBICMBHBMO: ::protobuf::MessageField<super::GGFPEEMILPL::GGFPEEMILPL>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.ALAGJNMDPEN)
    pub ALAGJNMDPEN: ::protobuf::MessageField<super::BEJAIEGKJCN::BEJAIEGKJCN>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.KLNPKPIMKHJ)
    pub KLNPKPIMKHJ: ::protobuf::MessageField<super::AFFKECMGPEI::AFFKECMGPEI>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.DDFLFFFIPIC)
    pub DDFLFFFIPIC: ::protobuf::MessageField<super::PAPEPLFDCON::PAPEPLFDCON>,
    // special fields
    // @@protoc_insertion_point(special_field:GetMonopolyInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetMonopolyInfoScRsp {
    fn default() -> &'a GetMonopolyInfoScRsp {
        <GetMonopolyInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetMonopolyInfoScRsp {
    pub fn new() -> GetMonopolyInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BMICCBDOCGG::BMICCBDOCGG>(
            "ICMBDLMGDBO",
            |m: &GetMonopolyInfoScRsp| { &m.ICMBDLMGDBO },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.ICMBDLMGDBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NHMKOHAACCC::NHMKOHAACCC>(
            "IBJOOCKCFHF",
            |m: &GetMonopolyInfoScRsp| { &m.IBJOOCKCFHF },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.IBJOOCKCFHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HHPGOCKNJCN::HHPGOCKNJCN>(
            "ABKOHABEMMF",
            |m: &GetMonopolyInfoScRsp| { &m.ABKOHABEMMF },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.ABKOHABEMMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AAOIINGJMGB::AAOIINGJMGB>(
            "OCJONDAINGB",
            |m: &GetMonopolyInfoScRsp| { &m.OCJONDAINGB },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.OCJONDAINGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FIPPOBBBBJJ::FIPPOBBBBJJ>(
            "BCNOIFKDEFO",
            |m: &GetMonopolyInfoScRsp| { &m.BCNOIFKDEFO },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.BCNOIFKDEFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KCLICGHKKPH::KCLICGHKKPH>(
            "DMBOMNEILKG",
            |m: &GetMonopolyInfoScRsp| { &m.DMBOMNEILKG },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.DMBOMNEILKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetMonopolyInfoScRsp| { &m.retcode },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "POJBLJNDABI",
            |m: &GetMonopolyInfoScRsp| { &m.POJBLJNDABI },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.POJBLJNDABI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OEEFPPBEAOF::OEEFPPBEAOF>(
            "PCOEFKDDBOE",
            |m: &GetMonopolyInfoScRsp| { &m.PCOEFKDDBOE },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.PCOEFKDDBOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GGFPEEMILPL::GGFPEEMILPL>(
            "OBBICMBHBMO",
            |m: &GetMonopolyInfoScRsp| { &m.OBBICMBHBMO },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.OBBICMBHBMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BEJAIEGKJCN::BEJAIEGKJCN>(
            "ALAGJNMDPEN",
            |m: &GetMonopolyInfoScRsp| { &m.ALAGJNMDPEN },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.ALAGJNMDPEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AFFKECMGPEI::AFFKECMGPEI>(
            "KLNPKPIMKHJ",
            |m: &GetMonopolyInfoScRsp| { &m.KLNPKPIMKHJ },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.KLNPKPIMKHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PAPEPLFDCON::PAPEPLFDCON>(
            "DDFLFFFIPIC",
            |m: &GetMonopolyInfoScRsp| { &m.DDFLFFFIPIC },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.DDFLFFFIPIC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetMonopolyInfoScRsp>(
            "GetMonopolyInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetMonopolyInfoScRsp {
    const NAME: &'static str = "GetMonopolyInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ICMBDLMGDBO)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IBJOOCKCFHF)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ABKOHABEMMF)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OCJONDAINGB)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BCNOIFKDEFO)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DMBOMNEILKG)?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.POJBLJNDABI)?;
                },
                8 => {
                    self.POJBLJNDABI.push(is.read_uint32()?);
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PCOEFKDDBOE)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OBBICMBHBMO)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ALAGJNMDPEN)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KLNPKPIMKHJ)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DDFLFFFIPIC)?;
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
        if let Some(v) = self.ICMBDLMGDBO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IBJOOCKCFHF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ABKOHABEMMF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OCJONDAINGB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BCNOIFKDEFO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DMBOMNEILKG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        for value in &self.POJBLJNDABI {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if let Some(v) = self.PCOEFKDDBOE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OBBICMBHBMO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ALAGJNMDPEN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KLNPKPIMKHJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DDFLFFFIPIC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ICMBDLMGDBO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.IBJOOCKCFHF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.ABKOHABEMMF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.OCJONDAINGB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.BCNOIFKDEFO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.DMBOMNEILKG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        for v in &self.POJBLJNDABI {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.PCOEFKDDBOE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.OBBICMBHBMO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.ALAGJNMDPEN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.KLNPKPIMKHJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.DDFLFFFIPIC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> GetMonopolyInfoScRsp {
        GetMonopolyInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.ICMBDLMGDBO.clear();
        self.IBJOOCKCFHF.clear();
        self.ABKOHABEMMF.clear();
        self.OCJONDAINGB.clear();
        self.BCNOIFKDEFO.clear();
        self.DMBOMNEILKG.clear();
        self.retcode = 0;
        self.POJBLJNDABI.clear();
        self.PCOEFKDDBOE.clear();
        self.OBBICMBHBMO.clear();
        self.ALAGJNMDPEN.clear();
        self.KLNPKPIMKHJ.clear();
        self.DDFLFFFIPIC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetMonopolyInfoScRsp {
        static instance: GetMonopolyInfoScRsp = GetMonopolyInfoScRsp {
            ICMBDLMGDBO: ::protobuf::MessageField::none(),
            IBJOOCKCFHF: ::protobuf::MessageField::none(),
            ABKOHABEMMF: ::protobuf::MessageField::none(),
            OCJONDAINGB: ::protobuf::MessageField::none(),
            BCNOIFKDEFO: ::protobuf::MessageField::none(),
            DMBOMNEILKG: ::protobuf::MessageField::none(),
            retcode: 0,
            POJBLJNDABI: ::std::vec::Vec::new(),
            PCOEFKDDBOE: ::protobuf::MessageField::none(),
            OBBICMBHBMO: ::protobuf::MessageField::none(),
            ALAGJNMDPEN: ::protobuf::MessageField::none(),
            KLNPKPIMKHJ: ::protobuf::MessageField::none(),
            DDFLFFFIPIC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetMonopolyInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetMonopolyInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetMonopolyInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMonopolyInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetMonopolyInfoScRsp.proto\x1a\x11AAOIINGJMGB.proto\x1a\x11AFFKECM\
    GPEI.proto\x1a\x11BEJAIEGKJCN.proto\x1a\x11BMICCBDOCGG.proto\x1a\x11FIPP\
    OBBBBJJ.proto\x1a\x11GGFPEEMILPL.proto\x1a\x11HHPGOCKNJCN.proto\x1a\x11K\
    CLICGHKKPH.proto\x1a\x11NHMKOHAACCC.proto\x1a\x11OEEFPPBEAOF.proto\x1a\
    \x11PAPEPLFDCON.proto\"\xe2\x04\n\x14GetMonopolyInfoScRsp\x12.\n\x0bICMB\
    DLMGDBO\x18\x07\x20\x01(\x0b2\x0c.BMICCBDOCGGR\x0bICMBDLMGDBO\x12.\n\x0b\
    IBJOOCKCFHF\x18\n\x20\x01(\x0b2\x0c.NHMKOHAACCCR\x0bIBJOOCKCFHF\x12.\n\
    \x0bABKOHABEMMF\x18\x05\x20\x01(\x0b2\x0c.HHPGOCKNJCNR\x0bABKOHABEMMF\
    \x12.\n\x0bOCJONDAINGB\x18\x0c\x20\x01(\x0b2\x0c.AAOIINGJMGBR\x0bOCJONDA\
    INGB\x12.\n\x0bBCNOIFKDEFO\x18\x0e\x20\x01(\x0b2\x0c.FIPPOBBBBJJR\x0bBCN\
    OIFKDEFO\x12.\n\x0bDMBOMNEILKG\x18\x06\x20\x01(\x0b2\x0c.KCLICGHKKPHR\
    \x0bDMBOMNEILKG\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retcode\x12\
    \x20\n\x0bPOJBLJNDABI\x18\x01\x20\x03(\rR\x0bPOJBLJNDABI\x12.\n\x0bPCOEF\
    KDDBOE\x18\x08\x20\x01(\x0b2\x0c.OEEFPPBEAOFR\x0bPCOEFKDDBOE\x12.\n\x0bO\
    BBICMBHBMO\x18\r\x20\x01(\x0b2\x0c.GGFPEEMILPLR\x0bOBBICMBHBMO\x12.\n\
    \x0bALAGJNMDPEN\x18\x0b\x20\x01(\x0b2\x0c.BEJAIEGKJCNR\x0bALAGJNMDPEN\
    \x12.\n\x0bKLNPKPIMKHJ\x18\x02\x20\x01(\x0b2\x0c.AFFKECMGPEIR\x0bKLNPKPI\
    MKHJ\x12.\n\x0bDDFLFFFIPIC\x18\t\x20\x01(\x0b2\x0c.PAPEPLFDCONR\x0bDDFLF\
    FFIPICb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::AAOIINGJMGB::file_descriptor().clone());
            deps.push(super::AFFKECMGPEI::file_descriptor().clone());
            deps.push(super::BEJAIEGKJCN::file_descriptor().clone());
            deps.push(super::BMICCBDOCGG::file_descriptor().clone());
            deps.push(super::FIPPOBBBBJJ::file_descriptor().clone());
            deps.push(super::GGFPEEMILPL::file_descriptor().clone());
            deps.push(super::HHPGOCKNJCN::file_descriptor().clone());
            deps.push(super::KCLICGHKKPH::file_descriptor().clone());
            deps.push(super::NHMKOHAACCC::file_descriptor().clone());
            deps.push(super::OEEFPPBEAOF::file_descriptor().clone());
            deps.push(super::PAPEPLFDCON::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetMonopolyInfoScRsp::generated_message_descriptor_data());
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
