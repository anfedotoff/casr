use crate::error;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
/// Classified information about program's execution.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ExecutionClass<'a> {
    /// Severity type.
    #[serde(rename(serialize = "Type", deserialize = "Type"))]
    pub severity: Cow<'a, str>,
    /// Class name.
    #[serde(rename(serialize = "ShortDescription", deserialize = "ShortDescription"))]
    pub short_description: Cow<'a, str>,
    /// Some description.
    #[serde(rename(serialize = "Description", deserialize = "Description"))]
    pub description: Cow<'a, str>,
    #[serde(rename(serialize = "Explanation", deserialize = "Explanation"))]
    pub explanation: Cow<'a, str>,
}

pub const CLASSES: &[(&str, &str, &str, &str); 71] = &[
    ("EXPLOITABLE", "SegFaultOnPc", "Segmentation fault on program counter", "The target tried to access data at an address that matches the program counter. This likely indicates that the program counter contents are tainted and can be controlled by an attacker."),
    ("EXPLOITABLE", "ReturnAv", "Access violation during return instruction", "The target crashed on a return instruction, which likely indicates stack corruption."),
    ("EXPLOITABLE", "BranchAv", "Access violation during branch instruction", "The target crashed on a branch instruction, which may indicate that the control flow is tainted."),
    ("EXPLOITABLE", "CallAv", "Access violation during call instruction", "The target crashed on a call instruction, which may indicate that the control flow is tainted."),
    ("EXPLOITABLE", "DestAv", "Access violation on destination operand", "The target crashed on an access violation at an address matching the destination operand of the instruction. This likely indicates a write access violation, which means the attacker may control the write address and/or value."),
    ("EXPLOITABLE", "BranchAvTainted", "Access violation during branch instruction from tainted source", "The target crashed on loading from memory (SourceAv). After taint tracking, target operand of branch instruction could be tainted."),
    ("EXPLOITABLE", "CallAvTainted", "Access violation during call instruction from tainted source", "The target crashed on loading from memory (SourceAv). After taint tracking, target operand of call instruction could be tainted."),
    ("EXPLOITABLE", "DestAvTainted", "Access violation on destination operand from tainted source", "The target crashed on loading from memory (SourceAv). After taint tracking, addres operand of memory store instruction could be tainted. This likely indicates a write access violation, which means the attacker may control the write address and/or value."),
    ("NOT_EXPLOITABLE", "AbortSignal", "Abort signal", "The target is stopped on a SIGABRT. SIGABRTs are often generated by libc and compiled check-code to indicate potentially exploitable conditions."),
    ("NOT_EXPLOITABLE", "TrapSignal", "Trap signal", "The target is stopped on a SIGTRAP. The SIGTRAP signal is sent to a process when an exception (or trap) occurs: a condition that a debugger has requested to be informed of – for example, when a particular function is executed, or when a particular variable changes value. "),
    ("NOT_EXPLOITABLE", "AccessViolation", "Access violation", "The target crashed due to an access violation but there is not enough additional information available to determine exploitability. Manual analysis is needed."),
    ("NOT_EXPLOITABLE", "SourceAv", "Access violation on source operand", "The target crashed on an access violation at an address matching the source operand of the current instruction. This likely indicates a read access violation."),
    ("PROBABLY_EXPLOITABLE", "BadInstruction", "Bad instruction", "The target tried to execute a malformed or privileged instruction. This may indicate that the control flow is tainted."),
    ("PROBABLY_EXPLOITABLE", "SegFaultOnPcNearNull", "Segmentation fault on program counter near NULL", "The target tried to access data at an address that matches the program counter. This may indicate that the program counter contents are tainted, however, it may also indicate a simple NULL dereference."),
    ("PROBABLY_EXPLOITABLE", "BranchAvNearNull", "Access violation near NULL during branch instruction", "The target crashed on a branch instruction, which may indicate that the control flow is tainted. However, there is a chance it could be a NULL dereference."),
    ("PROBABLY_EXPLOITABLE", "CallAvNearNull", "Access violation near NULL during call instruction", "The target crashed on a call instruction, which may indicate that the control flow is tainted. However, there is a chance it could be a NULL dereference."),
    ("PROBABLY_EXPLOITABLE", "DestAvNearNull", "Access violation near NULL on destination operand", "The target crashed on an access violation at an address matching the destination operand of the instruction. This likely indicates a write access violation, which means the attacker may control write address and/or value. However, it there is a chance it could be a NULL dereference."),
    ("NOT_EXPLOITABLE", "SourceAvNearNull", "Access violation near NULL on source operand", "The target crashed on an access violation at an address matching the source operand of the current instruction. This likely indicates a read access violation, which may mean the application crashed on a simple NULL dereference to data structure that has no immediate effect on control of the processor."),
    ("PROBABLY_EXPLOITABLE", "StackGuard", "Stack buffer overflow", "The target program is aborted due to stack cookie overwrite."),
    ("NOT_EXPLOITABLE", "SafeFunctionCheck", "Safe function check guard", "The target program is aborted due to safe function check guard: _chk()."),
    ("PROBABLY_EXPLOITABLE", "HeapError", "Heap error", "The target program is aborted due to error produced by heap allocator functions."),
    ("NOT_EXPLOITABLE", "FPE", "Arithmetic exception", "The target crashed due to arithmetic floating point exception."),
    ("NOT_EXPLOITABLE", "StackOverflow", "Stack overflow", "The target crashed on an access violation where the faulting instruction's mnemonic and the stack pointer seem to indicate a stack overflow."),
    ("UNDEFINED", "Undefined", "Undefined class", "There is no execution class for this type of exception."),
    ("NOT_EXPLOITABLE", "double-free", "Deallocation of freed memory", "The target crashed while trying to deallocate already freed memory."),
    ("NOT_EXPLOITABLE", "bad-free", "Invalid memory deallocation", "The target crashed on attempting free on address which was not malloc()-ed."),
    ("NOT_EXPLOITABLE", "alloc-dealloc-mismatch", "Invalid use of alloc/dealloc functions", "Mismatch between allocation and deallocation APIs."),
    ("NOT_EXPLOITABLE", "unknown-crash", "Sanitizer check fail", "Invalid memory access."),
    ("NOT_EXPLOITABLE", "heap-buffer-overflow(read)", "Heap buffer overflow", "The target reads data past the end, or before the beginning, of the intended heap buffer."),
    ("PROBABLY_EXPLOITABLE", "heap-buffer-overflow", "Heap buffer overflow", "The target attempts to read or write data past the end, or before the beginning, of the intended heap buffer."),
    ("EXPLOITABLE", "heap-buffer-overflow(write)", "Heap buffer overflow", "The target writes data past the end, or before the beginning, of the intended heap buffer."),
    ("NOT_EXPLOITABLE", "global-buffer-overflow(read)", "Global buffer overflow", "The target reads data past the end, or before the beginning, of the intended global buffer."),
    ("PROBABLY_EXPLOITABLE", "global-buffer-overflow", "Global buffer overflow", "The target attempts to read or write data past the end, or before the beginning, of the intended global buffer."),
    ("EXPLOITABLE", "global-buffer-overflow(write)", "Global buffer overflow", "The target writes data past the end, or before the beginning, of the intended global buffer."),
    ("NOT_EXPLOITABLE", "stack-use-after-scope(read)", "Use of out-of-scope stack memory", "The target crashed when reading from a stack address outside the lexical scope of a variable's lifetime."),
    ("PROBABLY_EXPLOITABLE", "stack-use-after-scope", "Use of out-of-scope stack memory", "The target crashed when using a stack address outside the lexical scope of a variable's lifetime."),
    ("EXPLOITABLE", "stack-use-after-scope(write)", "Use of out-of-scope stack memory", "The target crashed when writing on a stack address outside the lexical scope of a variable's lifetime."),
    ("PROBABLY_EXPLOITABLE", "use-after-poison", "Using poisoned memory", "The target crashed on trying to use the memory that was previously poisoned."),
    ("NOT_EXPLOITABLE", "stack-use-after-return(read)", "Use of stack memory after return", "The target crashed when reading from a stack memory of a returned function."),
    ("PROBABLY_EXPLOITABLE", "stack-use-after-return", "Use of stack memory after return", "The target crashed when using a stack memory of a returned function."),
    ("EXPLOITABLE", "stack-use-after-return(write)", "Use of stack memory after return", "The target crashed when writing to a stack memory of a returned function."),
    ("NOT_EXPLOITABLE", "stack-buffer-overflow(read)", "Stack buffer overflow", "The target reads data past the end, or before the beginning, of the intended stack buffer."),
    ("PROBABLY_EXPLOITABLE", "stack-buffer-overflow", "Stack buffer overflow", "The target attempts to read or write data past the end, or before the beginning, of the intended stack buffer."),
    ("EXPLOITABLE", "stack-buffer-overflow(write)", "Stack buffer overflow", "The target writes data past the end, or before the beginning, of the intended stack buffer."),
    ("NOT_EXPLOITABLE", "initialization-order-fiasco", "Bad initialization order", "Initializer for a global variable accesses dynamically initialized global from another translation unit, which is not yet initialized."),
    ("NOT_EXPLOITABLE", "stack-buffer-underflow(read)", "Stack buffer underflow", "The target reads from a buffer using buffer access mechanisms such as indexes or pointers that reference memory locations prior to the targeted buffer."),
    ("PROBABLY_EXPLOITABLE", "stack-buffer-underflow", "Stack buffer underflow", "The target is using buffer with an index or pointer that references a memory location prior to the beginning of the buffer."),
    ("EXPLOITABLE", "stack-buffer-underflow(write)", "Stack buffer underflow", "The target writes to a buffer using an index or pointer that references a memory location prior to the beginning of the buffer."),
    ("NOT_EXPLOITABLE", "heap-use-after-free(read)", "Use of deallocated memory", "The target crashed when reading from memory after it has been freed."),
    ("PROBABLY_EXPLOITABLE", "heap-use-after-free", "Use of deallocated memory", "The target crashed when using memory after it has been freed."),
    ("EXPLOITABLE", "heap-use-after-free(write)", "Use of deallocated memory", "The target crashed when writing to memory after it has been freed."),
    ("NOT_EXPLOITABLE", "container-overflow(read)", "Container overflow", "The target crashed when reading from memory inside the allocated heap region but outside of the current container bounds."),
    ("PROBABLY_EXPLOITABLE", "container-overflow", "Container overflow", "The target crashed when using memory inside the allocated heap region but outside of the current container bounds."),
    ("EXPLOITABLE", "container-overflow(write)", "Container overflow", "The target crashed when writing to memory inside the allocated heap region but outside of the current container bounds."),
    ("NOT_EXPLOITABLE", "new-delete-type-mismatch", "Invalid use of new/delete functions", "Deallocation size different from allocation size."),
    ("NOT_EXPLOITABLE", "bad-malloc_usable_size", "Bad function use", "Invalid argument to malloc_usable_size."),
    ("EXPLOITABLE", "param-overlap", "Overlapping memory ranges", "Call to function disallowing overlapping memory ranges."),
    ("PROBABLY_EXPLOITABLE", "negative-size-param", "Use of negative size", "Negative size used when accessing memory."),
    ("NOT_EXPLOITABLE", "odr-violation", "Multiple symbol definition", "Symbol defined in multiple translation units."),
    ("NOT_EXPLOITABLE", "memory-leaks", "Memory leaks", "The target does not sufficiently track and release allocated memory after it has been used, which slowly consumes remaining memory."),
    ("PROBABLY_EXPLOITABLE", "calloc-overflow", "Calloc parameters overflow", "Overflow in calloc parameters."),
    ("PROBABLY_EXPLOITABLE", "reallocarray-overflow", "Realloc parameters overflow", "Overflow in realloc parameters."),
    ("PROBABLY_EXPLOITABLE", "pvalloc-overflow", "Pvalloc parameters overflow", "Overflow in pvalloc parameters."),
    ("NOT_EXPLOITABLE", "invalid-allocation-alignment", "Invalid alignment", "Invalid allocation alignment."),
    ("NOT_EXPLOITABLE", "invalid-aligned-alloc-alignment", "Invalid alignment", "Invalid alignment requested in aligned_alloc."),
    ("NOT_EXPLOITABLE", "invalid-posix-memalign-alignment", "Invalid alignment", "Invalid alignment requested in posix_memalign."),
    ("NOT_EXPLOITABLE", "allocation-size-too-big", "Allocation size too big", "Requested allocation size exceeds maximum supported size."),
    ("NOT_EXPLOITABLE", "out-of-memory", "Memory limit exceeded", "The target has exceeded the memory limit."),
    ("NOT_EXPLOITABLE", "fuzz target exited", "Fuzz target exited", "Fuzz target exited."),
    ("NOT_EXPLOITABLE", "timeout", "Target timeout expired", "Timeout after several seconds."),
    ("PROBABLY_EXPLOITABLE", "overwrites-const-input", "Attempt to overwrite constant input", "Fuzz target overwrites its constant input."),
];

impl<'a> ExecutionClass<'a> {
    /// Construct `ExecutionClass` struct from tuple.
    ///
    /// # Arguments
    ///
    /// * `class` - tuple of strings represents execution class.
    pub fn new(class: (&'a str, &'a str, &'a str, &'a str)) -> Self {
        ExecutionClass {
            severity: Cow::Borrowed(class.0),
            short_description: Cow::Borrowed(class.1),
            description: Cow::Borrowed(class.2),
            explanation: Cow::Borrowed(class.3),
        }
    }

    /// Return `ExecutionClass` struct by short description.
    ///
    /// # Arguments
    ///
    /// * `short_desc` - short description of execution class.
    pub fn find(short_desc: &str) -> error::Result<Self> {
        for class in CLASSES.iter() {
            if class.1 == short_desc {
                return Ok(ExecutionClass::new(*class));
            }
        }
        Err(error::Error::Casr(format!(
            "Couldn't find class {} by name.",
            short_desc
        )))
    }

    /// Return `ExecutionClass` struct by short description and access information.
    ///
    /// # Arguments
    ///
    /// * `short_desc` - short description of execution class.
    ///
    /// * `rw` - access information.
    ///
    /// * `near_null` - is crash address near null
    pub fn san_find(
        short_desc: &'a str,
        rw: Option<&'a str>,
        near_null: bool,
    ) -> error::Result<Self> {
        match short_desc {
            "SEGV" => match (rw.unwrap_or("UNDEF"), near_null) {
                ("READ", false) => ExecutionClass::find("SourceAv"),
                ("READ", true) => ExecutionClass::find("SourceAvNearNull"),
                ("WRITE", false) => ExecutionClass::find("DestAv"),
                ("WRITE", true) => ExecutionClass::find("DestAvNearNull"),
                (_, _) => ExecutionClass::find("AccessViolation"),
            },
            "stack-overflow" => ExecutionClass::find("StackOverflow"),
            "deadly" => ExecutionClass::find("AbortSignal"), // hack: regexp matches word without spaces
            _ => {
                let pattern = match rw.unwrap_or("UNDEF") {
                    "READ" => format!("{}(read)", short_desc),
                    "WRITE" => format!("{}(write)", short_desc),
                    _ => short_desc.to_string(),
                };
                if let Ok(class) = ExecutionClass::find(&pattern) {
                    Ok(class)
                } else {
                    ExecutionClass::find(short_desc)
                }
            }
        }
    }
}
impl<'a> fmt::Display for ExecutionClass<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let explanation = if !self.explanation.is_empty() {
            format!("\nExplanation: {}", self.explanation)
        } else {
            "".to_string()
        };
        write!(
            f,
            "Severity: {}\nShort description: {}\nDescription: {}{}",
            self.severity, self.short_description, self.description, explanation
        )
    }
}
impl<'a> Default for ExecutionClass<'a> {
    fn default() -> Self {
        ExecutionClass {
            severity: Cow::Borrowed("UNDEFINED"),
            short_description: Cow::Borrowed("Undefined"),
            description: Cow::Borrowed("Undefined class"),
            explanation: Cow::Borrowed("The is no execution class for this type of exception"),
        }
    }
}
