use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::num::NonZeroUsize;

mod request;
mod response;
mod variables_arguments;

pub use self::{request::Request, response::Response, variables_arguments::VariablesArguments};

#[derive(Clone, Debug, Serialize)]
pub struct DebugCapability {
    #[serde(rename = "supportsCompletionsRequest")]
    pub supports_completions_request: bool,
    #[serde(rename = "supportsConditionalBreakpoints")]
    pub supports_conditional_breakpoints: bool,
    #[serde(rename = "supportsConfigurationDoneRequest")]
    pub supports_configuration_done_request: bool,
    #[serde(rename = "supportsDebuggerProperties")]
    pub supports_debugger_properties: bool,
    #[serde(rename = "supportsDelayedStackTraceLoading")]
    pub supports_delayed_stack_trace_loading: bool,
    #[serde(rename = "supportsEvaluateForHovers")]
    pub supports_evaluate_for_hovers: bool,
    #[serde(rename = "supportsExceptionInfoRequest")]
    pub supports_exception_info_request: bool,
    #[serde(rename = "supportsExceptionOptions")]
    pub supports_exception_options: bool,
    #[serde(rename = "supportsFunctionBreakpoints")]
    pub supports_function_breakpoints: bool,
    #[serde(rename = "supportsHitConditionalBreakpoints")]
    pub supports_hit_conditional_breakpoints: bool,
    #[serde(rename = "supportsLogPoints")]
    pub supports_log_points: bool,
    #[serde(rename = "supportsModulesRequest")]
    pub supports_modules_request: bool,
    #[serde(rename = "supportsSetExpression")]
    pub supports_set_expression: bool,
    #[serde(rename = "supportsSetVariable")]
    pub supports_set_variable: bool,
    /// Client supports the paging of variables.
    #[serde(rename = "supportsVariablePaging")]
    pub supports_variable_paging: bool,
    /// The debug adapter supports a `format` attribute on the `stackTrace`, `variables`, and `evaluate` requests.
    #[serde(rename = "supportsValueFormattingOptions")]
    pub supports_value_formatting_options: bool,
    #[serde(rename = "supportsTerminateDebuggee")]
    pub supports_terminate_debuggee: bool,
    #[serde(rename = "supportsGotoTargetsRequest")]
    pub supports_goto_targets_request: bool,
    #[serde(rename = "supportsClipboardContext")]
    pub supports_clipboard_context: bool,
    #[serde(rename = "supportsStepInTargetsRequest")]
    pub supports_step_in_targets_request: bool,
    #[serde(rename = "exceptionBreakpointFilters")]
    pub exception_breakpoint_filters: Vec<ExceptionBreakpointFilter>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExceptionBreakpointFilter {
    pub filter: String,
    pub label: String,
    pub default: bool,
}

impl Default for DebugCapability {
    fn default() -> Self {
        Self {
            supports_completions_request: true,
            supports_conditional_breakpoints: true,
            supports_configuration_done_request: true,
            supports_debugger_properties: true,
            supports_delayed_stack_trace_loading: true,
            supports_evaluate_for_hovers: true,
            supports_exception_info_request: true,
            supports_exception_options: true,
            supports_function_breakpoints: true,
            supports_hit_conditional_breakpoints: true,
            supports_log_points: true,
            supports_modules_request: true,
            supports_set_expression: true,
            supports_set_variable: true,
            supports_variable_paging: true,
            supports_value_formatting_options: true,
            supports_terminate_debuggee: true,
            supports_goto_targets_request: true,
            supports_clipboard_context: true,
            supports_step_in_targets_request: true,
            exception_breakpoint_filters: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct VariablesResponseBody {
    pub variables: Vec<Variable>,
}

impl<V: Into<Variable>> FromIterator<V> for VariablesResponseBody {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        VariablesResponseBody { variables: iter.into_iter().map(|v| v.into()).collect() }
    }
}

/// A Variable is a name/value pair.
///
/// The type attribute is shown if space permits or when hovering over the variable’s name.
///
/// The kind attribute is used to render additional properties of the variable, e.g. different icons can be used to indicate that a variable is public or private.
///
/// If the value is structured (has children), a handle is provided to retrieve the children with the variables request.
///
/// If the number of named or indexed children is large, the numbers should be returned via the namedVariables and indexedVariables attributes.
///
/// The client can use this information to present the children in a paged UI and fetch them in chunks.
#[derive(Clone, Debug, Serialize)]
pub struct Variable {
    /// The variable's name.
    pub name: String,
    /// The variable's value.
    /// This can be a multi-line text, e.g. for a function the body of a function.
    /// For structured variables (which do not have a simple value), it is
    /// recommended to provide a one-line representation of the structured object.
    /// This helps to identify the structured object in the collapsed state when
    /// its children are not yet visible.
    /// An empty string can be used if no value should be shown in the UI.
    pub value: String,
    /// The type of the variable's value. Typically shown in the UI when hovering
    /// over the value.
    /// This attribute should only be returned by a debug adapter if the
    /// corresponding capability `supportsVariableType` is true.
    #[serde(rename = "type")]
    pub typing: String,
    /// The evaluate name of this variable which can be passed to the `evaluate`
    /// request to fetch the variable's value.
    #[serde(rename = "evaluateName")]
    pub evaluate_name: String,
    /// If `variablesReference` is > 0, the variable is structured and its children
    /// can be retrieved by passing `variablesReference` to the `variables` request
    /// as long as execution remains suspended. See 'Lifetime of Object References'
    /// in the Overview section for details.
    #[serde(rename = "variablesReference")]
    pub variables_reference: usize,
    /// The number of named child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    #[serde(rename = "namedVariables")]
    pub named_variables: usize,
    /// The number of indexed child variables.
    /// The client can use this information to present the children in a paged UI
    /// and fetch them in chunks.
    #[serde(rename = "indexedVariables")]
    pub indexed_variables: usize,
    /// A memory reference to a location appropriate for this result.
    /// For pointer type eval results, this is generally a reference to the
    /// memory address contained in the pointer.
    /// This attribute may be returned by a debug adapter if corresponding
    /// capability `supportsMemoryReferences` is true.
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ModulesArguments {
    /// The index of the first module to return.
    ///
    /// if omitted modules start at 0.
    #[serde(rename = "startModule")]
    start: usize,
    /// The number of modules to return.
    ///
    /// If `moduleCount` is not specified or 0, all modules are returned.
    #[serde(rename = "moduleCount")]
    count: Option<NonZeroUsize>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ModulesResponseBody {
    pub modules: Vec<Module>,
    #[serde(rename = "totalModules")]
    pub total_modules: usize,
}

impl<M: Into<Module>> FromIterator<M> for ModulesResponseBody {
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self {
        let mut out = ModulesResponseBody { modules: vec![], total_modules: 0 };
        for m in iter.into_iter().map(|m| m.into()) {
            out.modules.push(m);
            out.total_modules += 1;
        }
        out
    }
}

/// An identifier for a module.
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    /// The module's identifier.
    pub id: u32,
    /// The module's name.
    pub name: String,
    /// The module's path.
    pub path: String,
    /// True if the module is optimized.
    #[serde(rename = "isOptimized")]
    pub is_optimized: bool,
    /// True if the module is considered 'user code' by a debugger that supports
    /// 'Just My Code'.
    #[serde(rename = "isUserCode")]
    pub is_user_code: bool,
    /// Version of Module.
    pub version: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableFilter {
    Indexed,
    Named,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueFormat {
    /// Display the value in hex.
    hex: Option<bool>,
}

/// This enumeration defines all possible access types for data breakpoints.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataBreakpointAccessType {
    Read,
    Write,
    ReadWrite,
}

/// The granularity of one ‘step’ in the stepping requests `next`, `stepIn`, `stepOut`, and `stepBack`
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SteppingGranularity {
    /// The step should allow the program to run until the current statement has finished executing.
    ///
    /// The meaning of a statement is determined by the adapter and it may be considered equivalent to a line.
    ///
    /// For example ‘for(int i = 0; i < 10; i++)’ could be considered to have 3 statements ‘int i = 0’, ‘i < 10’, and ‘i++’.
    Statement,
    /// The step should allow the program to run until the current source line has executed.
    Line,
    /// The step should allow one instruction to execute (e.g. one x86 instruction)
    Instruction,
}

/// The disconnect request asks the debug adapter to disconnect from the debuggee (thus ending the debug session) and then to shut down itself (the debug adapter).
///
/// In addition, the debug adapter must terminate the debuggee if it was started with the launch request. If an attach request was used to connect to the debuggee, then the debug adapter must not terminate the debuggee.
///
/// This implicit behavior of when to terminate the debuggee can be overridden with the terminateDebuggee argument (which is only supported by a debug adapter if the corresponding capability supportTerminateDebuggee is true).
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DisconnectArguments {
    /// A value of true indicates that this `disconnect` request is part of a
    /// restart sequence.
    pub restart: bool,

    /// Indicates whether the debuggee should be terminated when the debugger is disconnected.
    /// If unspecified, the debug adapter is free to do whatever it thinks is best.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportTerminateDebuggee` is true.
    #[serde(default, rename = "terminateDebuggee")]
    pub terminate_debuggee: bool,

    /// Indicates whether the debuggee should stay suspended when the debugger is disconnected.
    /// If unspecified, the debuggee should resume execution.
    /// The attribute is only honored by a debug adapter if the corresponding
    /// capability `supportSuspendDebuggee` is true.
    #[serde(default, rename = "suspendDebuggee")]
    pub suspend_debuggee: bool,
}
