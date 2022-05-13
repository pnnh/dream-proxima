use crate::config::is_debug;
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
};

#[derive(Clone, Copy)]
pub struct SimpleHelper;

impl HelperDef for SimpleHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        let release_path = if let Some(v) = h.param(1) {
            v.render()
        } else {
            "".to_string()
        };

        let res_url = if is_debug() {
            "http://127.0.0.1:3000"
        } else {
            "https://res.sfx.xyz"
        };
        out.write(res_url)?;
        if is_debug() || release_path.is_empty() {
            out.write(param.value().render().as_ref())?;
        } else {
            out.write(release_path.as_str())?;
        }
        Ok(())
    }
}
