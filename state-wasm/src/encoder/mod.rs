use ::backtrace::Backtrace;

use ::wasm_bindgen::JsValue;

#[derive(Debug)]
pub(crate) struct EncoderError {
    kind: EncoderErrorKind,
    stack: Backtrace,
}

impl EncoderError {
    fn new(kind: EncoderErrorKind) -> Self {
        EncoderError {
            kind,
            stack: Backtrace::new(),
        }
    }
}

impl From<EncoderErrorKind> for EncoderError {
    fn from(kind: EncoderErrorKind) -> Self {
        EncoderError::new(kind)
    }
}

#[derive(Debug)]
pub(crate) enum EncoderErrorKind {
    JsValueNotF64,
    JsValueNotI32(f64),
    JsValueNotBool,
    JsValueNotString,
}

pub(crate) trait JsValueEncoder
where
    Self: Sized,
{
    fn encode(value: Self) -> Result<JsValue, EncoderError>;
    fn decode(value: JsValue) -> Result<Self, EncoderError>;
}

impl JsValueEncoder for i32 {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(value))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        let f = value.as_f64().ok_or(EncoderErrorKind::JsValueNotF64)?;

        if f > (i32::MAX as f64) {
            Err(EncoderErrorKind::JsValueNotI32(f))?
        }

        Ok(f as i32)
    }
}

impl JsValueEncoder for f64 {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(value))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(value.as_f64().ok_or(EncoderErrorKind::JsValueNotF64)?)
    }
}

impl JsValueEncoder for bool {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(value))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(value.as_bool().ok_or(EncoderErrorKind::JsValueNotBool)?)
    }
}

impl JsValueEncoder for String {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(value))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(value
            .as_string()
            .ok_or(EncoderErrorKind::JsValueNotString)?)
    }
}

impl JsValueEncoder for Vec<i8> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Int8Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Int8Array::from(value).to_vec())
    }
}

impl JsValueEncoder for Vec<i16> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Int16Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Int16Array::from(value).to_vec())
    }
}

impl JsValueEncoder for Vec<i32> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Int32Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Int32Array::from(value).to_vec())
    }
}

impl JsValueEncoder for Vec<u8> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Uint8Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Uint8Array::from(value).to_vec())
    }
}

impl JsValueEncoder for Vec<u16> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Uint16Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Uint16Array::from(value).to_vec())
    }
}

impl JsValueEncoder for Vec<u32> {
    fn encode(value: Self) -> Result<JsValue, EncoderError> {
        Ok(JsValue::from(js_sys::Uint32Array::from(value.as_ref())))
    }

    fn decode(value: JsValue) -> Result<Self, EncoderError> {
        Ok(js_sys::Uint32Array::from(value).to_vec())
    }
}
