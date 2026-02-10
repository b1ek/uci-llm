use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub enum ChessPieces {
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    BlackPawn,
    
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    WhitePawn
}

impl TryFrom<char> for ChessPieces {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'r' => Ok(Self::BlackRook),
            'n' => Ok(Self::BlackKnight),
            'b' => Ok(Self::BlackBishop),
            'q' => Ok(Self::BlackQueen),
            'k' => Ok(Self::BlackKing),
            'p' => Ok(Self::BlackPawn),

            'R' => Ok(Self::WhiteRook),
            'N' => Ok(Self::WhiteKnight),
            'B' => Ok(Self::WhiteBishop),
            'Q' => Ok(Self::WhiteQueen),
            'K' => Ok(Self::WhiteKing),
            'P' => Ok(Self::WhitePawn),
            _ => Err(())
        }
    }
}

impl ChessPieces {
    pub fn as_human(&self) -> String {
        match self {
            Self::BlackRook => "Black rook",
            Self::BlackKnight => "Black knight",
            Self::BlackBishop => "Black bishop",
            Self::BlackQueen => "Black queen",
            Self::BlackKing => "Black king",
            Self::BlackPawn => "Black pawn",

            Self::WhiteRook => "White rook",
            Self::WhiteKnight => "White knight",
            Self::WhiteBishop => "White bishop",
            Self::WhiteQueen => "White queen",
            Self::WhiteKing => "White king",
            Self::WhitePawn => "White pawn",
        }.to_string()
    }
}

pub fn fen2md(fen: String) -> Result<String, String> {
    let components: Vec<_> = fen.split('/').collect();
    if components.len() != 8 {
        return Err("Invalid amount of components".to_string());
    }

    let signature: Vec<_> = components[7].split_whitespace().skip(1).collect();
    if signature.len() != 5 {
        return Err("Invalid signature".to_string());
    }

    let turn = signature[0];
    if turn.len() != 1 {
        return Err("Invalid turn indicator".to_string());
    }

    let turn = turn.chars().next().unwrap();
    if turn != 'w' && turn != 'b' {
        return Err("Invalid turn, must be either w or b".to_string());
    }

    let castles = signature[1];
    if !(1..=4).contains(&castles.len()) {
        return Err("Invalid castles notation".to_string());
    }

    let en_passant = signature[2];
    if !(1..=2).contains(&en_passant.len()) {
        return Err("Invalid en passant notation".to_string());
    }

    let halfmove = signature[3];
    let fullmove = signature[4];

    let mut markdown = format!("# Board\nFEN Notation is: {fen}\n## Readable digest of this FEN\n### Position\n");
    let mut invalid_pieces = vec![];
    let mut rank = 9_u8;

    for component in components {
        rank -= 1;
        let mut file = 0_u8;
        
        for chr in component.chars() {
            if chr.is_ascii_digit() {
                file += chr.to_digit(10).unwrap() as u8;
                continue;
            }
            if chr == ' ' && rank == 1 {
                break;
            }
            file += 1;
            
            let char_file = (('a' as u8) + (file - 1)) as char;
            match ChessPieces::try_from(chr) {
                Ok(v) => writeln!(markdown, "{} on {char_file}{rank}", v.as_human()).unwrap(),
                Err(_) => {
                    invalid_pieces.push(format!("{chr}{char_file}{rank}"));
                    continue;
                }
            }
        }
    }

    write!(markdown, "\n### Other\n").unwrap();

    if invalid_pieces.len() != 0 {
        return Err(format!("Invalid pieces: {}", invalid_pieces.join(", ")));
    }

    let turn = match turn {
        'w' => "white",
        'b' => "black",
        _ => unreachable!()
    }.to_string();

    writeln!(markdown, "Its {turn}'s turn\n").unwrap();

    if castles == "-" {
        writeln!(markdown, "No castling available").unwrap();
    } else {
        writeln!(markdown, "Castling options are:").unwrap();
        for castle in castles.chars() {
            let castle = match castle {
                'K' => "White can castle short",
                'Q' => "White can castle long",
                'k' => "Black can castle short",
                'q' => "Black can castle long",
                _ => return Err(format!("Invalid castling option: {castle}"))
            };
            writeln!(markdown, "- {castle}").unwrap();
        }
    }

    writeln!(markdown).unwrap();

    if en_passant == "-" {
        writeln!(markdown, "No en passant available").unwrap();
    } else {
        writeln!(markdown, "En passant available on {en_passant}").unwrap();
    }

    write!(markdown, "\nMove clocks:\n- Halfmove clock (for 50-move rule): {halfmove}\n- Fullmove clock: {fullmove}\n").unwrap();

    Ok(markdown)
}
