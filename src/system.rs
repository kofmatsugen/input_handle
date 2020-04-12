use crate::traits::InputParser;
use amethyst::{
    ecs::{Read, System, SystemData, World, Write, WriteExpect},
    input::InputHandler,
    shrev::EventChannel,
    utils::circular_buffer::CircularBuffer,
};

use std::marker::PhantomData;

pub struct InputHandleSystem<I> {
    _marker: PhantomData<I>,
}

impl<I> InputHandleSystem<I> {
    pub fn new() -> Self {
        InputHandleSystem {
            _marker: PhantomData,
        }
    }
}

impl<'s, I> System<'s> for InputHandleSystem<I>
where
    I: InputParser<'s>,
    I::Event: std::fmt::Debug,
{
    type SystemData = (
        Read<'s, InputHandler<I::BindingTypes>>,
        Write<'s, EventChannel<I::Event>>,
        Write<'s, I::InputSignal>,
        WriteExpect<'s, CircularBuffer<I::InputSignal>>,
        I::SystemData,
    );

    fn run(
        &mut self,
        (input, mut events, mut now_signal, mut input_buffer, system): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        thread_profiler::profile_scope!("input-handle");

        let prev = input_buffer.queue().iter().last();
        let current = I::add_buffer(&input, prev);
        input_buffer.push(current.clone());

        let parsed = I::parse_input(&input_buffer, system);

        if parsed.len() > 0 {
            log::info!("command detect: {:?}", parsed);
        }

        events.iter_write(parsed);

        *now_signal = current;
    }

    fn setup(&mut self, world: &mut World) {
        world.insert(CircularBuffer::<I::InputSignal>::new(I::BUFFER_SIZE));
        Self::SystemData::setup(world);
    }
}
