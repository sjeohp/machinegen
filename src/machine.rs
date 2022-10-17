use indices::*;
use itertools::Itertools;
use nalgebra::{dmatrix, DMatrix};
use nalgebra_sparse::{CooMatrix, CsrMatrix};
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug)]
enum Action {
    Right,
    Left,
    Stay,
    Write(u32),
}

pub type Priority = (u32, u32);
pub type Transition = ((u32, u32, u32), (u32, Action, Action), Priority);

#[derive(Clone, Debug)]
struct Machine {
    transitions: Vec<Transition>,
    program: Vec<u32>,
    memory: Vec<u32>,
    nstates: u32,
    nprogstates: u32,
    nmemstates: u32,
    state: u32,
    program_index: usize,
    memory_index: usize,
}

impl Machine {
    fn update(&mut self) -> bool {
        if self.state == self.nstates - 1 {
            // halt
            return true;
        }

        let key = (
            self.state,
            self.program[self.program_index],
            self.memory[self.memory_index],
        );

        let nstates = self.nstates;
        let nprogstates = self.nprogstates;
        let nmemstates = self.nmemstates;

        let t = self
            .transitions
            .iter()
            .find(|&&(k, _, _)| {
                k == key
                    || (k.0 == key.0 && k.1 == nprogstates && k.2 == nmemstates)
                    || (k.0 == nstates && k.1 == key.1 && k.2 == nmemstates)
                    || (k.0 == nstates && k.1 == nprogstates && k.2 == key.2)
                    || (k.0 == key.0 && k.1 == key.1 && k.2 == nmemstates)
                    || (k.0 == key.0 && k.1 == nprogstates && k.2 == key.2)
                    || (k.0 == nstates && k.1 == key.1 && k.2 == key.2)
            })
            .expect("transitions must cover all keys");

        let (newstate, prog_action, mem_action) = t.1;

        match prog_action {
            Action::Right => {
                self.program_index = (self.program_index + 1) % self.program.len();
            }
            Action::Left => {
                if self.program_index == 0 {
                    self.program_index = self.program.len() - 1;
                } else {
                    self.program_index -= 1;
            }
            Action::Stay => {}
            Action::Write(w) => {
                self.program[self.program_index] = w;
            }
        }

        match mem_action {
            Action::Right => {
                self.memory_index = (self.memory_index + 1) % self.memory.len();
            }
            Action::Left => {
                if self.memory_index == 0 {
                    self.memory_index = self.memory.len() - 1;
                } else {
                    self.memory_index -= 1;
                }
            }
            Action::Stay => {}
            Action::Write(w) => {
                self.memory[self.memory_index] = w;
            }
        }

        self.state = newstate;

        false
    }

    fn gen_random(
        ntransitions: usize,
        program_size: usize,
        memory_size: usize,
        nstates: u32,
        nprogstates: u32,
        nmemstates: u32,
    ) -> Self {
        let mut rng = thread_rng();
        let mut second_priorities = (0..nstates * nprogstates * nmemstates).collect::<Vec<u32>>();
        second_priorities.shuffle(&mut rng);
        let nsym = nstates + nprogstates + nmemstates;

        let base_transitions = (0..nsym)
            .map(|i| {
                let (key, priority) = if i < nstates {
                    let key = (i, nprogstates, nmemstates);
                    let priority1 = nprogstates * nmemstates;
                    (key, (priority1, second_priorities.pop().expect("oops")))
                } else if i < nstates + nprogstates {
                    let key = (nstates, i - nstates, nmemstates);
                    let priority1 = nstates * nmemstates;
                    (key, (priority1, second_priorities.pop().expect("oops")))
                } else {
                    let key = (nstates, nprogstates, i - (nstates + nprogstates));
                    let priority1 = nstates * nprogstates;
                    (key, (priority1, second_priorities.pop().expect("oops")))
                };
                let t = (
                    rng.gen_range(0..nstates),
                    match rng.gen_range(0..3) {
                        0 => Action::Right,
                        1 => Action::Left,
                        2 => Action::Stay,
                        _ => panic!("out of range"),
                    },
                    match rng.gen_range(0..4) {
                        0 => Action::Right,
                        1 => Action::Left,
                        2 => Action::Stay,
                        3 => Action::Write(rng.gen_range(0..nmemstates)),
                        _ => panic!("out of range"),
                    },
                );
                (key, t, priority)
            })
            .collect::<Vec<Transition>>();

        let specific_transitions = (0..ntransitions)
            .map(|_| {
                (
                    (
                        rng.gen_range(0..nstates),
                        rng.gen_range(0..nprogstates),
                        rng.gen_range(0..nmemstates),
                    ),
                    (
                        rng.gen_range(0..nstates),
                        match rng.gen_range(0..3) {
                            0 => Action::Right,
                            1 => Action::Left,
                            2 => Action::Stay,
                            _ => panic!("out of range"),
                        },
                        match rng.gen_range(0..4) {
                            0 => Action::Right,
                            1 => Action::Left,
                            2 => Action::Stay,
                            3 => Action::Write(rng.gen_range(0..nmemstates)),
                            _ => panic!("out of range"),
                        },
                    ),
                    (0, 0),
                )
            })
            .collect::<Vec<Transition>>();
        let program = (0..program_size)
            .map(|_| rng.gen_range(0..nprogstates))
            .collect::<Vec<u32>>();
        let memory = vec![0; memory_size];
        let mut transitions =
            Vec::with_capacity(base_transitions.len() + specific_transitions.len());
        transitions.extend_from_slice(&specific_transitions);
        transitions.extend_from_slice(&base_transitions);
        transitions.sort_by(
            |(_, _, (p1, p2)), (_, _, (q1, q2))| if p1 == p2 { p2.cmp(q2) } else { p1.cmp(p2) },
        );
        Self {
            transitions,
            program,
            memory,
            nstates,
            nprogstates,
            nmemstates,
            state: 0,
            program_index: 0,
            memory_index: 0,
        }
    }

    // we know which state each transition leaves but if an action moves an index we do not know
    // which state it enters so can only approach it probabilistically
    fn transition_matrix(&self) -> DMatrix<f32> {
        let idc = indices(&[
            self.nstates as usize,
            self.nprogstates as usize,
            self.nmemstates as usize,
        ]);

        let mut irows = vec![];
        let mut icols = vec![];
        let mut xvals = vec![];

        for &((state, prog, mem), (newstate, prog_action, mem_action), _) in
            self.transitions.iter().filter(|((state, prog, mem), _, _)| {
                state < &self.nstates && prog < &self.nprogstates && mem < &self.nmemstates
            })
        {
            let y = vec![state as usize, prog as usize, mem as usize];
            let idx = idc.iter().position(|x| x == &y).expect("must be found");

            let mut v = vec![];

            let p = match prog_action {
                Action::Right => {
                    // misses last index.
                    self.program
                        .as_slice()
                        .windows(2)
                        .filter(|&w| w[0] == prog)
                        .map(|w| w[1])
                        .sorted()
                        .dedup_with_count()
                        .collect::<Vec<(usize, u32)>>()
                }
                Action::Left => {
                    // misses index 0. is ok.
                    self.program
                        .as_slice()
                        .windows(2)
                        .filter(|&w| w[1] == mem)
                        .map(|w| w[0])
                        .sorted()
                        .dedup_with_count()
                        .collect::<Vec<(usize, u32)>>()
                }
                Action::Stay => {
                    vec![(1, self.program[self.program_index])]
                }
                _ => panic!("cannot write to program"),
            };

            let m = match mem_action {
                Action::Right => {
                    // misses last index.
                    self.memory
                        .as_slice()
                        .windows(2)
                        .filter(|&w| w[0] == prog)
                        .map(|w| w[1])
                        .sorted()
                        .dedup_with_count()
                        .collect::<Vec<(usize, u32)>>()
                }
                Action::Left => {
                    // misses index 0. is ok.
                    self.memory
                        .as_slice()
                        .windows(2)
                        .filter(|&w| w[1] == mem)
                        .map(|w| w[0])
                        .sorted()
                        .dedup_with_count()
                        .collect::<Vec<(usize, u32)>>()
                }
                Action::Stay => {
                    vec![(1, self.memory[self.memory_index])]
                }
                Action::Write(w) => {
                    vec![(1, w)]
                }
            };

            let psum = p.iter().map(|(c, _)| c).sum::<usize>();
            let msum = m.iter().map(|(c, _)| c).sum::<usize>();
            let prodsum = psum * msum;
            // icol and irow for every m and p
            for ((pcount, newprog), (mcount, newmem)) in p.iter().cartesian_product(m.iter()) {
                irows.push(idx);
                let to = vec![newstate as usize, *newprog as usize, *newmem as usize];
                let to_idx = idc.iter().position(|x| x == &to).expect("must be found");
                icols.push(to_idx);
                xvals.push(mcount * pcount / prodsum); // product of m and p probabilities
            }

            v.push(((state, prog, mem), (newstate, p, m)));
        }

        let nrows = (self.nstates * self.nprogstates * self.nmemstates) as usize;
        let ncols = nrows;
        let mut coo = CooMatrix::try_from_triplets(nrows, ncols, irows, icols, xvals).expect("construct matrix");
        let mut csr = CsrMatrix::from(&coo);
        dbg!(csr.eigenvalues());
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let ntransitions = 50;
        let program_size = 100;
        let memory_size = 100;
        let nstates = 5;
        let nprogstates = 2;
        let nmemstates = 2;
        let mut m = Machine::gen_random(
            ntransitions,
            program_size,
            memory_size,
            nstates,
            nprogstates,
            nmemstates,
        );
        let nsteps = 1000;
        let mut count = 0;
        for _ in 0..nsteps {
            count += 1;
            if m.update() {
                break;
            }
        }
        dbg!(count);
        assert!(true);
    }

    #[test]
    fn transition_matrix() {
        let ntransitions = 20;
        let program_size = 100;
        let memory_size = 100;
        let nstates = 5;
        let nprogstates = 2;
        let nmemstates = 2;
        let mut m = Machine::gen_random(
            ntransitions,
            program_size,
            memory_size,
            nstates,
            nprogstates,
            nmemstates,
        );
        dbg!(m.transition_matrix());
        assert!(true);
    }
}
