use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};

#[derive(Debug, Eq)]
enum Huffman {
    Leaf { c: char, freq: u32 },
    Node { freq: u32, left: Box<Huffman>, right: Box<Huffman> },
}

impl Huffman {
    fn get_freq(&self) -> u32 {
        match self {
            Huffman::Leaf { freq, .. } => *freq,
            Huffman::Node { freq, .. } => *freq,
        }
    }

    fn get_char(&self) -> char {
        match self {
            Huffman::Leaf { c, .. } => *c,
            Huffman::Node { left, .. } => left.get_char(), // escolhe o primeiro caractere da subarvore
        }
    }
}

impl PartialEq for Huffman {
    fn eq(&self, other: &Self) -> bool {
        (self.get_freq(), self.get_char()) == (other.get_freq(), other.get_char())
    }
}

impl Ord for Huffman {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_freq().cmp(&other.get_freq()) {
            Ordering::Equal => self.get_char().cmp(&other.get_char()).reverse(),
            other => other.reverse(), // menor frequência tem maior prioridade no heap
        }
    }
}


impl PartialOrd for Huffman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_frequency_table(text: &str) -> HashMap<char, u32> {
    let mut freq = HashMap::new();
    for c in text.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}


fn build_huffman_tree(freq: &HashMap<char, u32>) -> Huffman {
    let mut heap: BinaryHeap<Huffman> = freq.iter()
        .map(|(&c, &f)| Huffman::Leaf { c, freq: f })
        .collect();
    // enquanto tem mais de um nó, combina dois nós menores.
    while heap.len() > 1 {
        let min1 = heap.pop().unwrap();
        let min2 = heap.pop().unwrap();
        let new_node = Huffman::Node {
            freq: min1.get_freq() + min2.get_freq(),
            left: Box::new(min1),
            right: Box::new(min2),
        };
        heap.push(new_node);
    }

    heap.pop().unwrap()
}


fn build_codes(tree: &Huffman, prefix: String, codes: &mut HashMap<char, String>) {
    match tree {
        Huffman::Leaf { c, .. } => {
            codes.insert(*c, prefix);
        }
        Huffman::Node { left, right, .. } => {
            build_codes(left, format!("{}0", prefix), codes);
            build_codes(right, format!("{}1", prefix), codes);
        }
    }
}


fn encode(text: &str, codes: &HashMap<char, String>) -> String {
    text.chars()
        .map(|c| codes.get(&c).unwrap().clone())
        .collect()
}


fn decode(bits: &str, tree: &Huffman) -> String {
    let mut result = String::new();
    let mut node = tree;

    for b in bits.chars() {
        node = match node {
            Huffman::Node { left, right, .. } => {
                if b == '0' { left } else { right }
            }
            Huffman::Leaf { c, .. } => {
                result.push(*c);
                node = tree;
                // reprocessa o bit atual (precisa voltar e processar ele no nó raiz)
                node = match node {
                    Huffman::Node { left, right, .. } => {
                        if b == '0' { left } else { right }
                    }
                    _ => unreachable!()
                };
                node
            }
        }
    }

    // caso o último nó seja folha, precisa adicionar o caractere.
    if let Huffman::Leaf { c, .. } = node {
        result.push(*c);
    }

    result
}


// função auxiliar: converte string de bits para vetor de bytes
fn bits_to_bytes(bits: &str) -> Vec<u8> {
    bits.as_bytes()
        .chunks(8)
        .map(|chunk| {
            let mut byte = 0u8;
            for (i, &b) in chunk.iter().enumerate() {
                if b == b'1' {
                    byte |= 1 << (7 - i);
                }
            }
            byte
        })
        .collect()
}

// converte bytes pra uma string de bits
fn bytes_to_bits(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<_>>()
        .concat()
}

fn compress(input_path: &str, output_path: &str) {
    let mut input = String::new();
    BufReader::new(File::open(input_path).unwrap()).read_to_string(&mut input).unwrap();

    let freq_table = build_frequency_table(&input);
    let huffman_tree = build_huffman_tree(&freq_table);

    let mut codes = HashMap::new();
    build_codes(&huffman_tree, String::new(), &mut codes);

    let encoded = encode(&input, &codes);
    
    let padding = (8 - (encoded.len() % 8)) % 8;                         // calcular quantos bits faltam pra completar o byte.
    let encoded_padded = format!("{}{}", encoded, "0".repeat(padding)); // adiciona zeros que faltam no final (little endian)

    let encoded_bytes = bits_to_bytes(&encoded_padded);

    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    let n = freq_table.len() as u16;
    let t = encoded.len() as u32; // quantidade de bits codificados


    writer.write_all(&n.to_be_bytes()).unwrap();
    writer.write_all(&t.to_be_bytes()).unwrap();

    for (c, f) in &freq_table {
        let c_string = c.to_string();            // armazena a string no stack
        let c_bytes = c_string.as_bytes();        // pega o slice da string viva
        let len = c_bytes.len() as u8;
    
        writer.write_all(&[len]).unwrap();           // escreve o tamanho
        writer.write_all(c_bytes).unwrap();          // escreve os bytes do caractere
        writer.write_all(&f.to_be_bytes()).unwrap(); // escreve a frequência
    }
    
    

    writer.write_all(&encoded_bytes).unwrap();
}

fn decompress(input_path: &str, output_path: &str) {
    let mut reader = BufReader::new(File::open(input_path).unwrap());
    let mut buffer = [0u8; 2];
    reader.read_exact(&mut buffer).unwrap();
    let n = u16::from_be_bytes(buffer); // número de caracteres distintos

    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer).unwrap();
    let t = u32::from_be_bytes(buffer); // total de caracteres

    let mut freq_table = HashMap::new();
    for _ in 0..n {
        let mut len_buf = [0u8; 1];
        reader.read_exact(&mut len_buf).unwrap();
        let len = len_buf[0] as usize;
    
        let mut c_buf = vec![0u8; len];
        reader.read_exact(&mut c_buf).unwrap();
        let c = std::str::from_utf8(&c_buf).unwrap().chars().next().unwrap();
    
        let mut f = [0u8; 4];
        reader.read_exact(&mut f).unwrap();
        let f = u32::from_be_bytes(f);
    
        freq_table.insert(c, f);
    }
    

    let mut encoded_bytes = Vec::new();
    reader.read_to_end(&mut encoded_bytes).unwrap();
    let bits = bytes_to_bits(&encoded_bytes);
    let useful_bits = &bits[..t as usize];

    let huffman_tree = build_huffman_tree(&freq_table);
    let decoded = decode(useful_bits, &huffman_tree);

    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    writer.write_all(decoded.as_bytes()).unwrap();
}

fn main() {
    compress("input.txt", "file.bin");
    decompress("file.bin", "out.txt");
}
