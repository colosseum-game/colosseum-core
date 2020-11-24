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
//! Generated file from `consumable.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Consumable {
    // message fields
    pub display_name: ::std::string::String,
    pub description: ::std::string::String,
    pub max_count: u32,
    pub effect: ::protobuf::SingularPtrField<super::effect::Effect>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Consumable {
    fn default() -> &'a Consumable {
        <Consumable as ::protobuf::Message>::default_instance()
    }
}

impl Consumable {
    pub fn new() -> Consumable {
        ::std::default::Default::default()
    }

    // string display_name = 1;


    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }

    // string description = 2;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    // uint32 max_count = 3;


    pub fn get_max_count(&self) -> u32 {
        self.max_count
    }
    pub fn clear_max_count(&mut self) {
        self.max_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_count(&mut self, v: u32) {
        self.max_count = v;
    }

    // .Effect effect = 4;


    pub fn get_effect(&self) -> &super::effect::Effect {
        self.effect.as_ref().unwrap_or_else(|| <super::effect::Effect as ::protobuf::Message>::default_instance())
    }
    pub fn clear_effect(&mut self) {
        self.effect.clear();
    }

    pub fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effect(&mut self, v: super::effect::Effect) {
        self.effect = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_effect(&mut self) -> &mut super::effect::Effect {
        if self.effect.is_none() {
            self.effect.set_default();
        }
        self.effect.as_mut().unwrap()
    }

    // Take field
    pub fn take_effect(&mut self) -> super::effect::Effect {
        self.effect.take().unwrap_or_else(|| super::effect::Effect::new())
    }
}

impl ::protobuf::Message for Consumable {
    fn is_initialized(&self) -> bool {
        for v in &self.effect {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_count = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.effect)?;
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
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.display_name);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.description);
        }
        if self.max_count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.max_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.effect.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.display_name.is_empty() {
            os.write_string(1, &self.display_name)?;
        }
        if !self.description.is_empty() {
            os.write_string(2, &self.description)?;
        }
        if self.max_count != 0 {
            os.write_uint32(3, self.max_count)?;
        }
        if let Some(ref v) = self.effect.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Consumable {
        Consumable::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "display_name",
                |m: &Consumable| { &m.display_name },
                |m: &mut Consumable| { &mut m.display_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &Consumable| { &m.description },
                |m: &mut Consumable| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "max_count",
                |m: &Consumable| { &m.max_count },
                |m: &mut Consumable| { &mut m.max_count },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::effect::Effect>>(
                "effect",
                |m: &Consumable| { &m.effect },
                |m: &mut Consumable| { &mut m.effect },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Consumable>(
                "Consumable",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Consumable {
        static instance: ::protobuf::rt::LazyV2<Consumable> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Consumable::new)
    }
}

impl ::protobuf::Clear for Consumable {
    fn clear(&mut self) {
        self.display_name.clear();
        self.description.clear();
        self.max_count = 0;
        self.effect.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Consumable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Consumable {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10consumable.proto\x1a\x0ceffect.proto\"\x8f\x01\n\nConsumable\x12!\
    \n\x0cdisplay_name\x18\x01\x20\x01(\tR\x0bdisplayName\x12\x20\n\x0bdescr\
    iption\x18\x02\x20\x01(\tR\x0bdescription\x12\x1b\n\tmax_count\x18\x03\
    \x20\x01(\rR\x08maxCount\x12\x1f\n\x06effect\x18\x04\x20\x01(\x0b2\x07.E\
    ffectR\x06effectb\x06proto3\
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
