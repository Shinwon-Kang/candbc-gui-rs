#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ByteOrder {
    LittleEddian,
    BigEndian,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    UnsignedValue,
    SignedValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Version(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct BitTiming(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Node(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub id: u32,
    pub name: String,
    pub dlc: u32,
    pub sender: String,
    pub signals: Option<Vec<Signal>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Multiplexer {
    None,
    Multiplexer(u32),
    Switch,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Signal {
    pub name: String,
    pub multiplexer: Multiplexer,
    pub start_bit: u32,
    pub bit_size: u32,
    pub byte_ord: ByteOrder,
    pub value_type: ValueType,
    pub scale: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    pub receiver: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comment {
    Normal(String),
    Node {
        name: String,
        comment: String,
    },
    Message {
        id: u32,
        comment: String,
    },
    Signal {
        id: u32,
        name: String,
        comment: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct DBC {
    pub version: Version,                   // VERSION      : DBC 파일 버전
    pub new_symbol: Vec<Symbol>,            // NS_          : DBC에서 사용하는 새로운 심볼 정의
    pub bit_timing: Option<Vec<BitTiming>>, // BS_          : CAN 네트워크의 비트 타이밍
    pub nodes: Vec<Node>,                   // BU_          : 네트워크에 연결된 모든 노드
    pub messages: Vec<Message>,             // BO_          : CAN 메시지에 대한 정보 정의
    pub comments: Vec<Comment>,
}
