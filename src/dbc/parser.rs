use nom::{
    bytes::complete::{is_a, is_not, tag, take_till, take_until, take_while}, character::complete::{alpha1, line_ending, space0, space1}, multi::{many0, separated_list0}, sequence::delimited, IResult
};

use super::dbc::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_test() {
        let s: &str = "VERSION \"0.1\"\n";
        let version = Version("0.1".to_string());

        assert_eq!(parse_version(s).unwrap().1, version);
    }

    #[test]
    fn parse_indent_word_test() {
        let s = "\tCM_\n";

        assert_eq!(parse_indent_word(s).unwrap().1, "CM_");
    }

    #[test]
    fn parse_new_symbol_test() {
        let s = r#"NS_ :
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

        assert_eq!(parse_new_symbol(s).unwrap().1, new_symbol);
    }

    #[test]
    fn parse_bit_timing_test() {
        let s = "BS_:\n";

        let bit_timing = Option::None;

        assert_eq!(parse_bit_timing(s).unwrap().1, bit_timing);
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
            
            assert_eq!(parse_nodes(s).unwrap().1, nodes);
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
    let (input, _) = space0(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, version) = take_till(|c| c == '\"')(input)?;
    let (input, _) = tag("\"")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Version(version.to_string())))
}

fn parse_indent_word(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, symbol) = take_while(|c: char| (c.is_alphabetic() || c.is_ascii_punctuation()))(input)?;
    let (input, _) = line_ending(input)?;
    
    Ok((input, symbol))
}

fn parse_new_symbol(input: &str) -> IResult<&str, Vec<Symbol>> {
    let (input, _) = tag("NS_ :")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;
    let (input, symbols) = many0(parse_indent_word)(input)?;

    Ok((input, symbols.into_iter().map(|x| Symbol(x.to_string())).collect()))
}

fn parse_bit_timing(input: &str) -> IResult<&str, Option<Vec<BitTiming>>> {
    let (input, _) = tag("BS_")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Option::None))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, _) = tag("BU_")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    
    let (input, nodes) = separated_list0(tag(" "), is_not(" \n"))(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, nodes.into_iter().map(|x| Node(x.to_string())).collect()))
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
