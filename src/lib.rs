pub mod rock_paper_scissors;

use std::marker::PhantomData;

pub struct FiniteStateMachine<I, O, S, F, G> {
    _inputs: PhantomData<I>,
    _outputs: PhantomData<O>,
    current_state: S,
    next_state_function: F,
    output_function: G,
}

impl<I, O, S, F, G> FiniteStateMachine<I, O, S, F, G>
where
    S: Default,
    F: FnMut(&S, &I) -> S,
    G: FnMut(&S, &I) -> O,
{
    pub fn new(next_state_function: F, output_function: G) -> Self {
        Self {
            _inputs: PhantomData,
            _outputs: PhantomData,
            current_state: S::default(),
            next_state_function,
            output_function,
        }
    }
    pub fn transition(&mut self, input: &I) -> O {
        let next_state = (self.next_state_function)(&self.current_state, input);
        let output = (self.output_function)(&next_state, input);

        self.current_state = next_state;
        return output;
    }
}
