use crate::{BitTiming, ByteOrder, Message, Node, Signal, Symbol, ValueType, Version};
use nom::{
    bytes::complete::{is_a, is_not, tag, take_till, take_while},
    character::complete::{line_ending, space1},
    sequence::delimited,
    IResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_test() {
        let s = "VERSION \"0.1\"\n";
        let version = Version("0.1".to_string());

        assert_eq!(parse_version(s).unwrap().1, version);
    }

    #[test]
    fn parse_new_symbol_test() {
        let s = r#"
NS_ :
    CM_
    BA_
    VAL_
    CAT_
        "#;
        let new_symbol = vec![
            Symbol("CM_".to_string()),
            Symbol("BA_".to_string()),
            Symbol("VAL_".to_string()),
            Symbol("CAT_".to_string()),
        ];

        assert_eq!(parse_new_symbol(s), new_symbol);
    }

    #[test]
    fn parse_bit_timing_test() {
        let s = "BS_:\n";

        let bit_timing = Option::None;

        assert_eq!(parse_bit_timing(s), bit_timing);
    }

    #[test]
    fn parse_nodes_test() {
        let s = "BU_: DBG DRIVER IO MOTOR SENSOR\n";

        let nodes = vec![
            Node("DBG".to_string()),
            Node("DRIVER".to_string()),
            Node("IO".to_string()),
            Node("MOTOR".to_string()),
            Node("SENSOR".to_string()),
        ];

        assert_eq!(parse_nodes(s), nodes);
    }

    #[test]
    fn parse_message_teset() {
        let s = "BO_ 100 DRIVER_HEARTBEAT: 1 DRIVER\n";

        let message = Message {
            id: 0,
            name: "DRIVER_HEARTBEAT".to_string(),
            dlc: 8,
            sender: "DRIVER".to_string(),
        };

        assert_eq!(parse_message(s), message);
    }

    #[test]
    fn parse_signal_test() {
        let s = "SG_ DRIVER_HEARTBEAT_cmd : 0|8@1+ (1,0) [0|0] \"kph\" SENSOR,MOTOR\n";

        let signal = Signal {
            name: "DRIVER_HEARTBEAT_cmd".to_string(),
            start_bit: 0,
            bit_size: 8,
            byte_ord: ByteOrder::LittleEddian,
            value_type: ValueType::UnsignedValue,
            scale: 1.0,
            offset: 0,
            min: 0,
            max: 0,
            unit: "kph".to_string(),
            receiver: vec!["SENSOR".to_string(), "MOTOR".to_string()],
        };

        assert_eq!(parse_signal(s), signal);
    }
}

fn parse_version(input: &str) -> IResult<&str, Version> {
    let (input, _) = tag("VERSION")(input)?;
    let (input, _) = tag(" \"")(input)?;
    let (input, version) = take_till(|c| c == '\"')(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Version(version.to_string())))
}

fn parse_new_symbol(s: &str) -> Vec<Symbol> {
    vec![Symbol("".to_string())]
}

fn parse_bit_timing(s: &str) -> Option<Vec<BitTiming>> {
    Option::Some(vec![BitTiming("".to_string())])
}

fn parse_nodes(s: &str) -> Vec<Node> {
    vec![Node("".to_string())]
}

fn parse_message(s: &str) -> Message {
    Message {
        id: 0,
        name: "".to_string(),
        dlc: 0,
        sender: "".to_string(),
    }
}

fn parse_signal(S: &str) -> Signal {
    Signal {
        name: "".to_string(),
        start_bit: 0,
        bit_size: 0,
        byte_ord: ByteOrder::LittleEddian,
        value_type: ValueType::UnsignedValue,
        scale: 0.0,
        offset: 0,
        min: 0,
        max: 0,
        unit: "".to_string(),
        receiver: vec!["".to_string()],
    }
}
