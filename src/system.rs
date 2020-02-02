use crate::traits::InputParser;
use amethyst::{
    ecs::{Read, System},
    input::InputHandler,
    utils::circular_buffer::CircularBuffer,
};

use std::marker::PhantomData;

pub struct InputHandleSystem<I>
where
    I: InputParser,
{
    _marker: PhantomData<I>,
    input_buffer: CircularBuffer<I::InputSignal>,
}

impl<I> InputHandleSystem<I>
where
    I: InputParser,
{
    pub fn new() -> Self {
        InputHandleSystem {
            _marker: PhantomData,
            input_buffer: CircularBuffer::new(I::BUFFER_SIZE),
        }
    }
}

impl<'s, I> System<'s> for InputHandleSystem<I>
where
    I: InputParser,
{
    type SystemData = (Read<'s, InputHandler<I::BindingTypes>>,);

    fn run(&mut self, (input,): Self::SystemData) {
        self.input_buffer.push(I::add_buffer(&input));
    }
}
