#![allow(clippy::not_unsafe_ptr_arg_deref)]

use matchable::Matchable;
use regex::Regex;
use serde::Deserialize;
use swc_core::{
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_react_remove_properties_visitor::ReactRemovePropertiesVisitor;

#[derive(Debug, Default, Deserialize)]
pub struct Options {
    properties: Option<Vec<Matchable>>,
}

#[plugin_transform]
pub fn react_remove_properties(
    program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let options = metadata
        .get_transform_plugin_config()
        .map(|json| {
            serde_json::from_str::<Options>(&json)
                .expect("failed to parse config of plugin 'react-remove-properties'")
        })
        .unwrap_or_default();
    program.fold_with(&mut as_folder(ReactRemovePropertiesVisitor {
        properties: options
            .properties
            .unwrap_or_else(|| vec![Matchable::Regex(Regex::new("^data-test").unwrap())]),
    }))
}
