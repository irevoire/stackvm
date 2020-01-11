## Problem 2: Stack Virtual Machine Compiler

In this problem, you are tasked to implement a compiler from a set of statements
to a simple stack virtual machine.

This stack virtual machine executes programs with a stack and a variable heap(vmap).
The instrunction set of the target stack virtual machine is specified by Inst in [stackvm](stackvm.py).

Our high level programs are a list of statements that computes expressions and assign them to variables.
The high level program AST nodes are specified by Expr and Stmt in [stackvm](stackvm.py).

We also provided two example manual correspondence of the high-level program and their stackvm counterparts.

## Task 2.1: Compilation
Your task is to implement a compilation function that translates a high-level program to the stackvm counterparts:)


## Task 2.2(optional): Open ended - Parallel Execution
What if we want to support a parallel version of for loop that does computation using multiple threads.
What additional extensions to the stack vm and high level programs are necessary?
You do not have to implement this feature, please provide an brief description of your proposed approach below.

-----------

### The problem

We’ll have a problem if we try to do this without adding anything to the VM.
Let’s look at the case of a `for` that uses an external variable like that:
```
a := 0; // outside of the for
for i in 2 {
	a := a + 1;
}
```

We’ll be in trouble if we try to execute each iteration of this `for`-loop on multiple
thread, two threads could be trying to update `a` at the same time.

For example, if two threads were running this loop:
- The first thread could pull the content of `a` -> 0 and compute `a + 1` -> 1.
- Before the first thread finishes his assignation, the second thread could pull the content of
  `a` -> 1 and compute (a + 1) -> 2.

And at this point if the first thread execute before the second we’ll end with:
- `a := 1`
- `a := 2`

Which is what we are expecting.
But if the second thread finishes before the first we’ll get:
- `a := 2`
- `a := 1`

And end the `for`-loop with the value `1` inside of `2`. This is obviously wrong.


**This is what we call a [race condition](https://en.wikipedia.org/wiki/Race_condition).**
In reality the race condition could happens in a bunch of places, like when accessing the hashmap
storing the variables or when accessing the stack.

### In our case

The problem described above would take place with a good number of languages (C, C++, java, go, ...).

In our case, I think it is important to notice that since we are using python, and python implements
a GIL ([Global Interpreter Lock](https://en.wikipedia.org/wiki/Global_interpreter_lock)) we can be
sure our program won’t panic when two threads try to access a variable. There will never be more
than one thread running at the same time.


In the case of rust, it is impossible to design a program containing a race condition without making
uses of `unsafe` bloc. The compiler will stop you from compiling a program with a possible
race condition.

### The solution

There are multiple solutions to this kind of problem but the easiest one would be to create a
`mutex` type with some new instruction.
What we call a `mutex` in computer science is something that only **one** process can take at the
same time. Once a process “take” it the other should wait to take it. Then it’ll need to release
the `mutex` for others processes to be able to “take” it.

Here is an example of code we could write with this:
```
a := mutex(0); // outside of the for
for i in 2 {
	with a {
		a := a + 1;
	}
}
```

Here you can see I used two new keywords:
- `mutex` holds the value we want to protect
- `with` allow you to access and modify the content of `a`.
  Once we go out of the `with`-bloc, `a` is “released”, and the next thread can take `a`.

To avoid having race-condition we would then need to block the assignation of variables that were not
initialized in the `for` apart from the “mutexed” ones. Although reading from a variable declared
outside of the `for` is ok.
And bloc the uses of a “mutexed” variables outside of `with`-bloc.

-----

In this case, obviously, our loop ends up being executed in sequence and not in parallel since we are
doing nothing else than working on the protected value.
But if we were doing a bunch of calculations before or after the `with`-bloc we would actually execute
a part of the loop at the same time on multiple threads. (once again not in python because of the GIL)

### The implementation of the for

The other part of the question is to provide instructions to create our beloved threads.
Here since our `for` loop is very limited and only run on a known range, if we assume that
the indices in the range cannot be modified inside of the for-loop (which is the case in a good
amount of languages).

We would need two instructions:
- `thread n` that generate multiple threads and provide a variable `id`
  containing the `id` of the thread (going from 0 to n).
- A `join` function waiting for all the threads to finish.

For example:
```
x := 0;
for i in 3 {
	x := x + 1;
}
x := 1;
```

Would generate the following instructions:

```
// ==--- x := 0 ---==

pushconst 0
vstore x



// ==--- for loop ---==

thread 4
pushconst id
vstore i	// each thread start at his index in the loop (0, 1, 2, 3)
vload i
pushconst 2
infeq		// this is a new instruction doing: a <= b with a and b on the top of the stack
condjump(10)	// at this point the threads with the id 3 and 4 need to stop executing

// all the next instruction are unchanged
vload x
vload i
add
vstore x

vload i
pushconst 4 	// we increment i by the number of thread
add

vstore i
jump -12

join // we finished the execution and want to wait for all thread to finishe before continuing



// ==--- x := 1 ---==
pushconst 1
vstore x
```
