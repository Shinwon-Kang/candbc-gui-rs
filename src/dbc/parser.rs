use nom::{
    bytes::complete::{is_a, is_not, tag, take, take_till, take_until, take_while}, character::complete::{alpha1, alphanumeric1, char, line_ending, space0, space1}, combinator::recognize, multi::{many0, separated_list0}, number::complete::be_u16, sequence::delimited, IResult
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
            id: 100,
            name: "DRIVER_HEARTBEAT".to_string(),
            dlc: 1,
            sender: "DRIVER".to_string(),
        };

        assert_eq!(parse_message(s).unwrap().1, message);
    }

    #[test]
    fn parse_signal_test() {
        let s = "SG_ Voltage_1_2_C : 0|8@1+ (0.392156863,0) [0|100.000000065] \"kph\" SENSOR,MOTOR,K16_BECM\n";

        let signal = Signal {
            name: "Voltage_1_2_C".to_string(),
            start_bit: 0,
            bit_size: 8,
            byte_ord: ByteOrder::LittleEddian,
            value_type: ValueType::UnsignedValue,
            scale: 0.392156863,
            offset: 0,
            min: 0.0,
            max: 100.000000065,
            unit: "kph".to_string(),
            receiver: vec!["SENSOR".to_string(), "MOTOR".to_string(), "K16_BECM".to_string()],
        };

        assert_eq!(parse_signal(s).unwrap().1, signal);
    }
}

fn parse_word(input: &str) -> IResult<&str, &str> {
    let (input, word) = take_while(|c: char| ((c.is_ascii_alphanumeric() || c == '_') && c != ':'))(input)?;

    Ok((input, word))
}

fn parse_double_quote_word(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("\"")(input)?;
    let (input, word) = take_till(|c| c == '\"')(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, word))
}

fn parse_version(input: &str) -> IResult<&str, Version> {
    let (input, _) = tag("VERSION")(input)?;
    let (input, _) = space0(input)?;
    let (input, version) = parse_double_quote_word(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Version(version.to_string())))
}

fn parse_indent_word(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, symbol) = parse_word(input)?;
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
    let (input, _) = tag("BS_:")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Option::None))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, _) = tag("BU_:")(input)?;
    let (input, _) = space0(input)?;
    
    let (input, nodes) = separated_list0(tag(" "), is_not(" \n"))(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, nodes.into_iter().map(|x| Node(x.to_string())).collect()))
}

fn parse_message(input: &str) -> IResult<&str, Message> {
    let (input, _) = tag("BO_ ")(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = parse_word(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, dlc) = nom::character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, sender) = parse_word(input)?;
    let (input, _) = line_ending(input)?;
    
    Ok((input, Message {
        id: id,
        name: name.to_string(),
        dlc: dlc,
        sender: sender.to_string(),
    }))
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    let (input, _) = tag("SG_ ")(input)?;
    let (input, name) = parse_word(input)?;
    let (input, _) = tag(" : ")(input)?;
    let (input, start_bit) = nom::character::complete::u32(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, bit_size) = nom::character::complete::u32(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, byte_ord) = nom::character::complete::u8(input)?;
    let (input, value_type) = nom::character::complete::anychar(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('(')(input)?;
    let (input, scale) = nom::number::complete::recognize_float(input)?;
    let (input, _) = char(',')(input)?;
    let (input, offset) = nom::character::complete::u32(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('[')(input)?;
    let (input, min) = nom::number::complete::recognize_float(input)?;
    let (input, _) = char('|')(input)?;
    let (input, max) = nom::number::complete::recognize_float(input)?;
    let (input, _) = char(']')(input)?;
    let (input, _) = space1(input)?;
    let (input, unit) = parse_double_quote_word(input)?;
    let (input, _) = space1(input)?;
    let (input, receiver) = separated_list0(char(','), parse_word)(input)?;
    
    Ok((input, Signal {
        name: name.to_string(),
        start_bit: start_bit,
        bit_size: bit_size,
        byte_ord: if byte_ord == 1 {ByteOrder::LittleEddian} else {ByteOrder::BigEndian},
        value_type: if value_type == '+' {ValueType::UnsignedValue} else {ValueType::SignedValue},
        scale: scale.parse().unwrap(),
        offset: offset,
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
        unit: unit.to_string(),
        receiver: receiver.into_iter().map(|x| x.to_string()).collect(),
    }))
}

// todo: tag -> char