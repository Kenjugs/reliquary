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

//! Generated file from `FPOGBIOILFF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FPOGBIOILFF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FPOGBIOILFF {
    // message fields
    // @@protoc_insertion_point(field:FPOGBIOILFF.PMDADNCKJIA)
    pub PMDADNCKJIA: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.CNIJGAPOPAH)
    pub CNIJGAPOPAH: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.DGFNOGJFILI)
    pub DGFNOGJFILI: ::std::vec::Vec<super::OPMPNHEDCHB::OPMPNHEDCHB>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.PHHANACJEGG)
    pub PHHANACJEGG: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.BKCBHFNPNDB)
    pub BKCBHFNPNDB: ::std::vec::Vec<super::GGCPLONEAEG::GGCPLONEAEG>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::APFBCEPJLDC::APFBCEPJLDC>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.OJJLBIPFMAP)
    pub OJJLBIPFMAP: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.GABPEMANANE)
    pub GABPEMANANE: ::std::vec::Vec<super::GKHIKGFLEHO::GKHIKGFLEHO>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.NONCAHOJMIL)
    pub NONCAHOJMIL: bool,
    // @@protoc_insertion_point(field:FPOGBIOILFF.GDMHDNHIBIF)
    pub GDMHDNHIBIF: ::std::vec::Vec<super::TreasureDungeonRecordData::TreasureDungeonRecordData>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.ANMGHMCILLM)
    pub ANMGHMCILLM: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.NHLIFOPDKGM)
    pub NHLIFOPDKGM: ::std::vec::Vec<super::CJLIFBJOOAJ::CJLIFBJOOAJ>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.DHLPHKJBMHB)
    pub DHLPHKJBMHB: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.FDIPCGICJKF)
    pub FDIPCGICJKF: u32,
    // @@protoc_insertion_point(field:FPOGBIOILFF.KHGLEILELJP)
    pub KHGLEILELJP: bool,
    // @@protoc_insertion_point(field:FPOGBIOILFF.IOOLNIOPGBJ)
    pub IOOLNIOPGBJ: bool,
    // @@protoc_insertion_point(field:FPOGBIOILFF.KCAOCAEKGMK)
    pub KCAOCAEKGMK: ::std::vec::Vec<super::APFBCEPJLDC::APFBCEPJLDC>,
    // @@protoc_insertion_point(field:FPOGBIOILFF.NCFAJPAMBGD)
    pub NCFAJPAMBGD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FPOGBIOILFF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FPOGBIOILFF {
    fn default() -> &'a FPOGBIOILFF {
        <FPOGBIOILFF as ::protobuf::Message>::default_instance()
    }
}

impl FPOGBIOILFF {
    pub fn new() -> FPOGBIOILFF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMDADNCKJIA",
            |m: &FPOGBIOILFF| { &m.PMDADNCKJIA },
            |m: &mut FPOGBIOILFF| { &mut m.PMDADNCKJIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNIJGAPOPAH",
            |m: &FPOGBIOILFF| { &m.CNIJGAPOPAH },
            |m: &mut FPOGBIOILFF| { &mut m.CNIJGAPOPAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DGFNOGJFILI",
            |m: &FPOGBIOILFF| { &m.DGFNOGJFILI },
            |m: &mut FPOGBIOILFF| { &mut m.DGFNOGJFILI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHHANACJEGG",
            |m: &FPOGBIOILFF| { &m.PHHANACJEGG },
            |m: &mut FPOGBIOILFF| { &mut m.PHHANACJEGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BKCBHFNPNDB",
            |m: &FPOGBIOILFF| { &m.BKCBHFNPNDB },
            |m: &mut FPOGBIOILFF| { &mut m.BKCBHFNPNDB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &FPOGBIOILFF| { &m.avatar_list },
            |m: &mut FPOGBIOILFF| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJJLBIPFMAP",
            |m: &FPOGBIOILFF| { &m.OJJLBIPFMAP },
            |m: &mut FPOGBIOILFF| { &mut m.OJJLBIPFMAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GABPEMANANE",
            |m: &FPOGBIOILFF| { &m.GABPEMANANE },
            |m: &mut FPOGBIOILFF| { &mut m.GABPEMANANE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NONCAHOJMIL",
            |m: &FPOGBIOILFF| { &m.NONCAHOJMIL },
            |m: &mut FPOGBIOILFF| { &mut m.NONCAHOJMIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GDMHDNHIBIF",
            |m: &FPOGBIOILFF| { &m.GDMHDNHIBIF },
            |m: &mut FPOGBIOILFF| { &mut m.GDMHDNHIBIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANMGHMCILLM",
            |m: &FPOGBIOILFF| { &m.ANMGHMCILLM },
            |m: &mut FPOGBIOILFF| { &mut m.ANMGHMCILLM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NHLIFOPDKGM",
            |m: &FPOGBIOILFF| { &m.NHLIFOPDKGM },
            |m: &mut FPOGBIOILFF| { &mut m.NHLIFOPDKGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHLPHKJBMHB",
            |m: &FPOGBIOILFF| { &m.DHLPHKJBMHB },
            |m: &mut FPOGBIOILFF| { &mut m.DHLPHKJBMHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDIPCGICJKF",
            |m: &FPOGBIOILFF| { &m.FDIPCGICJKF },
            |m: &mut FPOGBIOILFF| { &mut m.FDIPCGICJKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHGLEILELJP",
            |m: &FPOGBIOILFF| { &m.KHGLEILELJP },
            |m: &mut FPOGBIOILFF| { &mut m.KHGLEILELJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOOLNIOPGBJ",
            |m: &FPOGBIOILFF| { &m.IOOLNIOPGBJ },
            |m: &mut FPOGBIOILFF| { &mut m.IOOLNIOPGBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KCAOCAEKGMK",
            |m: &FPOGBIOILFF| { &m.KCAOCAEKGMK },
            |m: &mut FPOGBIOILFF| { &mut m.KCAOCAEKGMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCFAJPAMBGD",
            |m: &FPOGBIOILFF| { &m.NCFAJPAMBGD },
            |m: &mut FPOGBIOILFF| { &mut m.NCFAJPAMBGD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FPOGBIOILFF>(
            "FPOGBIOILFF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FPOGBIOILFF {
    const NAME: &'static str = "FPOGBIOILFF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                7296 => {
                    self.PMDADNCKJIA = is.read_uint32()?;
                },
                72 => {
                    self.CNIJGAPOPAH = is.read_uint32()?;
                },
                7706 => {
                    self.DGFNOGJFILI.push(is.read_message()?);
                },
                112 => {
                    self.PHHANACJEGG = is.read_uint32()?;
                },
                13058 => {
                    self.BKCBHFNPNDB.push(is.read_message()?);
                },
                14450 => {
                    self.avatar_list.push(is.read_message()?);
                },
                104 => {
                    self.OJJLBIPFMAP = is.read_uint32()?;
                },
                12378 => {
                    self.GABPEMANANE.push(is.read_message()?);
                },
                12176 => {
                    self.NONCAHOJMIL = is.read_bool()?;
                },
                26 => {
                    self.GDMHDNHIBIF.push(is.read_message()?);
                },
                80 => {
                    self.ANMGHMCILLM = is.read_uint32()?;
                },
                10 => {
                    self.NHLIFOPDKGM.push(is.read_message()?);
                },
                48 => {
                    self.DHLPHKJBMHB = is.read_uint32()?;
                },
                16 => {
                    self.FDIPCGICJKF = is.read_uint32()?;
                },
                10568 => {
                    self.KHGLEILELJP = is.read_bool()?;
                },
                1504 => {
                    self.IOOLNIOPGBJ = is.read_bool()?;
                },
                3930 => {
                    self.KCAOCAEKGMK.push(is.read_message()?);
                },
                120 => {
                    self.NCFAJPAMBGD = is.read_uint32()?;
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
        if self.PMDADNCKJIA != 0 {
            my_size += ::protobuf::rt::uint32_size(912, self.PMDADNCKJIA);
        }
        if self.CNIJGAPOPAH != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.CNIJGAPOPAH);
        }
        for value in &self.DGFNOGJFILI {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PHHANACJEGG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PHHANACJEGG);
        }
        for value in &self.BKCBHFNPNDB {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.OJJLBIPFMAP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OJJLBIPFMAP);
        }
        for value in &self.GABPEMANANE {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NONCAHOJMIL != false {
            my_size += 2 + 1;
        }
        for value in &self.GDMHDNHIBIF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ANMGHMCILLM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.ANMGHMCILLM);
        }
        for value in &self.NHLIFOPDKGM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.DHLPHKJBMHB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.DHLPHKJBMHB);
        }
        if self.FDIPCGICJKF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FDIPCGICJKF);
        }
        if self.KHGLEILELJP != false {
            my_size += 2 + 1;
        }
        if self.IOOLNIOPGBJ != false {
            my_size += 2 + 1;
        }
        for value in &self.KCAOCAEKGMK {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NCFAJPAMBGD != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NCFAJPAMBGD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PMDADNCKJIA != 0 {
            os.write_uint32(912, self.PMDADNCKJIA)?;
        }
        if self.CNIJGAPOPAH != 0 {
            os.write_uint32(9, self.CNIJGAPOPAH)?;
        }
        for v in &self.DGFNOGJFILI {
            ::protobuf::rt::write_message_field_with_cached_size(963, v, os)?;
        };
        if self.PHHANACJEGG != 0 {
            os.write_uint32(14, self.PHHANACJEGG)?;
        }
        for v in &self.BKCBHFNPNDB {
            ::protobuf::rt::write_message_field_with_cached_size(1632, v, os)?;
        };
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(1806, v, os)?;
        };
        if self.OJJLBIPFMAP != 0 {
            os.write_uint32(13, self.OJJLBIPFMAP)?;
        }
        for v in &self.GABPEMANANE {
            ::protobuf::rt::write_message_field_with_cached_size(1547, v, os)?;
        };
        if self.NONCAHOJMIL != false {
            os.write_bool(1522, self.NONCAHOJMIL)?;
        }
        for v in &self.GDMHDNHIBIF {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.ANMGHMCILLM != 0 {
            os.write_uint32(10, self.ANMGHMCILLM)?;
        }
        for v in &self.NHLIFOPDKGM {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.DHLPHKJBMHB != 0 {
            os.write_uint32(6, self.DHLPHKJBMHB)?;
        }
        if self.FDIPCGICJKF != 0 {
            os.write_uint32(2, self.FDIPCGICJKF)?;
        }
        if self.KHGLEILELJP != false {
            os.write_bool(1321, self.KHGLEILELJP)?;
        }
        if self.IOOLNIOPGBJ != false {
            os.write_bool(188, self.IOOLNIOPGBJ)?;
        }
        for v in &self.KCAOCAEKGMK {
            ::protobuf::rt::write_message_field_with_cached_size(491, v, os)?;
        };
        if self.NCFAJPAMBGD != 0 {
            os.write_uint32(15, self.NCFAJPAMBGD)?;
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

    fn new() -> FPOGBIOILFF {
        FPOGBIOILFF::new()
    }

    fn clear(&mut self) {
        self.PMDADNCKJIA = 0;
        self.CNIJGAPOPAH = 0;
        self.DGFNOGJFILI.clear();
        self.PHHANACJEGG = 0;
        self.BKCBHFNPNDB.clear();
        self.avatar_list.clear();
        self.OJJLBIPFMAP = 0;
        self.GABPEMANANE.clear();
        self.NONCAHOJMIL = false;
        self.GDMHDNHIBIF.clear();
        self.ANMGHMCILLM = 0;
        self.NHLIFOPDKGM.clear();
        self.DHLPHKJBMHB = 0;
        self.FDIPCGICJKF = 0;
        self.KHGLEILELJP = false;
        self.IOOLNIOPGBJ = false;
        self.KCAOCAEKGMK.clear();
        self.NCFAJPAMBGD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FPOGBIOILFF {
        static instance: FPOGBIOILFF = FPOGBIOILFF {
            PMDADNCKJIA: 0,
            CNIJGAPOPAH: 0,
            DGFNOGJFILI: ::std::vec::Vec::new(),
            PHHANACJEGG: 0,
            BKCBHFNPNDB: ::std::vec::Vec::new(),
            avatar_list: ::std::vec::Vec::new(),
            OJJLBIPFMAP: 0,
            GABPEMANANE: ::std::vec::Vec::new(),
            NONCAHOJMIL: false,
            GDMHDNHIBIF: ::std::vec::Vec::new(),
            ANMGHMCILLM: 0,
            NHLIFOPDKGM: ::std::vec::Vec::new(),
            DHLPHKJBMHB: 0,
            FDIPCGICJKF: 0,
            KHGLEILELJP: false,
            IOOLNIOPGBJ: false,
            KCAOCAEKGMK: ::std::vec::Vec::new(),
            NCFAJPAMBGD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FPOGBIOILFF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FPOGBIOILFF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FPOGBIOILFF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FPOGBIOILFF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FPOGBIOILFF.proto\x1a\x11APFBCEPJLDC.proto\x1a\x11CJLIFBJOOAJ.prot\
    o\x1a\x11GGCPLONEAEG.proto\x1a\x11GKHIKGFLEHO.proto\x1a\x11OPMPNHEDCHB.p\
    roto\x1a\x1fTreasureDungeonRecordData.proto\"\xe9\x05\n\x0bFPOGBIOILFF\
    \x12!\n\x0bPMDADNCKJIA\x18\x90\x07\x20\x01(\rR\x0bPMDADNCKJIA\x12\x20\n\
    \x0bCNIJGAPOPAH\x18\t\x20\x01(\rR\x0bCNIJGAPOPAH\x12/\n\x0bDGFNOGJFILI\
    \x18\xc3\x07\x20\x03(\x0b2\x0c.OPMPNHEDCHBR\x0bDGFNOGJFILI\x12\x20\n\x0b\
    PHHANACJEGG\x18\x0e\x20\x01(\rR\x0bPHHANACJEGG\x12/\n\x0bBKCBHFNPNDB\x18\
    \xe0\x0c\x20\x03(\x0b2\x0c.GGCPLONEAEGR\x0bBKCBHFNPNDB\x12.\n\x0bavatar_\
    list\x18\x8e\x0e\x20\x03(\x0b2\x0c.APFBCEPJLDCR\navatarList\x12\x20\n\
    \x0bOJJLBIPFMAP\x18\r\x20\x01(\rR\x0bOJJLBIPFMAP\x12/\n\x0bGABPEMANANE\
    \x18\x8b\x0c\x20\x03(\x0b2\x0c.GKHIKGFLEHOR\x0bGABPEMANANE\x12!\n\x0bNON\
    CAHOJMIL\x18\xf2\x0b\x20\x01(\x08R\x0bNONCAHOJMIL\x12<\n\x0bGDMHDNHIBIF\
    \x18\x03\x20\x03(\x0b2\x1a.TreasureDungeonRecordDataR\x0bGDMHDNHIBIF\x12\
    \x20\n\x0bANMGHMCILLM\x18\n\x20\x01(\rR\x0bANMGHMCILLM\x12.\n\x0bNHLIFOP\
    DKGM\x18\x01\x20\x03(\x0b2\x0c.CJLIFBJOOAJR\x0bNHLIFOPDKGM\x12\x20\n\x0b\
    DHLPHKJBMHB\x18\x06\x20\x01(\rR\x0bDHLPHKJBMHB\x12\x20\n\x0bFDIPCGICJKF\
    \x18\x02\x20\x01(\rR\x0bFDIPCGICJKF\x12!\n\x0bKHGLEILELJP\x18\xa9\n\x20\
    \x01(\x08R\x0bKHGLEILELJP\x12!\n\x0bIOOLNIOPGBJ\x18\xbc\x01\x20\x01(\x08\
    R\x0bIOOLNIOPGBJ\x12/\n\x0bKCAOCAEKGMK\x18\xeb\x03\x20\x03(\x0b2\x0c.APF\
    BCEPJLDCR\x0bKCAOCAEKGMK\x12\x20\n\x0bNCFAJPAMBGD\x18\x0f\x20\x01(\rR\
    \x0bNCFAJPAMBGDb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::APFBCEPJLDC::file_descriptor().clone());
            deps.push(super::CJLIFBJOOAJ::file_descriptor().clone());
            deps.push(super::GGCPLONEAEG::file_descriptor().clone());
            deps.push(super::GKHIKGFLEHO::file_descriptor().clone());
            deps.push(super::OPMPNHEDCHB::file_descriptor().clone());
            deps.push(super::TreasureDungeonRecordData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FPOGBIOILFF::generated_message_descriptor_data());
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
