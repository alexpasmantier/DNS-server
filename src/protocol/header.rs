#[derive(Default, Debug)]
pub struct DNSHeader {
    /// 16 bits
    /// A random ID assigned to query packets. Response packets must reply with the same ID. Expected: 1234.
    pub id: u16,

    /// 1 bit
    /// 1 for a reply packet, 0 for a question packet. Expected: 1.
    pub qr: u8,

    /// 4 bits
    /// Specifies the kind of query in a message. Expected: 0.
    pub opcode: u8,

    /// 1 bit
    /// 1 if the responding server "owns" the domain queried, i.e., it's authoritative. Expected: 0.
    pub aa: u8,

    /// 1 bit
    /// 1 if the message is larger than 512 bytes. Always 0 in UDP responses. Expected: 0.
    pub tc: u8,

    /// 1 bit
    /// Sender sets this to 1 if the server should recursively resolve this query, 0 otherwise. Expected: 0.
    pub rd: u8,

    /// 1 bit
    /// Server sets this to 1 to indicate that recursion is available. Expected: 0.
    pub ra: u8,

    /// 3 bits
    /// Used by DNSSEC queries. At inception, it was reserved for future use. Expected: 0.
    pub z: u8,

    /// 4 bits
    /// Response code indicating the status of the response. Expected: 0.
    pub rcode: u8,

    /// 16 bits
    /// Number of questions in the Question section. Expected: 0.
    pub qdcount: u16,

    /// 16 bits
    /// Number of records in the Answer section. Expected: 0.
    pub ancount: u16,

    /// 16 bits
    /// Number of records in the Authority section. Expected: 0.
    pub nscount: u16,

    /// 16 bits
    /// Number of records in the Additional section. Expected: 0.
    pub arcount: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut byte_array = [0; 12];
        // convert header as two bytes
        byte_array[..2].copy_from_slice(&self.id.to_be_bytes());

        // convert {qr, opcode, aa, tc, rd} into 1 byte
        let qr_opcode_aa_tc_rd =
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        byte_array[2] = qr_opcode_aa_tc_rd;

        // convert {ra, z, rcode} into 1 byte
        let ra_z_rcode = (self.ra << 7) | (self.z << 4) | self.rcode;
        byte_array[3] = ra_z_rcode;

        // convert qdcount into 2 bytes
        byte_array[4..6].copy_from_slice(&self.qdcount.to_be_bytes());

        // convert ancount into 2 bytes
        byte_array[6..8].copy_from_slice(&self.ancount.to_be_bytes());

        // convert nscount into 2 bytes
        byte_array[8..10].copy_from_slice(&self.nscount.to_be_bytes());

        // convert arcount into 2 bytes
        byte_array[10..12].copy_from_slice(&self.arcount.to_be_bytes());

        println!("byte_array is {:?}", byte_array);
        byte_array
    }
}
