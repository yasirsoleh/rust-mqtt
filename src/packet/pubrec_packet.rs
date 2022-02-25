use heapless::Vec;

use crate::encoding::variable_byte_integer::VariableByteIntegerEncoder;
use crate::packet::mqtt_packet::Packet;
use crate::utils::buffer_reader::BuffReader;
use crate::utils::buffer_writer::BuffWriter;

use super::packet_type::PacketType;
use super::property::Property;

pub struct PubrecPacket<'a, const MAX_PROPERTIES: usize> {
    // 7 - 4 mqtt control packet type, 3-0 flagy
    pub fixed_header: u8,
    // 1 - 4 B lenght of variable header + len of payload
    pub remain_len: u32,

    pub packet_identifier: u16,
    pub reason_code: u8,

    pub property_len: u32,

    // properties
    pub properties: Vec<Property<'a>, MAX_PROPERTIES>,
}

impl<'a, const MAX_PROPERTIES: usize> PubrecPacket<'a, MAX_PROPERTIES> {
    pub fn decode_pubrec_packet(&mut self, buff_reader: &mut BuffReader<'a>) {
        if self.decode_fixed_header(buff_reader) != (PacketType::Pubrec).into() {
            log::error!("Packet you are trying to decode is not PUBREC packet!");
            return;
        }
        self.packet_identifier = buff_reader.read_u16().unwrap();
        self.reason_code = buff_reader.read_u8().unwrap();
        self.decode_properties(buff_reader);
    }
}

impl<'a, const MAX_PROPERTIES: usize> Packet<'a> for PubrecPacket<'a, MAX_PROPERTIES> {
    fn new() -> Self {
        todo!()
    }

    fn encode(&mut self, buffer: &mut [u8]) -> usize {
        let mut buff_writer = BuffWriter::new(buffer);

        let mut rm_ln = self.property_len;
        let property_len_enc: [u8; 4] =
            VariableByteIntegerEncoder::encode(self.property_len).unwrap();
        let property_len_len = VariableByteIntegerEncoder::len(property_len_enc);
        rm_ln = rm_ln + property_len_len as u32 + 3;

        buff_writer.write_u8(self.fixed_header);
        buff_writer.write_variable_byte_int(rm_ln);
        buff_writer.write_u16(self.packet_identifier);
        buff_writer.write_u8(self.reason_code);
        buff_writer.write_variable_byte_int(self.property_len);
        buff_writer.encode_properties::<MAX_PROPERTIES>(&self.properties);
        return buff_writer.position;
    }

    fn decode(&mut self, buff_reader: &mut BuffReader<'a>) {
        self.decode_pubrec_packet(buff_reader);
    }

    fn set_property_len(&mut self, value: u32) {
        self.property_len = value;
    }

    fn get_property_len(&mut self) -> u32 {
        return self.property_len;
    }

    fn push_to_properties(&mut self, property: Property<'a>) {
        self.properties.push(property);
    }
    fn set_fixed_header(&mut self, header: u8) {
        self.fixed_header = header;
    }

    fn set_remaining_len(&mut self, remaining_len: u32) {
        self.remain_len = remaining_len;
    }
}
