from enum import Enum
import random
import functools


class Action(Enum):
    Right = 1
    Left = 2
    Stay = 3
    Write = 4


class Machine:
    transitions = []
    program = []
    memory = []
    nstates = 0
    nprogstates = 0
    nmemstates = 0
    state = 0
    program_index = 0
    memory_index = 0

    def __init__(self, transitions, program, memory, nstates, nprogstates, nmemstates):
        self.transitions = transitions
        self.program = program
        self.memory = memory
        self.nstates = nstates
        self.nprogstates = nprogstates
        self.nmemstates = nmemstates
        self.state = 0
        self.program_index = 0
        self.memory_index = 0

    def update(self):
        if self.state == self.nstates - 1:
            # halt
            return True

        key = (
            self.state,
            self.program[self.program_index],
            self.memory[self.memory_index],
        )

        nstates = self.nstates
        nprogstates = self.nprogstates
        nmemstates = self.nmemstates

        def get_element(arr, compare):
            for el in arr:
                if compare(el[0]):
                    return el
            raise Exception("element must be found")

        t = get_element(
            self.transitions,
            (
                lambda k: k == key
                or (k[0] == key[0] and k[1] == nprogstates and k[2] == nmemstates)
                or (k[0] == nstates and k[1] == key[1] and k[2] == nmemstates)
                or (k[0] == nstates and k[1] == nprogstates and k[2] == key[2])
                or (k[0] == key[0] and k[1] == key[1] and k[2] == nmemstates)
                or (k[0] == key[0] and k[1] == nprogstates and k[2] == key[2])
                or (k[0] == nstates and k[1] == key[1] and k[2] == key[2])
            ),
        )

        (newstate, prog_action, (mem_action, mem_write)) = t[1]

        if prog_action == Action.Right:
            self.program_index = (self.program_index + 1) % len(self.program)
        elif prog_action == Action.Left:
            if self.program_index == 0:
                self.program_index = len(self.program) - 1
            else:
                self.program_index -= 1
        elif prog_action == Action.Stay:
            None
        else:
            raise Exception("cannot write to program")

        if mem_action == Action.Right:
            self.memory_index = (self.memory_index + 1) % len(self.memory)
        elif mem_action == Action.Left:
            if self.memory_index == 0:
                self.memory_index = len(self.memory) - 1
            else:
                self.memory_index -= 1
        elif mem_action == Action.Stay:
            None
        else:
            self.memory[self.memory_index] = mem_write

        self.state = newstate

        return False

    def gen_random(
        ntransitions, program_size, memory_size, nstates, nprogstates, nmemstates
    ):
        nsym = nstates * nprogstates * nmemstates
        second_priorities = list(range(0, nsym))
        random.shuffle(second_priorities)

        def gen_program_action():
            r = random.randrange(3)
            if r == 0:
                return Action.Right
            elif r == 1:
                return Action.Left
            else:
                return Action.Stay

        def gen_memory_action():
            r = random.randrange(4)
            if r == 0:
                return (Action.Right, nmemstates)
            elif r == 1:
                return (Action.Left, nmemstates)
            elif r == 2:
                return (Action.Stay, nmemstates)
            else:
                return (Action.Write, random.randrange(nmemstates))

        def gen_transition_output():
            return (
                random.randrange(nstates),
                gen_program_action(),
                gen_memory_action(),
            )

        def gen_base_transition(i):
            if i < nstates:
                key = (i, nprogstates, nmemstates)
                prior = nprogstates * nmemstates
                return (key, gen_transition_output(), (prior, second_priorities.pop()))
            elif i < nstates + nprogstates:
                key = (nstates, i - nstates, nmemstates)
                prior = nstates * nmemstates
                return (key, gen_transition_output(), (prior, second_priorities.pop()))
            else:
                key = (nstates, nprogstates, i - (nstates + nprogstates))
                prior = nstates * nprogstates
                return (key, gen_transition_output(), (prior, second_priorities.pop()))

        base_transitions = [gen_base_transition(i) for i in range(nsym)]

        def gen_specific_transition():
            return (
                (
                    random.randrange(nstates),
                    random.randrange(nprogstates),
                    random.randrange(nmemstates),
                ),
                (random.randrange(nstates), gen_program_action(), gen_memory_action()),
                (0, 0),
            )

        specific_transitions = [gen_specific_transition() for x in range(ntransitions)]

        transitions = sorted(
            specific_transitions + base_transitions,
            key=functools.cmp_to_key(
                lambda x, y: y[2][1] - x[2][1]
                if x[2][0] == y[2][0]
                else y[2][0] - x[2][0]
            ),
        )
        program = [random.randrange(nprogstates) for x in range(program_size)]
        memory = [0 for x in range(memory_size)]

        return Machine(transitions, program, memory, nstates, nprogstates, nmemstates)


if __name__ == "__main__":
    print("hello world")
    ntransitions = 50
    program_size = 100
    memory_size = 100
    nstates = 5
    nprogstates = 2
    nmemstates = 2
    m = Machine.gen_random(
        ntransitions, program_size, memory_size, nstates, nprogstates, nmemstates
    )
    nsteps = 1000
    count = 0
    for i in range(nsteps):
        count += 1
        if m.update():
            break
    print(count)
