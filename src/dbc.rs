pub mod dbc_parser;

#[derive(Debug, PartialEq)]
enum ByteOrder {
    LittleEddian,
    BigEndian,
}

#[derive(Debug, PartialEq)]
enum ValueType {
    UnsignedValue,
    SignedValue,
}

pub struct DBC {
    version: Version,                   // VERSION      : DBC 파일 버전
    new_symbol: Vec<Symbol>,            // NS_          : DBC에서 사용하는 새로운 심볼 정의
    bit_timing: Option<Vec<BitTiming>>, // BS_          : CAN 네트워크의 비트 타이밍
    nodes: Vec<Node>,                   // BU_          : 네트워크에 연결된 모든 노드
    messages: Vec<Message>,             // BO_          : CAN 메시지에 대한 정보 정의
    signals: Vec<Signal>,               // SG_          : CAN 메시지 내의 개별 신호 정의
}

#[derive(Debug, PartialEq)]
pub struct Version(pub String);

#[derive(Debug, PartialEq)]
pub struct Symbol(pub String);

#[derive(Debug, PartialEq)]
pub struct BitTiming(pub String);

#[derive(Debug, PartialEq)]
pub struct Node(pub String);

#[derive(Debug, PartialEq)]
pub struct Message {
    id: u16,
    name: String,
    dlc: u16,
    sender: String,
}

#[derive(Debug, PartialEq)]
pub struct Signal {
    name: String,
    start_bit: u16,
    bit_size: u16,
    byte_ord: ByteOrder,
    value_type: ValueType,
    scale: f32,
    offset: u16,
    min: u16,
    max: u16,
    unit: String,
    receiver: Vec<String>,
}
]