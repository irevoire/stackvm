#-------------------------------------------------
# Instructions of a stackvm
#-------------------------------------------------
class Inst:
    """Base class for all StackVM instructions."""
    pass

class AddInst(Inst):
    """Pop top two values from the stack, push their sum.

    Note
    ----
    result = a + b where a = stack[-2], b = stack[-1]
    """
    def __str__(self):
        return "add"

class MulInst(Inst):
    """Pop top two values from the stack, push their product"""
    def __str__(self):
        return "mul"

class EQInst(Inst):
    """Pop top two values from the stack, push their product

    Note
    ----
    result = a == b where a = stack[-2], b = stack[-1]
    """
    def __str__(self):
        return "eq"

class VLoadInst(Inst):
    """Load variable from vmap to the top of the stack.

    Parameters
    ----------
    name : str
        The variable name.
    """
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return "vload %s" % self.name

class VStoreInst(Inst):
    """Pop the top of the stack and store to vmap.

    Parameters
    ----------
    name : str
        The variable name.
    """
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return "vstore %s" % self.name

class PushConstInst(Inst):
    """Push const value to the top of the stack.

    Parameters
    ----------
    value : int
        The variable name.
    """
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return "push_const %s" % str(self.value)

class CondJumpInst(Inst):
    """
    pc = pc + offset if the value on the top of the stack is true.
    otherwise pc = pc + 1

    Parameters
    ----------
    offset: int
        Relative offset in the PC.
    """
    def __init__(self, offset):
        self.offset = offset

    def __str__(self):
        return "cond_jump %s" % str(self.offset)

class JumpInst(Inst):
    """
    pc = pc + offset

    Parameters
    ----------
    offset: int
        Relative offset in the PC.
    """
    def __init__(self, offset):
        self.offset = offset

    def __str__(self):
        return "jump %s" % str(self.offset)

#-------------------------------------------------
# High level program AST nodes.
#-------------------------------------------------
class Expr:
    """Base class for all expressions."""
    def __add__(self, other):
        return AddExpr(self, other)

    def __mul__(self, other):
        return MulExpr(self, other)

    def __eq__(self, other):
        return EQExpr(self, other)


class Var(Expr):
    """A variable.

    Parameters
    ----------
    name : str
        The name of the variable.1

    Note
    ----
    A variable is uniquely indexed by a name.
    """
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return self.name

class ConstExpr(Expr):
    """A constant value.

    Parameters
    ----------
    value : int
        The value.
    """
    def __init__(self, value):
        self.value = value

    def __str__(self):
        return str(self.value)

class AddExpr(Expr):
    """AST Node of a + b

    Parameters
    ----------
    a : Expr
        The left operand

    b : Expr
        The right operand
    """
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def __str__(self):
        return "(" + str(self.a) + " + " + str(self.b) + ")"


class MulExpr(Expr):
    """AST Node of a * b

    Parameters
    ----------
    a : Expr
        The left operand

    b : Expr
        The right operand
    """
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def __str__(self):
        return "(" + str(self.a) + " * " + str(self.b) + ")"


class EQExpr(Expr):
    """AST Node of a == b

    Parameters
    ----------
    a : Expr
        The left operand

    b : Expr
        The right operand
    """
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def __str__(self):
        return "(" + str(self.a) + " == " + str(self.b) + ")"


class Stmt:
    """Base class for all statements."""
    pass


class Assign(Stmt):
    """Asssign statement var = expr

    Parameters
    ----------
    var_name : str
        The name of the variable to be assigned.

    expr : Expr
        The expression to be assigned
    """
    def __init__(self, var_name, expr):
        self.var_name = var_name
        self.expr = expr

    def __str__(self, indent=0):
        return " " * indent + "%s := %s" % (self.var_name, str(self.expr))

class Seq(Stmt):
    """Execute sequence of statements in order.

    Parameters
    ----------
    seq_list : List[Stmt]
    """
    def __init__(self, seq_list):
        self.seq_list = seq_list

    def __str__(self, indent=0):
        res = []
        for stmt in self.seq_list:
            res.append(stmt.__str__(indent))
        return "\n".join(res)


class For(Stmt):
    """A for statement

    for (loop_var = 0; loop_var < extent; ++loop_var) {
       body
    }

    Parameters
    ----------
    loop_var : str
        The loop variable.

    Parameters
    ----------
    loop_var : str
        The loop variable.

    begin : Expr
        The beginning of the loop variable.

    end : Expr
        The end of the loop,

    """
    def __init__(self, loop_var, extent, body):
        self.loop_var = loop_var
        self.extent = extent
        self.body = body

    def __str__(self, indent=0):
        prefix = " " * indent
        res = prefix +" for %s := 0 to %s {\n" % (self.loop_var, self.extent)
        res += self.body.__str__(indent + 2) + "\n}"
        return res

#-------------------------------------------------
# Helper programs
#-------------------------------------------------
def run_stackvm(program, vmap, debug_print=False):
    """Simple executor to execute a stackvm program

    Parameters
    ----------
    program : List[Inst]
        A stackvm program

    vmap : Dict[Str, int]
        Dictionary storing variable bindings.

    debug_print : bool
        Print each instrunction
    """
    pc = 0
    stack = []

    while pc < len(program):
        inst = program[pc]
        if debug_print:
            print("pc=%d\tinst=%s\tvmap=%s" % (pc, str(inst), str(vmap)))
        if isinstance(inst, VLoadInst):
            stack.append(vmap[inst.name])
            pc = pc + 1
        elif isinstance(inst, VStoreInst):
            value = stack.pop()
            vmap[inst.name] = value
            pc = pc + 1
        elif isinstance(inst, PushConstInst):
            stack.append(inst.value)
            pc = pc + 1
        elif isinstance(inst, AddInst):
            b = stack.pop()
            a = stack.pop()
            stack.append(a + b)
            pc = pc + 1
        elif isinstance(inst, MulInst):
            b = stack.pop()
            a = stack.pop()
            stack.append(a * b)
            pc = pc + 1
        elif isinstance(inst, EQInst):
            b = stack.pop()
            a = stack.pop()
            stack.append(a == b)
            pc = pc + 1
        elif isinstance(inst, CondJumpInst):
            cond = stack.pop()
            if cond:
                pc = pc + inst.offset
            else:
                pc = pc + 1
        elif isinstance(inst, JumpInst):
            pc = pc + inst.offset
        else:
            raise RuntimeError("Unknown instruction %s" % type(inst))

#-------------------------------------------------
# Examples for your reference.
#-------------------------------------------------
def example_seq():
    """Example function

    This function provides an statement and
    the corresponding manually written stackvm instructions.
    """
    ## Construct example statement.
    x = Var("x")
    y = Var("y")
    z = Var("z")
    one = ConstExpr(1)
    stmt = Seq([
        Assign("z", (x + y) * y),
        Assign("z", z + one),
        ])

    print(stmt)
    # The corresponding stackvm instructions.
    program = [
        VLoadInst("x"),
        VLoadInst("y"),
        AddInst(),
        VLoadInst("y"),
        MulInst(),
        VStoreInst("z"),
        VLoadInst("z"),
        PushConstInst(1),
        AddInst(),
        VStoreInst("z"),
        ]
    # Execute the program
    vmap = {"x": 2, "y": 3}
    run_stackvm(program, vmap)
    print(vmap)


def example_for():
    """Example function

    This function provides an statement and
    the corresponding manually written stackvm instructions.
    """
    ## Construct example statement.
    extent = 4
    x = Var("x")
    i = Var("i")
    stmt = For("i", ConstExpr(extent), Assign("x", x + i))
    print("YOUR VM")
    print(stmt)
    # The corresponding stackvm instructions.

    program = [
        PushConstInst(0),
        VStoreInst("i"),
        VLoadInst("i"),
        PushConstInst(extent),
        EQInst(),
        CondJumpInst(10),
        VLoadInst("x"),
        VLoadInst("i"),
        AddInst(),
        VStoreInst("x"),
        VLoadInst("i"),
        PushConstInst(1),
        AddInst(),
        VStoreInst("i"),
        JumpInst(-12),
        ]

    # Execute the program
    vmap = {"x": 1}
    run_stackvm(program, vmap)
    print("THE VM")
    print(vmap)

#-------------------------------------------------
# Task
#-------------------------------------------------
def compile_to_stackvm(stmt):
    """Compile a statement to stackvm.

    Parameters
    ----------
    stmt : The statement to be compiled.

    Returns
    -------
    program : List[Inst]
        The stack vm instructions.
    """
    program = []
    # TODO: implement me,
    return program


if __name__ == "__main__":
    example_seq()
    example_for()


