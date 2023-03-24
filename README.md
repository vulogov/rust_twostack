# What's on the Stack ?

What is a two-dimensional stack? rust_twostack crate provides the capability of a two-dimensional stack structure for Rust. This data structure can hold dynamically-typed values supplied by the rust_dynamic crate. You likely are very familiar with the LIFO or FIFO stack as a data structure designed to store the values that can be pushed to the front or back and extracted according to the application logic from LIFO or FIFO queues. But what is wrong with this time-tested idea?

rust_twostack provides a data structure that holds the stack of the stacks of Values. When you perform complicated stack-based computations, a single stack doesn't provide you with a few necessary protections:

Data isolation. When you place all your data related to multiple computational steps on a single stack, it is an elementary misstep to overlook the boundaries of the data, which could lead to very costly errors in computation.

Data separation. When you write data processing pipelines, your application commonly works when first obtaining and preprocessing the data, and then you run an analysis on prepared data. Those efforts require you to access different sources and utilize various preprocessing logic. Placing often non-related data into the same stack is a bad idea and an example of a  poor design.

Hence comes the idea of the Stack-of-the-stacks, where you can prepare data segments, each in individual stack space, and process and merge them in a controllable manner.

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
