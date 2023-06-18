use std::io::Write as CursorWrite;

use crate::{
    context::Context,
    error::{Error, Result},
    write::Write,
    write_encoder::WriteEncoder,
};
use serde::ser::{self, Serialize};

pub struct ArraySerializer<'a> {
    array_len: u32,
    array_serializer: Serializer,
    parent_encoder: &'a mut WriteEncoder,
}

impl ser::SerializeSeq for ArraySerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut self.array_serializer)?;
        self.array_len += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.parent_encoder.write_array_length(&self.array_len);
        self.parent_encoder
            .write(&self.array_serializer.write_encoder.get_buffer());
        Ok(())
    }
}

impl ser::SerializeTuple for ArraySerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut self.array_serializer)?;
        self.array_len += 1;
        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.parent_encoder.write_array_length(&self.array_len);
        self.parent_encoder
            .write(&self.array_serializer.write_encoder.get_buffer());
        Ok(())
    }
}

pub struct Serializer {
    write_encoder: WriteEncoder,
}

impl Default for Serializer {
    fn default() -> Self {
        Self {
            write_encoder: WriteEncoder::new(&[], Context::new()),
        }
    }
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)?;
    Ok(serializer.write_encoder.get_buffer())
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ArraySerializer<'a>;
    // TODO: should tuples be serialized as sequences?. Ex: (u8, bool) = [3, true]?
    type SerializeTuple = ArraySerializer<'a>;
    // TODO: should tuples be serialized as sequences?. Ex: Color(u8, bool) = [3, true]?
    type SerializeTupleStruct = Self;
    // TODO: should tuples be serialized as sequences?. Ex: Color(u8, bool) = [3, true]?
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_encoder.write_bool(&v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write_encoder.write_i8(&v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write_encoder.write_i16(&v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_encoder.write_i32(&v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_encoder.write_i64(&v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_encoder.write_u8(&v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_encoder.write_u16(&v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_encoder.write_u32(&v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_encoder.write_u64(&v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_encoder.write_f32(&v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_encoder.write_f64(&v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.write_encoder.write_string(&v.to_string());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write_encoder.write_string(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write_encoder.write_bytes(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.write_encoder.write_nil();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.write_encoder.write_u32(&_variant_index);
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        // TODO: optimize for the case where len is defined

        let array_ser = ArraySerializer {
            array_len: 0,
            array_serializer: Serializer::default(),
            parent_encoder: &mut self.write_encoder,
        };
        Ok(array_ser)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output += "{";
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.


// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}}";
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::to_string;
    use serde_derive::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expected = r#"{"int":1,"seq":["a","b"]}"#;
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected = r#""Unit""#;
        assert_eq!(to_string(&u).unwrap(), expected);

        let n = E::Newtype(1);
        let expected = r#"{"Newtype":1}"#;
        assert_eq!(to_string(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        let expected = r#"{"Tuple":[1,2]}"#;
        assert_eq!(to_string(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        let expected = r#"{"Struct":{"a":1}}"#;
        assert_eq!(to_string(&s).unwrap(), expected);
    }
}
