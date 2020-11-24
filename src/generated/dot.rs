// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `dot.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct DOT {
    // message fields
    pub aspect: super::aspect::Aspect,
    pub damage_value: u32,
    // message oneof groups
    pub lifetime: ::std::option::Option<DOT_oneof_lifetime>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DOT {
    fn default() -> &'a DOT {
        <DOT as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum DOT_oneof_lifetime {
    lifetime_duration(u32),
}

impl DOT {
    pub fn new() -> DOT {
        ::std::default::Default::default()
    }

    // .Aspect aspect = 1;


    pub fn get_aspect(&self) -> super::aspect::Aspect {
        self.aspect
    }
    pub fn clear_aspect(&mut self) {
        self.aspect = super::aspect::Aspect::ASPECT_NONE;
    }

    // Param is passed by value, moved
    pub fn set_aspect(&mut self, v: super::aspect::Aspect) {
        self.aspect = v;
    }

    // uint32 damage_value = 2;


    pub fn get_damage_value(&self) -> u32 {
        self.damage_value
    }
    pub fn clear_damage_value(&mut self) {
        self.damage_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_damage_value(&mut self, v: u32) {
        self.damage_value = v;
    }

    // uint32 lifetime_duration = 3;


    pub fn get_lifetime_duration(&self) -> u32 {
        match self.lifetime {
            ::std::option::Option::Some(DOT_oneof_lifetime::lifetime_duration(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_lifetime_duration(&mut self) {
        self.lifetime = ::std::option::Option::None;
    }

    pub fn has_lifetime_duration(&self) -> bool {
        match self.lifetime {
            ::std::option::Option::Some(DOT_oneof_lifetime::lifetime_duration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_lifetime_duration(&mut self, v: u32) {
        self.lifetime = ::std::option::Option::Some(DOT_oneof_lifetime::lifetime_duration(v))
    }
}

impl ::protobuf::Message for DOT {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.aspect, 1, &mut self.unknown_fields)?
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.damage_value = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.lifetime = ::std::option::Option::Some(DOT_oneof_lifetime::lifetime_duration(is.read_uint32()?));
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
        if self.aspect != super::aspect::Aspect::ASPECT_NONE {
            my_size += ::protobuf::rt::enum_size(1, self.aspect);
        }
        if self.damage_value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.damage_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.lifetime {
            match v {
                &DOT_oneof_lifetime::lifetime_duration(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.aspect != super::aspect::Aspect::ASPECT_NONE {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.aspect))?;
        }
        if self.damage_value != 0 {
            os.write_uint32(2, self.damage_value)?;
        }
        if let ::std::option::Option::Some(ref v) = self.lifetime {
            match v {
                &DOT_oneof_lifetime::lifetime_duration(v) => {
                    os.write_uint32(3, v)?;
                },
            };
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

    fn new() -> DOT {
        DOT::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::aspect::Aspect>>(
                "aspect",
                |m: &DOT| { &m.aspect },
                |m: &mut DOT| { &mut m.aspect },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "damage_value",
                |m: &DOT| { &m.damage_value },
                |m: &mut DOT| { &mut m.damage_value },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                "lifetime_duration",
                DOT::has_lifetime_duration,
                DOT::get_lifetime_duration,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DOT>(
                "DOT",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DOT {
        static instance: ::protobuf::rt::LazyV2<DOT> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DOT::new)
    }
}

impl ::protobuf::Clear for DOT {
    fn clear(&mut self) {
        self.aspect = super::aspect::Aspect::ASPECT_NONE;
        self.damage_value = 0;
        self.lifetime = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DOT {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DOT {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tdot.proto\x1a\x0caspect.proto\"\x84\x01\n\x03DOT\x12\x1f\n\x06aspect\
    \x18\x01\x20\x01(\x0e2\x07.AspectR\x06aspect\x12!\n\x0cdamage_value\x18\
    \x02\x20\x01(\rR\x0bdamageValue\x12-\n\x11lifetime_duration\x18\x03\x20\
    \x01(\rH\0R\x10lifetimeDurationB\n\n\x08lifetimeb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
