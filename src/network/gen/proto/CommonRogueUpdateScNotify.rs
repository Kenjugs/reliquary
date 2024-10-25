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

//! Generated file from `CommonRogueUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CommonRogueUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CommonRogueUpdateScNotify {
    // message oneof groups
    pub IJMNDINHKGD: ::std::option::Option<common_rogue_update_sc_notify::IJMNDINHKGD>,
    // special fields
    // @@protoc_insertion_point(special_field:CommonRogueUpdateScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CommonRogueUpdateScNotify {
    fn default() -> &'a CommonRogueUpdateScNotify {
        <CommonRogueUpdateScNotify as ::protobuf::Message>::default_instance()
    }
}

impl CommonRogueUpdateScNotify {
    pub fn new() -> CommonRogueUpdateScNotify {
        ::std::default::Default::default()
    }

    // .LMPOJGEHFIP GMPHOLJJDCA = 14;

    pub fn GMPHOLJJDCA(&self) -> &super::LMPOJGEHFIP::LMPOJGEHFIP {
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(ref v)) => v,
            _ => <super::LMPOJGEHFIP::LMPOJGEHFIP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GMPHOLJJDCA(&mut self) {
        self.IJMNDINHKGD = ::std::option::Option::None;
    }

    pub fn has_GMPHOLJJDCA(&self) -> bool {
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GMPHOLJJDCA(&mut self, v: super::LMPOJGEHFIP::LMPOJGEHFIP) {
        self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GMPHOLJJDCA(&mut self) -> &mut super::LMPOJGEHFIP::LMPOJGEHFIP {
        if let ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(_)) = self.IJMNDINHKGD {
        } else {
            self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(super::LMPOJGEHFIP::LMPOJGEHFIP::new()));
        }
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GMPHOLJJDCA(&mut self) -> super::LMPOJGEHFIP::LMPOJGEHFIP {
        if self.has_GMPHOLJJDCA() {
            match self.IJMNDINHKGD.take() {
                ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LMPOJGEHFIP::LMPOJGEHFIP::new()
        }
    }

    // .KIANELPLCMF OJFJFEJMJPG = 8;

    pub fn OJFJFEJMJPG(&self) -> &super::KIANELPLCMF::KIANELPLCMF {
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(ref v)) => v,
            _ => <super::KIANELPLCMF::KIANELPLCMF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OJFJFEJMJPG(&mut self) {
        self.IJMNDINHKGD = ::std::option::Option::None;
    }

    pub fn has_OJFJFEJMJPG(&self) -> bool {
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OJFJFEJMJPG(&mut self, v: super::KIANELPLCMF::KIANELPLCMF) {
        self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OJFJFEJMJPG(&mut self) -> &mut super::KIANELPLCMF::KIANELPLCMF {
        if let ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(_)) = self.IJMNDINHKGD {
        } else {
            self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(super::KIANELPLCMF::KIANELPLCMF::new()));
        }
        match self.IJMNDINHKGD {
            ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OJFJFEJMJPG(&mut self) -> super::KIANELPLCMF::KIANELPLCMF {
        if self.has_OJFJFEJMJPG() {
            match self.IJMNDINHKGD.take() {
                ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KIANELPLCMF::KIANELPLCMF::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LMPOJGEHFIP::LMPOJGEHFIP>(
            "GMPHOLJJDCA",
            CommonRogueUpdateScNotify::has_GMPHOLJJDCA,
            CommonRogueUpdateScNotify::GMPHOLJJDCA,
            CommonRogueUpdateScNotify::mut_GMPHOLJJDCA,
            CommonRogueUpdateScNotify::set_GMPHOLJJDCA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KIANELPLCMF::KIANELPLCMF>(
            "OJFJFEJMJPG",
            CommonRogueUpdateScNotify::has_OJFJFEJMJPG,
            CommonRogueUpdateScNotify::OJFJFEJMJPG,
            CommonRogueUpdateScNotify::mut_OJFJFEJMJPG,
            CommonRogueUpdateScNotify::set_OJFJFEJMJPG,
        ));
        oneofs.push(common_rogue_update_sc_notify::IJMNDINHKGD::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CommonRogueUpdateScNotify>(
            "CommonRogueUpdateScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CommonRogueUpdateScNotify {
    const NAME: &'static str = "CommonRogueUpdateScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(is.read_message()?));
                },
                66 => {
                    self.IJMNDINHKGD = ::std::option::Option::Some(common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.IJMNDINHKGD {
            match v {
                &common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.IJMNDINHKGD {
            match v {
                &common_rogue_update_sc_notify::IJMNDINHKGD::GMPHOLJJDCA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &common_rogue_update_sc_notify::IJMNDINHKGD::OJFJFEJMJPG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
            };
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

    fn new() -> CommonRogueUpdateScNotify {
        CommonRogueUpdateScNotify::new()
    }

    fn clear(&mut self) {
        self.IJMNDINHKGD = ::std::option::Option::None;
        self.IJMNDINHKGD = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CommonRogueUpdateScNotify {
        static instance: CommonRogueUpdateScNotify = CommonRogueUpdateScNotify {
            IJMNDINHKGD: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CommonRogueUpdateScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CommonRogueUpdateScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CommonRogueUpdateScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonRogueUpdateScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CommonRogueUpdateScNotify`
pub mod common_rogue_update_sc_notify {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CommonRogueUpdateScNotify.IJMNDINHKGD)
    pub enum IJMNDINHKGD {
        // @@protoc_insertion_point(oneof_field:CommonRogueUpdateScNotify.GMPHOLJJDCA)
        GMPHOLJJDCA(super::super::LMPOJGEHFIP::LMPOJGEHFIP),
        // @@protoc_insertion_point(oneof_field:CommonRogueUpdateScNotify.OJFJFEJMJPG)
        OJFJFEJMJPG(super::super::KIANELPLCMF::KIANELPLCMF),
    }

    impl ::protobuf::Oneof for IJMNDINHKGD {
    }

    impl ::protobuf::OneofFull for IJMNDINHKGD {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CommonRogueUpdateScNotify as ::protobuf::MessageFull>::descriptor().oneof_by_name("IJMNDINHKGD").unwrap()).clone()
        }
    }

    impl IJMNDINHKGD {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<IJMNDINHKGD>("IJMNDINHKGD")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fCommonRogueUpdateScNotify.proto\x1a\x11KIANELPLCMF.proto\x1a\x11LM\
    POJGEHFIP.proto\"\x8e\x01\n\x19CommonRogueUpdateScNotify\x120\n\x0bGMPHO\
    LJJDCA\x18\x0e\x20\x01(\x0b2\x0c.LMPOJGEHFIPH\0R\x0bGMPHOLJJDCA\x120\n\
    \x0bOJFJFEJMJPG\x18\x08\x20\x01(\x0b2\x0c.KIANELPLCMFH\0R\x0bOJFJFEJMJPG\
    B\r\n\x0bIJMNDINHKGDb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::KIANELPLCMF::file_descriptor().clone());
            deps.push(super::LMPOJGEHFIP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CommonRogueUpdateScNotify::generated_message_descriptor_data());
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
