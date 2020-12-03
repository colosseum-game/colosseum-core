// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc 3.14.0
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `combat_event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_PRE;

#[derive(PartialEq,Clone,Default)]
pub struct AttackEvent {
    // message fields
    pub targets: ::std::vec::Vec<super::target::Target>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a AttackEvent {
    fn default() -> &'a AttackEvent {
        <AttackEvent as ::protobuf::Message>::default_instance()
    }
}

impl AttackEvent {
    pub fn new() -> AttackEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "targets",
            |m: &AttackEvent| { &m.targets },
            |m: &mut AttackEvent| { &mut m.targets },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AttackEvent>(
            "AttackEvent",
            0,
            fields,
        )
    }
}

impl ::protobuf::Message for AttackEvent {
    fn is_initialized(&self) -> bool {
        for v in &self.targets {
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
                    ::protobuf::rt::read_repeated_message_into_vec(wire_type, is, &mut self.targets)?;
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
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.targets {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
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

    fn new() -> AttackEvent {
        AttackEvent::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }

    fn default_instance() -> &'static AttackEvent {
        static instance: AttackEvent = AttackEvent {
            targets: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for AttackEvent {
    fn clear(&mut self) {
        self.targets.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AttackEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AttackEvent {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default)]
pub struct SkillEvent {
    // message fields
    pub skill: ::std::string::String,
    pub targets: ::std::vec::Vec<super::target::Target>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a SkillEvent {
    fn default() -> &'a SkillEvent {
        <SkillEvent as ::protobuf::Message>::default_instance()
    }
}

impl SkillEvent {
    pub fn new() -> SkillEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill",
            |m: &SkillEvent| { &m.skill },
            |m: &mut SkillEvent| { &mut m.skill },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "targets",
            |m: &SkillEvent| { &m.targets },
            |m: &mut SkillEvent| { &mut m.targets },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SkillEvent>(
            "SkillEvent",
            1,
            fields,
        )
    }
}

impl ::protobuf::Message for SkillEvent {
    fn is_initialized(&self) -> bool {
        for v in &self.targets {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.skill = is.read_string()?;
                },
                1 => {
                    ::protobuf::rt::read_repeated_message_into_vec(wire_type, is, &mut self.targets)?;
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
        if !self.skill.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.skill);
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.skill.is_empty() {
            os.write_string(2, &self.skill)?;
        }
        for v in &self.targets {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
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

    fn new() -> SkillEvent {
        SkillEvent::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 1)
    }

    fn default_instance() -> &'static SkillEvent {
        static instance: SkillEvent = SkillEvent {
            skill: ::std::string::String::new(),
            targets: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for SkillEvent {
    fn clear(&mut self) {
        self.skill.clear();
        self.targets.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SkillEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SkillEvent {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsumableEvent {
    // message fields
    pub consumable: ::std::string::String,
    pub targets: ::std::vec::Vec<super::target::Target>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a ConsumableEvent {
    fn default() -> &'a ConsumableEvent {
        <ConsumableEvent as ::protobuf::Message>::default_instance()
    }
}

impl ConsumableEvent {
    pub fn new() -> ConsumableEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "consumable",
            |m: &ConsumableEvent| { &m.consumable },
            |m: &mut ConsumableEvent| { &mut m.consumable },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "targets",
            |m: &ConsumableEvent| { &m.targets },
            |m: &mut ConsumableEvent| { &mut m.targets },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ConsumableEvent>(
            "ConsumableEvent",
            2,
            fields,
        )
    }
}

impl ::protobuf::Message for ConsumableEvent {
    fn is_initialized(&self) -> bool {
        for v in &self.targets {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.consumable = is.read_string()?;
                },
                1 => {
                    ::protobuf::rt::read_repeated_message_into_vec(wire_type, is, &mut self.targets)?;
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
        if !self.consumable.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.consumable);
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.consumable.is_empty() {
            os.write_string(2, &self.consumable)?;
        }
        for v in &self.targets {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
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

    fn new() -> ConsumableEvent {
        ConsumableEvent::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 2)
    }

    fn default_instance() -> &'static ConsumableEvent {
        static instance: ConsumableEvent = ConsumableEvent {
            consumable: ::std::string::String::new(),
            targets: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for ConsumableEvent {
    fn clear(&mut self) {
        self.consumable.clear();
        self.targets.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsumableEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsumableEvent {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default)]
pub struct CombatEvent {
    // message oneof groups
    pub event: ::std::option::Option<combat_event::Event>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a CombatEvent {
    fn default() -> &'a CombatEvent {
        <CombatEvent as ::protobuf::Message>::default_instance()
    }
}

impl CombatEvent {
    pub fn new() -> CombatEvent {
        ::std::default::Default::default()
    }

    // .AttackEvent attack_event = 1;

    pub fn get_attack_event(&self) -> &AttackEvent {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::attack_event(ref v)) => v,
            _ => <AttackEvent as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_attack_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_attack_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::attack_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_attack_event(&mut self, v: AttackEvent) {
        self.event = ::std::option::Option::Some(combat_event::Event::attack_event(v))
    }

    // Mutable pointer to the field.
    pub fn mut_attack_event(&mut self) -> &mut AttackEvent {
        if let ::std::option::Option::Some(combat_event::Event::attack_event(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(combat_event::Event::attack_event(AttackEvent::new()));
        }
        match self.event {
            ::std::option::Option::Some(combat_event::Event::attack_event(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_attack_event(&mut self) -> AttackEvent {
        if self.has_attack_event() {
            match self.event.take() {
                ::std::option::Option::Some(combat_event::Event::attack_event(v)) => v,
                _ => panic!(),
            }
        } else {
            AttackEvent::new()
        }
    }

    // .ConsumableEvent consumable_event = 2;

    pub fn get_consumable_event(&self) -> &ConsumableEvent {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::consumable_event(ref v)) => v,
            _ => <ConsumableEvent as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_consumable_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_consumable_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::consumable_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_consumable_event(&mut self, v: ConsumableEvent) {
        self.event = ::std::option::Option::Some(combat_event::Event::consumable_event(v))
    }

    // Mutable pointer to the field.
    pub fn mut_consumable_event(&mut self) -> &mut ConsumableEvent {
        if let ::std::option::Option::Some(combat_event::Event::consumable_event(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(combat_event::Event::consumable_event(ConsumableEvent::new()));
        }
        match self.event {
            ::std::option::Option::Some(combat_event::Event::consumable_event(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_consumable_event(&mut self) -> ConsumableEvent {
        if self.has_consumable_event() {
            match self.event.take() {
                ::std::option::Option::Some(combat_event::Event::consumable_event(v)) => v,
                _ => panic!(),
            }
        } else {
            ConsumableEvent::new()
        }
    }

    // string equipable_event = 3;

    pub fn get_equipable_event(&self) -> &str {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::equipable_event(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_equipable_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_equipable_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::equipable_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_equipable_event(&mut self, v: ::std::string::String) {
        self.event = ::std::option::Option::Some(combat_event::Event::equipable_event(v))
    }

    // Mutable pointer to the field.
    pub fn mut_equipable_event(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(combat_event::Event::equipable_event(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(combat_event::Event::equipable_event(::std::string::String::new()));
        }
        match self.event {
            ::std::option::Option::Some(combat_event::Event::equipable_event(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_equipable_event(&mut self) -> ::std::string::String {
        if self.has_equipable_event() {
            match self.event.take() {
                ::std::option::Option::Some(combat_event::Event::equipable_event(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bool forfeit_event = 4;

    pub fn get_forfeit_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::forfeit_event(v)) => v,
            _ => false,
        }
    }

    pub fn clear_forfeit_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_forfeit_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::forfeit_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_forfeit_event(&mut self, v: bool) {
        self.event = ::std::option::Option::Some(combat_event::Event::forfeit_event(v))
    }

    // bool skip_event = 5;

    pub fn get_skip_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::skip_event(v)) => v,
            _ => false,
        }
    }

    pub fn clear_skip_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_skip_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::skip_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_skip_event(&mut self, v: bool) {
        self.event = ::std::option::Option::Some(combat_event::Event::skip_event(v))
    }

    // .SkillEvent skill_event = 6;

    pub fn get_skill_event(&self) -> &SkillEvent {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::skill_event(ref v)) => v,
            _ => <SkillEvent as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_skill_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_skill_event(&self) -> bool {
        match self.event {
            ::std::option::Option::Some(combat_event::Event::skill_event(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_skill_event(&mut self, v: SkillEvent) {
        self.event = ::std::option::Option::Some(combat_event::Event::skill_event(v))
    }

    // Mutable pointer to the field.
    pub fn mut_skill_event(&mut self) -> &mut SkillEvent {
        if let ::std::option::Option::Some(combat_event::Event::skill_event(_)) = self.event {
        } else {
            self.event = ::std::option::Option::Some(combat_event::Event::skill_event(SkillEvent::new()));
        }
        match self.event {
            ::std::option::Option::Some(combat_event::Event::skill_event(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_skill_event(&mut self) -> SkillEvent {
        if self.has_skill_event() {
            match self.event.take() {
                ::std::option::Option::Some(combat_event::Event::skill_event(v)) => v,
                _ => panic!(),
            }
        } else {
            SkillEvent::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, AttackEvent>(
            "attack_event",
            CombatEvent::has_attack_event,
            CombatEvent::get_attack_event,
            CombatEvent::mut_attack_event,
            CombatEvent::set_attack_event,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ConsumableEvent>(
            "consumable_event",
            CombatEvent::has_consumable_event,
            CombatEvent::get_consumable_event,
            CombatEvent::mut_consumable_event,
            CombatEvent::set_consumable_event,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "equipable_event",
            CombatEvent::has_equipable_event,
            CombatEvent::get_equipable_event,
            CombatEvent::set_equipable_event,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "forfeit_event",
            CombatEvent::has_forfeit_event,
            CombatEvent::get_forfeit_event,
            CombatEvent::set_forfeit_event,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "skip_event",
            CombatEvent::has_skip_event,
            CombatEvent::get_skip_event,
            CombatEvent::set_skip_event,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, SkillEvent>(
            "skill_event",
            CombatEvent::has_skill_event,
            CombatEvent::get_skill_event,
            CombatEvent::mut_skill_event,
            CombatEvent::set_skill_event,
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CombatEvent>(
            "CombatEvent",
            3,
            fields,
        )
    }
}

impl ::protobuf::Message for CombatEvent {
    fn is_initialized(&self) -> bool {
        if let Some(combat_event::Event::attack_event(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(combat_event::Event::consumable_event(ref v)) = self.event {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(combat_event::Event::skill_event(ref v)) = self.event {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::attack_event(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::consumable_event(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::equipable_event(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::forfeit_event(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::skip_event(is.read_bool()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.event = ::std::option::Option::Some(combat_event::Event::skill_event(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &combat_event::Event::attack_event(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &combat_event::Event::consumable_event(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &combat_event::Event::equipable_event(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &combat_event::Event::forfeit_event(v) => {
                    my_size += 2;
                },
                &combat_event::Event::skip_event(v) => {
                    my_size += 2;
                },
                &combat_event::Event::skill_event(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.event {
            match v {
                &combat_event::Event::attack_event(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &combat_event::Event::consumable_event(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &combat_event::Event::equipable_event(ref v) => {
                    os.write_string(3, v)?;
                },
                &combat_event::Event::forfeit_event(v) => {
                    os.write_bool(4, v)?;
                },
                &combat_event::Event::skip_event(v) => {
                    os.write_bool(5, v)?;
                },
                &combat_event::Event::skill_event(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> CombatEvent {
        CombatEvent::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 3)
    }

    fn default_instance() -> &'static CombatEvent {
        static instance: CombatEvent = CombatEvent {
            event: ::std::option::Option::None,
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for CombatEvent {
    fn clear(&mut self) {
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.event = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CombatEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CombatEvent {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CombatEvent`
pub mod combat_event {

    #[derive(Clone,PartialEq,Debug)]
    pub enum Event {
        attack_event(super::AttackEvent),
        consumable_event(super::ConsumableEvent),
        equipable_event(::std::string::String),
        forfeit_event(bool),
        skip_event(bool),
        skill_event(super::SkillEvent),
    }

    impl ::protobuf::Oneof for Event {
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12combat_event.proto\x1a\x0bparty.proto\x1a\x0ctarget.proto\"0\n\x0b\
    AttackEvent\x12!\n\x07targets\x18\x01\x20\x03(\x0b2\x07.TargetR\x07targe\
    ts\"E\n\nSkillEvent\x12\x14\n\x05skill\x18\x02\x20\x01(\tR\x05skill\x12!\
    \n\x07targets\x18\x01\x20\x03(\x0b2\x07.TargetR\x07targets\"T\n\x0fConsu\
    mableEvent\x12\x1e\n\nconsumable\x18\x02\x20\x01(\tR\nconsumable\x12!\n\
    \x07targets\x18\x01\x20\x03(\x0b2\x07.TargetR\x07targets\"\xab\x02\n\x0b\
    CombatEvent\x121\n\x0cattack_event\x18\x01\x20\x01(\x0b2\x0c.AttackEvent\
    H\0R\x0battackEvent\x12=\n\x10consumable_event\x18\x02\x20\x01(\x0b2\x10\
    .ConsumableEventH\0R\x0fconsumableEvent\x12)\n\x0fequipable_event\x18\
    \x03\x20\x01(\tH\0R\x0eequipableEvent\x12%\n\rforfeit_event\x18\x04\x20\
    \x01(\x08H\0R\x0cforfeitEvent\x12\x1f\n\nskip_event\x18\x05\x20\x01(\x08\
    H\0R\tskipEvent\x12.\n\x0bskill_event\x18\x06\x20\x01(\x0b2\x0b.SkillEve\
    ntH\0R\nskillEventB\x07\n\x05eventb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> ::protobuf::reflect::FileDescriptor {
    static file_descriptor_lazy: ::protobuf::rt::LazyV2<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::LazyV2::INIT;
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        deps.push(super::party::file_descriptor());
        deps.push(super::target::file_descriptor());
        let mut messages = ::std::vec::Vec::new();
        messages.push(AttackEvent::generated_message_descriptor_data());
        messages.push(SkillEvent::generated_message_descriptor_data());
        messages.push(ConsumableEvent::generated_message_descriptor_data());
        messages.push(CombatEvent::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    ::protobuf::reflect::FileDescriptor::new_generated_2(file_descriptor)
}