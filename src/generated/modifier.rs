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
//! Generated file from `modifier.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Modifier {
    // message oneof groups
    pub expression: ::std::option::Option<Modifier_oneof_expression>,
    pub lifetime: ::std::option::Option<Modifier_oneof_lifetime>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Modifier {
    fn default() -> &'a Modifier {
        <Modifier as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Modifier_oneof_expression {
    add(u32),
    subtract(u32),
    multiply(super::fraction::Fraction),
}

#[derive(Clone,PartialEq,Debug)]
pub enum Modifier_oneof_lifetime {
    lifetime_duration(u32),
}

impl Modifier {
    pub fn new() -> Modifier {
        ::std::default::Default::default()
    }

    // uint32 add = 1;


    pub fn get_add(&self) -> u32 {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::add(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_add(&mut self) {
        self.expression = ::std::option::Option::None;
    }

    pub fn has_add(&self) -> bool {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::add(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add(&mut self, v: u32) {
        self.expression = ::std::option::Option::Some(Modifier_oneof_expression::add(v))
    }

    // uint32 subtract = 2;


    pub fn get_subtract(&self) -> u32 {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::subtract(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_subtract(&mut self) {
        self.expression = ::std::option::Option::None;
    }

    pub fn has_subtract(&self) -> bool {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::subtract(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_subtract(&mut self, v: u32) {
        self.expression = ::std::option::Option::Some(Modifier_oneof_expression::subtract(v))
    }

    // .Fraction multiply = 3;


    pub fn get_multiply(&self) -> &super::fraction::Fraction {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::multiply(ref v)) => v,
            _ => <super::fraction::Fraction as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_multiply(&mut self) {
        self.expression = ::std::option::Option::None;
    }

    pub fn has_multiply(&self) -> bool {
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::multiply(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_multiply(&mut self, v: super::fraction::Fraction) {
        self.expression = ::std::option::Option::Some(Modifier_oneof_expression::multiply(v))
    }

    // Mutable pointer to the field.
    pub fn mut_multiply(&mut self) -> &mut super::fraction::Fraction {
        if let ::std::option::Option::Some(Modifier_oneof_expression::multiply(_)) = self.expression {
        } else {
            self.expression = ::std::option::Option::Some(Modifier_oneof_expression::multiply(super::fraction::Fraction::new()));
        }
        match self.expression {
            ::std::option::Option::Some(Modifier_oneof_expression::multiply(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_multiply(&mut self) -> super::fraction::Fraction {
        if self.has_multiply() {
            match self.expression.take() {
                ::std::option::Option::Some(Modifier_oneof_expression::multiply(v)) => v,
                _ => panic!(),
            }
        } else {
            super::fraction::Fraction::new()
        }
    }

    // uint32 lifetime_duration = 4;


    pub fn get_lifetime_duration(&self) -> u32 {
        match self.lifetime {
            ::std::option::Option::Some(Modifier_oneof_lifetime::lifetime_duration(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_lifetime_duration(&mut self) {
        self.lifetime = ::std::option::Option::None;
    }

    pub fn has_lifetime_duration(&self) -> bool {
        match self.lifetime {
            ::std::option::Option::Some(Modifier_oneof_lifetime::lifetime_duration(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_lifetime_duration(&mut self, v: u32) {
        self.lifetime = ::std::option::Option::Some(Modifier_oneof_lifetime::lifetime_duration(v))
    }
}

impl ::protobuf::Message for Modifier {
    fn is_initialized(&self) -> bool {
        if let Some(Modifier_oneof_expression::multiply(ref v)) = self.expression {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.expression = ::std::option::Option::Some(Modifier_oneof_expression::add(is.read_uint32()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.expression = ::std::option::Option::Some(Modifier_oneof_expression::subtract(is.read_uint32()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.expression = ::std::option::Option::Some(Modifier_oneof_expression::multiply(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.lifetime = ::std::option::Option::Some(Modifier_oneof_lifetime::lifetime_duration(is.read_uint32()?));
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
        if let ::std::option::Option::Some(ref v) = self.expression {
            match v {
                &Modifier_oneof_expression::add(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Modifier_oneof_expression::subtract(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Modifier_oneof_expression::multiply(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.lifetime {
            match v {
                &Modifier_oneof_lifetime::lifetime_duration(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.expression {
            match v {
                &Modifier_oneof_expression::add(v) => {
                    os.write_uint32(1, v)?;
                },
                &Modifier_oneof_expression::subtract(v) => {
                    os.write_uint32(2, v)?;
                },
                &Modifier_oneof_expression::multiply(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.lifetime {
            match v {
                &Modifier_oneof_lifetime::lifetime_duration(v) => {
                    os.write_uint32(4, v)?;
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

    fn new() -> Modifier {
        Modifier::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                "add",
                Modifier::has_add,
                Modifier::get_add,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                "subtract",
                Modifier::has_subtract,
                Modifier::get_subtract,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::fraction::Fraction>(
                "multiply",
                Modifier::has_multiply,
                Modifier::get_multiply,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                "lifetime_duration",
                Modifier::has_lifetime_duration,
                Modifier::get_lifetime_duration,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Modifier>(
                "Modifier",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Modifier {
        static instance: ::protobuf::rt::LazyV2<Modifier> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Modifier::new)
    }
}

impl ::protobuf::Clear for Modifier {
    fn clear(&mut self) {
        self.expression = ::std::option::Option::None;
        self.expression = ::std::option::Option::None;
        self.expression = ::std::option::Option::None;
        self.lifetime = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Modifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Modifier {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0emodifier.proto\x1a\x0efraction.proto\x1a\x1egoogle/protobuf/wrappe\
    rs.proto\"\xae\x01\n\x08Modifier\x12\x12\n\x03add\x18\x01\x20\x01(\rH\0R\
    \x03add\x12\x1c\n\x08subtract\x18\x02\x20\x01(\rH\0R\x08subtract\x12'\n\
    \x08multiply\x18\x03\x20\x01(\x0b2\t.FractionH\0R\x08multiply\x12-\n\x11\
    lifetime_duration\x18\x04\x20\x01(\rH\x01R\x10lifetimeDurationB\x0c\n\ne\
    xpressionB\n\n\x08lifetimeb\x06proto3\
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