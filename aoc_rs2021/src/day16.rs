struct ParseResult {
    length: usize,
    version_sum: u64,
    result: u64
}

pub fn compute(input: &str) -> (u64, u64) {

    let mut bits = String::new();

    input.chars().for_each(|c| {
        bits.push_str(&hex_to_bin(c));
    });

    let packet = parse_packet(&bits);

    (packet.version_sum, packet.result)
}

fn parse_packet(stream: &str) -> ParseResult{
    let mut version_sum = bin_to_dec(&stream[0..3]);
    let type_id = bin_to_dec(&stream[3..6]);
    let mut length = 6;

    if type_id == 4 {
        let mut contents = String::new();

        loop {
            contents.push_str(&stream[length+1..length+5]);

            if &stream[length..length+1] == "0" {
                break;
            }

            length += 5;
        }

        return ParseResult {
            length: length + 5,
            version_sum: version_sum,
            result: bin_to_dec(&contents)
        };
    }

    let mut results = Vec::<u64>::new();

    if &stream[6..7] == "1" {
        let num_packets = bin_to_dec(&stream[7..18]);
        length = 18;

        for _ in 0..num_packets {
            let packet = parse_packet(&stream[length..]);
            length += packet.length;
            version_sum += packet.version_sum;
            results.push(packet.result);
        }
    } else {
        let sub_length = bin_to_dec(&stream[7..22]);
        length = 22;
        while length != 22 + sub_length as usize {
            let packet = parse_packet(&stream[length..]);
            length += packet.length;
            version_sum += packet.version_sum;
            results.push(packet.result);
        }
    }

    let final_result = match type_id {
        0 => results.iter().sum(),
        1 => results.iter().product(),
        2 => *results.iter().min().unwrap(),
        3 => *results.iter().max().unwrap(),
        5 => (results[0] > results[1]) as u64,
        6 => (results[0] < results[1]) as u64,
        7 => (results[0] == results[1]) as u64,
        _ => panic!("Not possible"),
    };

    ParseResult{ length: length, version_sum: version_sum, result: final_result}
}


fn bin_to_dec(string: &str) -> u64 {
    u64::from_str_radix(string, 2).unwrap()
}

fn hex_to_bin(c: char) -> String {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
            _  => "",
    }.to_string()
}