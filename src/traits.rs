use amethyst::{
    input::{BindingTypes, InputHandler},
    utils::circular_buffer::CircularBuffer,
};

pub trait InputParser {
    const BUFFER_SIZE: usize; // 入力を覚えるF数
    type BindingTypes: BindingTypes; // 入力キー
    type InputSignal; // バッファに詰める入力
    type Event; // バッファに詰めた入力から生成された実際に各エンティティにくばるイベント

    // 入力を確認してバッファに信号を生成する
    fn add_buffer(handler: &InputHandler<Self::BindingTypes>) -> Self::InputSignal;

    // バッファの信号をパースして処理するためのイベントを生成する
    // 格ゲーのコマンド入力とかに使う
    fn parse_input(buffer: &mut CircularBuffer<Self::InputSignal>) -> Option<Self::Event>;
}
