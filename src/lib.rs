use std::collections::BTreeMap;
use std::rc::Rc;

/// An enum describing why a method failed.
pub enum Error {
    /// The method failed because it would cause the debuggee to run.
    DebuggeeWouldRun,

    /// The method failed because the environment is not a debuggee environment.
    EnvironmentNotDebuggee,

    /// The method failed because the frame is not a debuggee frame.
    FrameNotDebuggee,

    /// The method failed because the object is not extensible.
    ObjectNotExtensible,

    /// The method failed because the object is not a global.
    ObjectNotGlobal,

    /// The method failed because the offset is not a valid.
    OffsetNotValid,

    /// The method failed because the property is not configurable.
    PropertyNotConfigurable,

    /// The method failed because there is no such variable.
    VariableNotFound
}

/// A return type for methods that are fallible.
pub type Fallible<T> = Result<T, Error>;

/// A value in the debuggee. This is either a primitive value or a wrapper to an
/// object in the debuggee. A primitive value is either undefined, null, a
/// boolean, a string, or a number.
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Object(Object)
}

/// A completion value describes how a call or evaluation completed.
pub enum CompletionValue {
    /// The call or evaluation returned the given value as a result.
    Return(Value),

    /// The call or evaluation threw the given value as an exception.
    Throw(Value),

    /// The call or evaluation was terminated.
    Terminate
}

/// A resumption value describes how the debuggee should continue executing.
pub type ResumptionValue = Option<CompletionValue>;

/// An enum describing the type of an environment.
pub enum EnvironmentType {
    /// And environment introduced by a function call, call to `eval`, etc.
    Declarative,

    /// An environment introduced by a global or DOM element.
    Object,

    /// An environment introduced by a `with` statement.
    With
}

/// A wrapper to a lexical environment 
pub struct Environment;

impl Environment {
    /// If the wrapped environment is the variable environment for a function
    /// call, returns a wrapper to the function being called. Otherwise,
    /// returns `None`.
    ///
    /// # Errors
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn callee(&self) -> Fallible<Option<Object>> {
        unimplemented!()
    }

    /// Returns a wrapper to the innermost environment that binds a variable to
    /// the given `name`. If there is no such environment, returns `None`
    /// instead.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the object
    /// reflected by a wrapped environment is a proxy, returns
    /// `DebuggeeWouldRun`.
    ///
    /// If a wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn find(&self, name: &str) -> Fallible<Option<Environment>> {
        unimplemented!()
    }

    /// Returns the value of the variable bound to the given `name` by the
    /// wrapped environment. If there is no such variable, returns `Undefined`
    /// instead.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the object
    /// reflected by the wrapped environment is a proxy, returns
    /// `DebuggeeWouldRun`.
    ///
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn get_variable(&self, name: &str) -> Fallible<Value> {
        unimplemented!()
    }

    /// Returns `true` if the wrapped environment is a debuggee environment.
    /// Returns `false` otherwise.
    pub fn is_inspectable() -> bool {
        unimplemented!()
    }

    /// Returns the names of the variables bound by the wrapped environment.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the object
    /// reflected by the wrapped environment is a proxy, returns
    /// `DebuggeeWouldRun`.
    ///
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn names(&self) -> Fallible<Vec<String>> {
        unimplemented!()
    }

    /// If the wrapped environment is a global or with environment, returns a
    /// wrapper to the object reflected by the wrapped environment. Otherwise,
    /// returns `None`.
    ///
    /// # Errors
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn object(&self) -> Fallible<Option<Object>> {
        unimplemented!()
    }

    /// Returns `true` if the wrapped environment is optimized out. Returns
    /// `false` otherwise.
    ///
    /// # Errors
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn is_optimized_out(&self) -> Fallible<bool> {
        unimplemented!()
    }

    /// Returns a wrapper to the environment enclosing the wrapped environment.
    /// If the there is no environment enclosing the wrapped environment because
    /// the wrapped environment is the outermost environment, returns `None`
    /// instead.
    ///
    /// # Errors
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn parent(&self) -> Fallible<Option<Environment>> {
        unimplemented!()
    }

    /// Sets the value of the variable bound to the given `name` by the wrapped
    /// environment.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the object
    /// reflected by the wrapped environment is a proxy, returns
    /// `DebuggeeWouldRun`.
    ///
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    ///
    /// If the value of the variable could not be set because there is no such
    /// variable, returns `VariableNotFound`.
    pub fn set_variable(&self, name: &str, value: &Value) -> Fallible<()> {
        unimplemented!()
    }

    /// Returns the type of the wrapped environment.
    ///
    /// # Errors
    /// If the wrapped environment could not be inspected because it is not a
    /// debuggee environment, returns `EnvironmentNotDebuggee`.
    pub fn get_type(&self) -> Fallible<EnvironmentType> {
        unimplemented!()
    }
}

/// An enum describing where a frame is executing.
pub enum FrameImplementation {
    /// A frame executing in the interpreter.
    Interpreter,

    /// A frame executing in the non-optimized JIT.
    Baseline,

    /// A frame executing in the optimized JIT.
    Ion,
}

/// An enum describing the type of a frame.
pub enum FrameType {
    /// A frame introduced by a function call.
    Call,

    /// A frame introduced by a call to `eval`.
    Eval,

    /// A frame introduced by global code.
    Global,

    /// A frame introduced by top-level module code.
    Module
}

/// A trait for values that can be used as pop handler.
pub trait PopHandler {
    fn handle(&self, frame: &Frame, completion: &CompletionValue) -> ResumptionValue;
}

/// A trait for values that can be used as step handler.
pub trait StepHandler {
    fn handle(&self, frame: &Frame) -> ResumptionValue;
}

/// A wrapper to a stack frame.
pub struct Frame;

impl Frame {
    /// If the wrapped frame is a call frame, returns the arguments for the
    /// call. Otherwise, returns `None`.
    pub fn arguments(&self) -> Option<Vec<Value>> {
        unimplemented!()
    }

    /// If the wrapped frame is a call frame, returns a wrapper to the function
    /// being called. Otherwise, returns `None`.
    pub fn callee(&self) -> Option<Object> {
        unimplemented!()
    }

    /// Returns the depth of the wrapped frame on the stack.
    pub fn depth(&self) -> u32 {
        unimplemented!()
    }

    /// Returns `true` if the wrapped frame is a call frame for a function being
    /// called as a constructor. Returns `false` otherwise.
    pub fn is_constructing(&self) -> bool {
        unimplemented!()
    }

    /// Returns the environment in which the wrapped frame is executing. If
    /// the wrapped frame does not have an environment because it is not a
    /// debuggee frame, returns `None` instead.
    pub fn environment(&self) -> Option<Environment> {
        unimplemented!()
    }

    /// Evaluates the given `code` in the environment of the wrapped frame.
    /// Returns a completion value describing how the evaluation completed.
    ///
    /// # Errors
    /// If wrapped frame does not have an environment because it is not a
    /// debuggee frame, returns `FrameNotDebuggee`.
    pub fn eval(&self, code: &str) -> CompletionValue {
        unimplemented!()
    }

    /// Evaluates the given `code` in the environment of the wrapped frame,
    /// extending the environment with the given `bindings`, where `bindings` is
    /// a map from variable names to values. Returns a completion value
    /// describing how the evaluation completed.
    ///
    /// # Errors
    /// If wrapped frame does not have an environment because it is not a
    /// debuggee frame, returns `FrameNotDebuggee`.
    pub fn eval_with_bindings(&self, code: &str, bindings: &BTreeMap<String, Value>) -> CompletionValue {
        unimplemented!()
    }

    /// Returns the type of the wrapped frame.
    pub fn get_type(&self) -> FrameType {
        unimplemented!()
    }

    /// Returns the implementation of the wrapped frame.
    pub fn implementation(&self) -> FrameImplementation {
        unimplemented!()
    }

    /// Returns `true` if the wrapped frame is still on the stack. Returns
    /// `false` otherwise.
    pub fn is_live(&self) -> bool {
        unimplemented!()
    }

    /// Returns the offset of the bytecode being executed in the script of the
    /// wrapped frame. If the wrapped frame does not have a script because it is
    /// not a debuggee frame, returns `None` instead.
    pub fn offset(&self) -> Option<u32> {
        unimplemented!()
    }

    /// Returns a wrapper to the next-older visible frame. If there is no such
    /// frame, returns `None` instead.
    pub fn older(&self) -> Option<Frame> {
        unimplemented!()
    }

    /// Returns the pop handler for the wrapped frame. If there is no pop
    /// handler for the wrapped frame, returns `None` instead.
    pub fn pop_handler(&self) -> Option<Rc<Box<PopHandler>>> {
        unimplemented!()
    }

    /// Returns the step handler for the wrapped frame. If there is no step
    /// handler for the wrapped frame, returns `None` instead.
    pub fn step_handler(&self) -> Option<Rc<Box<StepHandler>>> {
        unimplemented!()
    }

    /// Returns the script being executed in the wrapped frame. If the wrapped
    /// frame does not have a script because it is not a debuggee frame, returns
    /// `None` instead.
    pub fn script(&self) -> Option<Script> {
        unimplemented!()
    }

    /// Sets the pop handler for the wrapped frame to the given `handler`. When
    /// the wrapped frame is popped from the stack, the `handle` method of the
    /// given `handler` will be called. If the given `handler` is `None`, the
    /// pop handler for the wrapped frame is cleared instead.
    pub fn set_pop_handler(&self, handler: Option<Rc<Box<PopHandler>>>) -> () {
        unimplemented!()
    }

    /// Sets the step handler for the wrapped frame to the given `handler`. When
    /// the offset of the bytecode being executed in the script of the wrapped
    /// frame changes, the `handle` method of the given `handler` will be
    /// called. If the given handler is `None`, the step handler for the wrapped
    /// frame is cleared instead.
    pub fn set_step_handler(&self, handler: Option<Rc<Box<StepHandler>>>) -> () {
        unimplemented!()
    }

    /// If the wrapped frame is a call frame, returns the this value for the
    /// call. Otherwise, returns `None`.
    pub fn this(&self) -> Value {
        unimplemented!()
    }
}

/// A property descriptor describes a property. It consists of zero or more
/// optional attributes.
///
/// A property descriptor is either a data property descriptor, an accessor
/// property descriptor, or a generic property descriptor. A data property
/// descriptor has either a writable or a value attribute. An accessor property
/// has either a get or a set attribute. A generic property descriptor is
/// neither a data property descriptor nor an accessor property descriptor.
pub struct PropertyDescriptor {
    /// If `false`, the attributes of the property, except its value, may not
    /// be changed. Defaults to `false`.
    configurable: Option<bool>,

    /// If `false,` the property will not be enumerated by a for-in enumeration.
    /// Defaults to `false`.
    enumerable: Option<bool>,

    /// If `false`, the value of the property may not be changed. Defaults to
    /// `false`.
    writable: Option<bool>,

    /// The value of the property. Defaults to `Undefined`.
    value: Option<Value>,

    /// A getter for the property. Must be either a wrapper to a function or
    /// `Undefined`. Defaults to `Undefined`.
    get: Option<Value>,

    /// A setter for the property. Must be either a wrapper to a function or
    /// `Undefined`. Defaults to `Undefined`.
    set: Option<Value>
}

/// A wrapper to an object in the debuggee.
pub struct Object;

impl Object {
    /// Returns a wrapper to the global environment of the wrapped object.
    ///
    /// # Errors
    /// If the wrapped object does not have a global environment because it is
    /// not a global, returns `ObjectNotGlobal`.
    pub fn as_environment(&self) -> Fallible<Environment> {
        unimplemented!()
    }

    /// If the wrapped object is a bound function, returns the arguments to
    /// which it was bound. Otherwise, returns `None`.
    pub fn bound_arguments(&self) -> Option<Vec<Value>> {
        unimplemented!()
    }

    /// If the wrapped object is a bound function, returns the target function;
    /// that is, the function that was bound to a particular this value and
    /// arguments. Otherwise, returns `None`.
    pub fn bound_target_function(&self) -> Option<Object> {
        unimplemented!()
    }

    /// If the wrapped object is a bound function, returns the this value to
    /// which it was bound. Otherwise, returns `Undefined`.
    pub fn bound_this(&self) -> Value {
        unimplemented!()
    }

    /// Calls the wrapped object with the given `this` value and `arguments`.
    /// Returns a completion value describing how the call completed.
    ///
    /// # Errors
    /// If the wrapped object could not be called because it is not callable,
    /// returns `ObjectNotCallable`.
    pub fn call(&self, this: &Value, arguments: &[Value]) -> Fallible<CompletionValue> {
        unimplemented!();
    }

    /// Calls the wrapped object as a constructor with the given `arguments`.
    /// Returns a completion value describing how the call completed.
    ///
    /// # Errors
    /// If the wrapped object could not be called because it is not callable,
    /// returns `ObjectNotCallable`.
    pub fn construct(&self, arguments: &[Value]) -> Fallible<CompletionValue> {
        unimplemented!();
    }

    /// Returns the value of the internal property [[Class]] of the wrapped
    /// object.
    pub fn class(&self) -> String {
        unimplemented!()
    }

    /// If the wrapped object is a function with a display name, returns the
    /// display name of the function. Otherwise, returns `None`.
    pub fn display_name(&self) -> Option<String> {
        unimplemented!()
    }

    /// Defines a property with the given `name` on the wrapped object, as
    /// described by the given `descriptor`. If there already is such a
    /// property, modifies the existing property. Otherwise, adds a new
    /// property.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    ///
    /// If a new property could not be added because the wrapped object is not
    /// extensible, returns `ObjectNotExtensible`.
    ///
    /// If an existing property could not be modified because the property is
    /// not configurable, returns `PropertyNotConfigurable`.
    pub fn define_property(&self, name: &str, descriptor: &PropertyDescriptor) -> Fallible<()> {
        unimplemented!()
    }

    /// Deletes the property with the given `name` from the wrapped object.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    ///
    /// If the property could not be deleted because it is non-configurable,
    /// returns `PropertyNotConfigurable`.
    pub fn delete_property(&self, name: &str) -> Fallible<()> {
        unimplemented!()
    }

    /// If the wrapped object is a function in the debuggee, returns a wrapper
    /// to the environment in which the function was created. Otherwise, returns
    /// `None`.
    pub fn environment(&self) -> Option<Environment> {
        unimplemented!()
    }

    /// Executes the given `code` in the global environment of the wrapped
    /// object. Returns a completion value describing how the execution
    /// completed.
    ///
    /// # Errors
    /// If the wrapped object does not have a global environment because it is
    /// not a global, returns `ObjectNotGlobal`.
    pub fn execute_in_global(&self, code: &str) -> Fallible<CompletionValue> {
        unimplemented!()
    }

    /// Executes the given `code` in the global environment of the wrapped
    /// object, extending the environment with the given `bindings`, where
    /// `bindings` is a map from variable names to values. Returns a completion
    /// value describing how the execution completed.
    ///
    /// # Errors
    /// If the wrapped object does not have a global environment because it is
    /// not a global, returns `ObjectNotGlobal`.
    pub fn execute_in_global_with_bindings(&self, code: &str, bindings: &BTreeMap<String, Value>) -> Fallible<CompletionValue> {
        unimplemented!()
    }

    /// Freezes the wrapped object; that is, prevents extensions on it, and
    /// makes all its properties non-configurable and non-writable.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn freeze(&self) -> Fallible<()> {
        unimplemented!()
    }

    /// Returns a property descriptor for the own property with the given
    /// `name` on the wrapped object.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn get_own_property_descriptor(&self, name: &str) -> Fallible<PropertyDescriptor> {
        unimplemented!()
    }

    /// Returns the names of the own properties of the wrapped object.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn get_own_property_names(&self) -> Fallible<Vec<String>> {
        unimplemented!()
    }

    /// Returns a wrapper to the prototype of the wrapped object. If the
    /// wrapped object does not have a prototype, returns `None` instead.
    pub fn get_prototype_of(&self) -> Option<Object> {
        unimplemented!()
    }

    /// Returns a wrapper to the global of the wrapped object.
    pub fn global(&self) -> Object {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is an arrow function. Returns
    /// `false` otherwise.
    pub fn is_arrow_function(&self) -> bool {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is a bound function. Returns
    /// `false` otherwise.
    pub fn is_bound_function(&self) -> bool {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is callable. Returns `false`
    /// otherwise.
    pub fn is_callable(&self) -> bool {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is extensible; that is, if
    /// properties can be added to it. Returns `false` otherwise.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn is_extensible(&self) -> Fallible<bool> {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is frozen; that is, if it is not
    /// extensible, and all its properties are non-configurable and
    /// non-writable. Returns `false` otherwise.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn is_frozen(&self) -> Fallible<bool> {
        unimplemented!()
    }

    /// Returns `true` if the wrapped object is sealed; that is, if it is not
    /// extensible, and all its properties are non-configurable. Returns
    /// `false` otherwise.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn is_sealed(&self) -> Fallible<bool> {
        unimplemented!()
    }

    /// If the wrapped object is a named function, returns the name of the
    /// function. Otherwise, returns `None`.
    pub fn name(&self) -> Option<String> {
        unimplemented!()
    }

    /// If the wrapped object is a function in the debuggee, returns the names
    /// of the parameters of the function. Otherwise, returns `None`.
    pub fn parameter_names(&self) -> Option<Vec<String>> {
        unimplemented!()
    }

    /// Prevents extensions on the wrapped object; that is, prevents properties
    /// from being added to it.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn prevent_extensions(&self) -> Fallible<()> {
        unimplemented!()
    }

    /// Seals the wrapped object; that is, prevents extensions on it, and makes
    /// all its properties non-configurable.
    ///
    /// # Errors
    /// If this method would cause the debuggee to run because the wrapped
    /// object is a proxy, returns `DebuggeeWouldRun`.
    pub fn seal(&self) -> Fallible<()> {
        unimplemented!()
    }

    /// If the wrapped object is a function in the debuggee, returns a wrapper
    /// to the script of the function. Otherwise, returns `None`.
    pub fn script(&self) -> Option<Script> {
        unimplemented!()
    }
}

/// A trait for values that can be used as breakpoint handler.
pub trait BreakpointHandler {
    fn handle(&self, frame: &Frame) -> ResumptionValue;
}

/// A wrapper to a compiled script.
pub struct Script;

impl Script {
    /// Clears all the breakpoints in this script.
    pub fn clear_all_breakpoints(&self) -> () {
        unimplemented!()
    }

    /// Clears the breakpoints at the given `offset` in the wrapped script.
    ///
    /// # Errors
    /// If the given `offset` is not a valid offset in the wrapped script,
    /// returns `OffsetNotValid`.
    pub fn clear_breakpoints(&self, offset: u32) -> Fallible<()> {
        unimplemented!()
    }

    /// Returns the display name of the wrapped script. If the wrapped script
    /// has no display name, returns `None` instead.
    pub fn display_name(&self) -> Option<String> {
        unimplemented!()
    }

    /// Returns a map from lines to the offsets that are entry points for each
    /// line.
    pub fn get_all_line_offsets(&self) -> BTreeMap<u32, Vec<u32>> {
        unimplemented!()
    }

    /// Returns the handlers for the breakpoints at the given `offset` in the
    /// wrapped script.
    ///
    /// # Errors
    /// If the given `offset` is not a valid offset in the wrapped script,
    /// returns `OffsetNotValid`.
    pub fn get_breakpoints(&self, offset: u32) -> Vec<Rc<Box<BreakpointHandler>>> {
        unimplemented!()
    }

    /// Returns the wrappers to the scripts for each function in the wrapped
    /// script.
    pub fn get_child_scripts(&self) -> Vec<Script> {
        unimplemented!()
    }

    /// Returns the offsets that are entry points for the given `line`.
    pub fn get_line_offsets(&self, line: u32) -> Vec<u32> {
        unimplemented!()
    }

    /// Returns a wrapper to the global in which the script is being executed.
    pub fn global(&self) -> Object {
        unimplemented!()
    }

    /// The number of lines spanned by the code of the wrapped script in the
    /// document from which its source was loaded.
    pub fn line_count(&self) -> u32 {
        unimplemented!()
    }

    /// Sets a breakpoint at the given `offset` in the wrapped script. When the
    /// breakpoint is hit, the `handle` method of the given `handler` will be
    /// called.
    ///
    /// # Errors
    /// If the given `offset` is not a valid offset in the wrapped script,
    /// returns `OffsetNotValid`.
    pub fn set_breakpoint(&self, offset: u32, handler: Rc<Box<BreakpointHandler>>) -> Fallible<()> {
        unimplemented!()
    }

    /// Returns a wrapper to the source from which the wrapped script was
    /// compiled. If the source was not retained, returns `None` instead.
    pub fn source(&self) -> Option<Source> {
        unimplemented!()
    }

    /// The number of characters spanned by the code of the wrapped script in
    /// the source from which it was compiled.
    pub fn source_length(&self) -> u32 {
        unimplemented!()
    }

    /// The index of the character at which the code of the wrapped script
    /// starts in the source from which it was compiled.
    pub fn source_start(&self) -> u32 {
        unimplemented!()
    }

    /// The line number at which the code of the wrapped script starts in the
    /// document from which its source was loaded.
    pub fn start_line(&self) -> u32 {
        unimplemented!()
    }

    /// Returns the url of the document from which the source of the wrapped
    /// script was loaded.
    pub fn url(&self) -> String {
        unimplemented!()
    }
}

/// An enum describing how a source was introduced.
pub enum IntroductionType {
    /// A source introduced by a call to `eval`.
    Eval,

    /// A source introduced by a DOM event handler.
    EventHandler,

    /// A source introduced by a function call.
    Function,

    /// A source introduced by a call to `importScripts`.
    ImportScripts,

    /// A source introduced by a javascript: URL.
    JavaScriptURL,

    /// A source introduced by a <script> element.
    ScriptElement,

    /// A source introduced by a call to `setInterval`.
    SetInterval,

    /// A source introduced by a call to `setTimeout`.
    SetTimeout,

    /// A source introduced by a call to `Worker`.
    Worker
}

/// A wrapped to a JavaScript source.
pub struct Source;

impl Source {
    /// Returns a unique identifier for the wrapped source. This allows
    /// different wrappers to the same source to be compared.
    pub fn canonical_id(&self) -> String {
        unimplemented!();
    }

    /// If the wrapped source was introduced by a DOM element, returns a wrapper
    /// to that DOM element. Otherwise, returns `None`.
    pub fn element(&self) -> Option<Object> {
        unimplemented!()
    }

    /// If the wrapped source was introduced by an attribute on a DOM element,
    /// returns the name of that attribute. Otherwise, returns `None`.
    pub fn element_attribute_name(&self) -> Option<String> {
        unimplemented!()
    }

    /// If the wrapped source was introduced by a function call in the debuggee,
    /// returns the offset of the bytecode for the call. Otherwise, returns
    /// `None`.
    pub fn introduction_offset() -> Option<u32> {
        unimplemented!()
    }

    /// If the wrapped source was introduced by a function call in the debuggee,
    /// returns a wrapper to the script containing the call. Otherwise, returns
    /// `None`.
    pub fn introduction_script() -> Option<Script> {
        unimplemented!()
    }

    /// Returns the introduction type of the wrapped source. If the introduction
    /// type of the wrapped source is unknown, returns `None` instead.
    pub fn introduction_type() -> Option<IntroductionType> {
        unimplemented!()
    }

    /// If the wrapped source is source mapped, and the URL of the source map is
    /// known, returns that URL. Otherwise, returns `None`.
    pub fn source_map_url(&self) -> Option<String> {
        unimplemented!()
    }

    /// Returns the text of the wrapped source.
    pub fn text(&self) -> String {
        unimplemented!()
    }

    /// Returns the url of the document from which the wrapped source was
    /// loaded.
    pub fn url(&self) -> String {
        unimplemented!()
    }
}
