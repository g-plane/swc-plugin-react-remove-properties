use matchable::Matchable;
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

pub struct ReactRemovePropertiesVisitor {
    pub properties: Vec<Matchable>,
}

impl VisitMut for ReactRemovePropertiesVisitor {
    fn visit_mut_jsx_opening_element(&mut self, jsx_opening_element: &mut JSXOpeningElement) {
        jsx_opening_element.visit_mut_children_with(self);

        jsx_opening_element.attrs.retain(|attr| {
            if let JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(Ident { sym, .. }),
                ..
            }) = attr
            {
                self.properties
                    .iter()
                    .all(|matchable| !matchable.is_match(sym))
            } else {
                true
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use swc_core::ecma::{transforms::testing::test, visit::as_folder};
    use swc_ecma_parser::{EsConfig, Syntax};

    test!(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| as_folder(ReactRemovePropertiesVisitor {
            properties: vec![
                Matchable::Str("data-foo".into()),
                Matchable::Str("data-bar".into())
            ]
        }),
        array,
        "<div data-test data-foo data-bar></div>",
        "<div data-test></div>"
    );

    test!(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| as_folder(ReactRemovePropertiesVisitor {
            properties: vec![Matchable::Regex(Regex::new("^data-test").unwrap()),]
        }),
        simple,
        r#"<div className="bar" data-test="thisIsASelectorForSelenium"></div>"#,
        r#"<div className="bar"></div>"#
    );

    test!(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| as_folder(ReactRemovePropertiesVisitor {
            properties: vec![Matchable::Regex(Regex::new("^data-test").unwrap()),]
        }),
        regex,
        r#"<div className="bar" data-test-long-string="thisIsASelectorForSelenium"></div>"#,
        r#"<div className="bar"></div>"#
    );
}
