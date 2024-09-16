# What's on the Stack ?

What is a two-dimensional stack? The rust_twostack crate offers the functionality of a two-dimensional stack structure for the Rust programming language. This data structure can contain dynamically typed values provided by the rust_dynamic crate. While the concepts of Last-In-First-Out (LIFO) and First-In-First-Out (FIFO) stacks are likely familiar to you, serving as data structures designed to store and extract values based on specific application logic, it is important to consider the potential limitations of this well-established concept.

rust_twostack offers a data structure that holds a stack of stacks of values. A single stack may not provide the necessary protections when performing complex stack-based computations:

Data isolation: When consolidating data from multiple computational steps onto a single stack, it is imperative to carefully consider the data boundaries to mitigate the risk of costly computation errors.

When setting up data processing pipelines, it is standard practice for an application to acquire and preprocess data before analyzing it. This process requires accessing diverse data sources and applying various preprocessing methods. Combining dissimilar data in the same pipeline is ill-advised and indicative of subpar design.

Hence arises the concept of the Stack-of-the-stacks, allowing you to organize data segments in individual stack spaces and process and merge them in a controlled manner.

## What are the properties of the Stack-of-the-stacks ?

* rust_twostack provides a structure called TS that controls the deque-based buffer of the stack structures
* You can rotate, create new stacks, access the current stack, and remove stacks from the Stack-of-the-stacks
* Stacks could be either anonymous or named. You can rotate Stack-of-the-stacks and position a stack based on its name to become a current stack

## How you create Stack-of-the-stacks ?

```rust
// You call a ::new() function like this
let ts = TS::new();

```

## How you can control Stack-of-the-stacks ?

Here is the list of TS object methods that control the Stack-of-the-stacks.

| Function name | Description |
|---|---|
| TS::new() | Creates a new instance of the Stack-of-the-stacks. Adds one anonymous stack to serve as initial default stack |
| TS.clear() | First, removes all created stacks and then adds one anonymous stack to serve as initial default stack |
| TS.ensure() | If stack-of-stacks is empty, add a new anonymous stack |
| TS.len() | Return a number of stacks |
| TS.is_empty() | Return "true" if Stack-of-stacks is empty, "false" otherwise. Note, if you got "false", likely it is due to internal error and you have a corrupt structure. |
| TS.stack_len() | Return a number of values in current stack |
| TS.current() | Return a reference to a current stack that holds the data, or None. If this function return None, it is likely due to internal error and you are dealing with corrupted structure |
| TS.push() | Push data into current stack |
| TS.pull() | Pull and remove data from the current stack, returns data element or None if stack is empty |
| TS.add_stack() | Add new anonymous data stack and make it current |
| TS.add_named_stack() | Add new named data stack and make it current |
| TS.position() | Locate stack in the Stack-of-the-stacks by the name and make it current |
| TS.swap() | For the current stack that have to be at least 2 values deep, this method will swap last two elements of the stack |
| TS.dup() | Duplicate data value located on top of the stack |
| TS.drop() | Drop the top stack |
| TS.return_to() | Pull value from the current stack, drop stack and push the value to the new current |
| TS.pull_to() | Pull element from the previous stack and push to the current |
| TS.apply(Value, StackOp) | Apply element to the Stack and convert state of the stack to the element's attributes. Possible values of StackOp are: StackOp::None - add element w/o modifying stack state, StackOp::TakeOne - take one element from the stack and push to the beginning of the Value attributes, StackOp::TakeAll - take all elements from the stack and add the the beginning of the attributes |
| TS.eval() | Execute Value by CtxFun in the stack context |




## How you can control data stack ?

You can access a data stack by calling TS.current(). After that, you can control it directly by calling object methods.

| Function name | Description |
|---|---|
| Stack.len() | Return the number of data elements in the stack |
| Stack.is_empty()) | Return "true" if data stack is empty |
| Stack.peek() | Return the reference on current element in the data stack without removing it from the stack. If stack is empty, returns None |
| Stack.pull() | Remove and return the current element from the stack |
| Stack.push() | Push new data element to the stack |
| Stack.clear() | Remove all elements from the stack |
| Stack.pull() | Remove and return the current element from the stack |
| Stack.left() | Rotate data stack one position to the left |
| Stack.right() | Rotate data stack one position to the right |
