use amethyst::{
    ecs::SystemData,
    input::{BindingTypes, InputHandler},
    utils::circular_buffer::CircularBuffer,
};

pub trait InputParser {
    const BUFFER_SIZE: usize; // 入力を覚えるF数
    type BindingTypes: BindingTypes; // 入力キー
    type InputSignal: 'static + Sync + Send + Copy + Default; // バッファに詰める入力(ビットフラグなどでまとめる)
    type Event: 'static + Sync + Send; // バッファに詰めた入力から生成された実際に各エンティティにくばるイベント
    type SystemData: for<'s> SystemData<'s>;

    // 入力を確認してバッファに信号を生成する
    fn add_buffer(
        handler: &InputHandler<Self::BindingTypes>,
        prev_input: Option<Self::InputSignal>,
    ) -> Self::InputSignal;

    // バッファの信号をパースして処理するためのイベントを生成する
    // 格ゲーのコマンド入力とかに使う
    fn parse_input(
        buffer: &CircularBuffer<Self::InputSignal>,
        system: &mut Self::SystemData,
    ) -> Option<Self::Event>;
}
