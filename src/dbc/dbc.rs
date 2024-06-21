#[derive(Debug, PartialEq)]
pub enum ByteOrder {
    LittleEddian,
    BigEndian,
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    UnsignedValue,
    SignedValue,
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
    pub id: u32,
    pub name: String,
    pub dlc: u32,
    pub sender: String,
}

#[derive(Debug, PartialEq)]
pub struct Signal {
    pub name: String,
    pub start_bit: u32,
    pub bit_size: u32,
    pub byte_ord: ByteOrder,
    pub value_type: ValueType,
    pub scale: f64,
    pub offset: u32,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    pub receiver: Vec<String>,
}

pub struct DBC {
    version: Version,                   // VERSION      : DBC 파일 버전
    new_symbol: Vec<Symbol>,            // NS_          : DBC에서 사용하는 새로운 심볼 정의
    bit_timing: Option<Vec<BitTiming>>, // BS_          : CAN 네트워크의 비트 타이밍
    nodes: Vec<Node>,                   // BU_          : 네트워크에 연결된 모든 노드
    messages: Vec<Message>,             // BO_          : CAN 메시지에 대한 정보 정의
    signals: Vec<Signal>,               // SG_          : CAN 메시지 내의 개별 신호 정의
}

impl DBC {}
