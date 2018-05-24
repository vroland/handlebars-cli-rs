
use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError};
use serde_json::{Value, Number};

#[cfg(feature = "mediawiki")]
use mwparser_utils::util::filename_to_make;

#[cfg(feature = "mediawiki")]
pub struct MakeEscapeHelper;

#[cfg(feature = "mediawiki")]
impl HelperDef for MakeEscapeHelper {
     fn call_inner(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &mut RenderContext,
    ) -> Result<Option<Value>, RenderError> {
        let path = try!(h.param(0,)
            .and_then(|v| v.value().as_str(),)
            .ok_or(RenderError::new(
                "Param 0 with str type is required for make escape helper"
            ),)
        );
        Ok(Some(Value::String(filename_to_make(&path))))
    }
}

pub struct AddHelper;

impl HelperDef for AddHelper {
    fn call_inner(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &mut RenderContext,
    ) -> Result<Option<Value>, RenderError> {
        let p1 = try!(
            h.param(0,)
                .and_then(|v| v.value().as_f64(),)
                .ok_or(RenderError::new(
                    "Param 0 with f64 type is required for add helper."
                ),)
        );
        let p2 = try!(
            h.param(1,)
                .and_then(|v| v.value().as_f64(),)
                .ok_or(RenderError::new(
                    "Param 1 with f64 type is required for add helper."
                ),)
        );

        Ok(Some(Value::Number(Number::from_f64(p1 + p2).unwrap_or(0.into()))))
    }
}

pub struct MultHelper;

impl HelperDef for MultHelper {
    fn call_inner(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &mut RenderContext,
    ) -> Result<Option<Value>, RenderError> {
        let p1 = try!(
            h.param(0,)
                .and_then(|v| v.value().as_f64(),)
                .ok_or(RenderError::new(
                    "Param 0 with f64 type is required for add helper."
                ),)
        );
        let p2 = try!(
            h.param(1,)
                .and_then(|v| v.value().as_f64(),)
                .ok_or(RenderError::new(
                    "Param 1 with f64 type is required for add helper."
                ),)
        );

        Ok(Some(Value::Number(Number::from_f64(p1 * p2).unwrap_or(0.into()))))
    }
}
