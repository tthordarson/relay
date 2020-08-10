/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::Diagnostic;
use graphql_ir::{
    FragmentDefinition, OperationDefinition, Program, ValidationMessage, ValidationResult,
    Validator,
};
use graphql_syntax::OperationKind;

mod extract_module_name;

pub fn validate_module_names(program: &Program) -> ValidationResult<()> {
    (ValidateModuleNames {}).validate_program(program)
}

pub struct ValidateModuleNames {}

impl Validator for ValidateModuleNames {
    const NAME: &'static str = "ValidateModuleNames";
    const VALIDATE_ARGUMENTS: bool = false;
    const VALIDATE_DIRECTIVES: bool = true;

    fn validate_operation(&mut self, operation: &OperationDefinition) -> ValidationResult<()> {
        let operation_name = operation.name.item.to_string();
        let path = operation.name.location.source_location().path();
        let module_name =
            extract_module_name::extract_module_name(path).expect("Unable to extract module name.");
        let (operation_type_suffix, pluralized_string) = match operation.kind {
            OperationKind::Query => ("Query", "Queries"),
            OperationKind::Mutation => ("Mutation", "Mutations"),
            OperationKind::Subscription => ("Subscription", "Subscriptions"),
        };

        if !operation_name.starts_with(&module_name)
            || !operation_name.ends_with(operation_type_suffix)
        {
            return Err(vec![Diagnostic::new(
                ValidationMessage::InvalidOperationName {
                    pluralized_string: pluralized_string.to_string(),
                    operation_type_suffix: operation_type_suffix.to_string(),
                    module_name,
                    operation_name,
                },
                vec![operation.name.location],
            )]);
        }

        Ok(())
    }

    fn validate_fragment(&mut self, fragment: &FragmentDefinition) -> ValidationResult<()> {
        let fragment_name = fragment.name.item.to_string();
        let path = fragment.name.location.source_location().path();
        let module_name =
            extract_module_name::extract_module_name(path).expect("Unable to extract module name.");

        if !fragment_name.starts_with(&module_name) {
            return Err(vec![Diagnostic::new(
                ValidationMessage::InvalidFragmentName {
                    module_name,
                    fragment_name,
                },
                vec![fragment.name.location],
            )]);
        }
        Ok(())
    }
}