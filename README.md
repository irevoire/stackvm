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
