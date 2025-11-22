

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MType {
    Quiet = 0,
    Capture = 1,
    Promote = 2,
    KingsideCastle = 3,
    QueensideCastle = 4,
    EnPassant = 5,
    PromotionCapture = 6,
    None = 7,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdditionalInfo {
    PromoPieceType(u8),
    CastlingRookPos(u8),
    Custom(u8),
}


#[derive(PartialEq,Copy,Clone)]
pub struct Move(u32);

impl Move {
    pub fn encode(src: u8, dest: u8, mtype: MType, extra: Option<AdditionalInfo>) -> Self {
        let mut value: u32 = ((src as u32) << 16) | ((dest as u32) << 8) | (mtype as u32);

        if let Some(extra_info) = extra {
            let extra_val = match extra_info {
                AdditionalInfo::PromoPieceType(p) => p,
                AdditionalInfo::CastlingRookPos(p) => p,
                AdditionalInfo::Custom(p) => p,
            };
            value |= (extra_val as u32) << 24;
        }

        Move(value)
    }

    pub fn from_square(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    pub fn to_square(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    pub fn mtype(&self) -> MType {
        match (self.0 & 0b111) as u8 {
            0 => MType::Quiet,
            1 => MType::Capture,
            2 => MType::Promote,
            3 => MType::KingsideCastle,
            4 => MType::QueensideCastle,
            5 => MType::EnPassant,
            6 => MType::PromotionCapture,
            _ => MType::None,
        }
    }

    pub fn extra(&self) -> Option<u8> {
        let val = ((self.0 >> 24) & 0xFF) as u8;
        if val == 0 { None } else { Some(val) }
    }
}
