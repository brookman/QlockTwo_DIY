static ES: u128 = 1 << 112 | 1 << 92;
static LA: u128 = 1 << 52 | 1 << 51;
static SON: u128 = 1 << 92 | 1 << 91 | 1 << 72;
static LAS: u128 = 1 << 52 | 1 << 51 | 1 << 32;

static UNA: u128 = 1 << 31 | 1 << 12 | 1 << 10;
static DOS: u128 = 1 << 111 | 1 << 93 | 1 << 90;
static TRES: u128 = 1 << 70 | 1 << 53 | 1 << 50 | 1 << 33;
static CUATRO: u128 = 1 << 110 | 1 << 94 | 1 << 89 | 1 << 74 | 1 << 69 | 1 << 54;
static CINCO: u128 = 1 << 49 | 1 << 34 | 1 << 29 | 1 << 14 | 1 << 8;
static SEIS: u128 = 1 << 109 | 1 << 95 | 1 << 88 | 1 << 75;
static SIETE: u128 = 1 << 55 | 1 << 48 | 1 << 35 | 1 << 28 | 1 << 15;
static OCHO: u128 = 1 << 108 | 1 << 96 | 1 << 87 | 1 << 76;
static NUEVE: u128 = 1 << 67 | 1 << 56 | 1 << 47 | 1 << 36 | 1 << 27;
static DIEZ: u128 = 1 << 86 | 1 << 77 | 1 << 66 | 1 << 57;
static ONCE: u128 = 1 << 37 | 1 << 26 | 1 << 17 | 1 << 5;
static DOCE: u128 = 1 << 106 | 1 << 98 | 1 << 85 | 1 << 78;

static Y: u128 = 1 << 58;
static MENOS: u128 = 1 << 45 | 1 << 38 | 1 << 25 | 1 << 18 | 1 << 4;

static CINCO_2: u128 = 1 << 43 | 1 << 40 | 1 << 23 | 1 << 20 | 1 << 2;
static DIEZ_2: u128 = 1 << 39 | 1 << 24 | 1 << 19 | 1 << 3;
static CUARTO: u128 = 1 << 61 | 1 << 42 | 1 << 41 | 1 << 22 | 1 << 21 | 1 << 1;
static VEINTE: u128 = 1 << 99 | 1 << 84 | 1 << 79 | 1 << 64 | 1 << 59 | 1 << 44;
static VEINTICINCO: u128 = 1 << 104 | 1 << 100 | 1 << 83 | 1 << 80 | 1 << 63 | 1 << 60 | 1 << 43 | 1 << 40 | 1 << 23 | 1 << 20 | 1 << 2;
static MEDIA: u128 = 1 << 103 | 1 << 101 | 1 << 82 | 1 << 81 | 1 << 62;

static M1: u128 = 1 << 11;
static M2: u128 = 1 << 0;
static M3: u128 = 1 << 102;
static M4: u128 = 1 << 113;

pub fn get_led_pattern(hours: u8, minutes: u8) -> u128 {
    let mut bits = 0u128;

    if minutes < 35 { // after
        bits |= get_hour(hours);
        if minutes >= 5 { // for the first 5 min it's "sharp"
            bits |= Y;
        }
    } else { // before
        bits |= get_hour(hours + 1);
        bits |= MENOS;
    }

    if minutes >= 5 && minutes < 10 || minutes >= 55 {
        bits |= CINCO_2;
    } else if minutes >= 10 && minutes < 15 || minutes >= 50 && minutes < 55 {
        bits |= DIEZ_2;
    } else if minutes >= 15 && minutes < 20 || minutes >= 45 && minutes < 50 {
        bits |= CUARTO;
    } else if minutes >= 20 && minutes < 25 || minutes >= 40 && minutes < 45 {
        bits |= VEINTE;
    } else if minutes >= 25 && minutes < 30 || minutes >= 35 && minutes < 40 {
        bits |= VEINTICINCO;
    } else if minutes >= 30 && minutes < 35 {
        bits |= MEDIA;
    }
    bits |= get_minute_dots(minutes);
    return bits;
}


fn get_hour(hours: u8) -> u128 {
    let mut hour = hours % 12;
    if hour == 0 {
        hour = 12;
    }

    let hour_bits = match hour {
        1 => UNA,
        2 => DOS,
        3 => TRES,
        4 => CUATRO,
        5 => CINCO,
        6 => SEIS,
        7 => SIETE,
        8 => OCHO,
        9 => NUEVE,
        10 => DIEZ,
        11 => ONCE,
        12 => DOCE,
        _ => 0
    };
    if hour == 1 {
        ES | LA | hour_bits
    } else {
        SON | LAS | hour_bits
    }
}

fn get_minute_dots(minute: u8) -> u128 {
    return match minute % 5 {
        1 => M1,
        2 => M1 | M2,
        3 => M1 | M2 | M3,
        4 => M1 | M2 | M3 | M4,
        _ => 0
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    static CLOCKFACE: &'static [(char, u8)] = &[
        ('E', 112), ('S', 092), ('O', 91), ('N', 72), ('E', 71), ('L', 52), ('A', 51), ('S', 32), ('U', 31), ('N', 12), ('A', 10),
        ('D', 111), ('O', 093), ('S', 90), ('I', 73), ('T', 70), ('R', 53), ('E', 50), ('S', 33), ('O', 30), ('A', 13), ('M', 09),
        ('C', 110), ('U', 094), ('A', 89), ('T', 74), ('R', 69), ('O', 54), ('C', 49), ('I', 34), ('N', 29), ('C', 14), ('O', 08),
        ('S', 109), ('E', 095), ('I', 88), ('S', 75), ('A', 68), ('S', 55), ('I', 48), ('E', 35), ('T', 28), ('E', 15), ('N', 07),
        ('O', 108), ('C', 096), ('H', 87), ('O', 76), ('N', 67), ('U', 56), ('E', 47), ('V', 36), ('E', 27), ('P', 16), ('M', 06),
        ('L', 107), ('A', 097), ('D', 86), ('I', 77), ('E', 66), ('Z', 57), ('S', 46), ('O', 37), ('N', 26), ('C', 17), ('E', 05),
        ('D', 106), ('O', 098), ('C', 85), ('E', 78), ('L', 65), ('Y', 58), ('M', 45), ('E', 38), ('N', 25), ('O', 18), ('S', 04),
        ('O', 105), ('V', 099), ('E', 84), ('I', 79), ('N', 64), ('T', 59), ('E', 44), ('D', 39), ('I', 24), ('E', 19), ('Z', 03),
        ('V', 104), ('E', 100), ('I', 83), ('N', 80), ('T', 63), ('I', 60), ('C', 43), ('I', 40), ('N', 23), ('C', 20), ('O', 02),
        ('M', 103), ('E', 101), ('D', 82), ('I', 81), ('A', 62), ('C', 61), ('U', 42), ('A', 41), ('R', 22), ('T', 21), ('O', 01),
    ];

    #[test]
    fn expect_patterns_to_match_strings() {
        assert_eq!("ES", evaluate_to_string(ES));
        assert_eq!("LA", evaluate_to_string(LA));
        assert_eq!("SON", evaluate_to_string(SON));
        assert_eq!("LAS", evaluate_to_string(LAS));
        assert_eq!("UNA", evaluate_to_string(UNA));
        assert_eq!("DOS", evaluate_to_string(DOS));
        assert_eq!("TRES", evaluate_to_string(TRES));
        assert_eq!("CUATRO", evaluate_to_string(CUATRO));
        assert_eq!("CINCO", evaluate_to_string(CINCO));
        assert_eq!("SEIS", evaluate_to_string(SEIS));
        assert_eq!("SIETE", evaluate_to_string(SIETE));
        assert_eq!("OCHO", evaluate_to_string(OCHO));
        assert_eq!("NUEVE", evaluate_to_string(NUEVE));
        assert_eq!("DIEZ", evaluate_to_string(DIEZ));
        assert_eq!("ONCE", evaluate_to_string(ONCE));
        assert_eq!("DOCE", evaluate_to_string(DOCE));
        assert_eq!("Y", evaluate_to_string(Y));
        assert_eq!("MENOS", evaluate_to_string(MENOS));
        assert_eq!("CINCO", evaluate_to_string(CINCO_2));
        assert_eq!("DIEZ", evaluate_to_string(DIEZ_2));
        assert_eq!("CUARTO", evaluate_to_string(CUARTO));
        assert_eq!("VEINTE", evaluate_to_string(VEINTE));
        assert_eq!("VEINTICINCO", evaluate_to_string(VEINTICINCO));
        assert_eq!("MEDIA", evaluate_to_string(MEDIA));
    }

    #[test]
    fn expect_time_examples_to_match_strings() {
        assert_time(01, 00, "Es la una");
        assert_time(02, 00, "Son las dos");
        assert_time(03, 00, "Son las tres");
        assert_time(04, 00, "Son las cuatro");
        assert_time(05, 00, "Son las cinco");
        assert_time(06, 00, "Son las seis");
        assert_time(07, 00, "Son las siete");
        assert_time(08, 00, "Son las ocho");
        assert_time(09, 00, "Son las nueve");
        assert_time(10, 00, "Son las diez");
        assert_time(11, 00, "Son las once");
        assert_time(12, 00, "Son las doce");

        assert_time(01, 05, "Es la una y cinco");
        assert_time(02, 10, "Son las dos y diez");
        assert_time(03, 15, "Son las tres y cuarto");
        assert_time(04, 20, "Son las cuatro y veinte");
        assert_time(05, 25, "Son las cinco y veinticinco");
        assert_time(06, 30, "Son las seis y media");
        assert_time(07, 35, "Son las ocho menos veinticinco");
        assert_time(08, 40, "Son las nueve menos veinte");
        assert_time(09, 45, "Son las diez menos cuarto");
        assert_time(10, 50, "Son las once menos diez");
        assert_time(11, 55, "Son las doce menos cinco");

        assert_time(13, 05, "Es la una y cinco");
        assert_time(14, 10, "Son las dos y diez");
        assert_time(15, 15, "Son las tres y cuarto");
        assert_time(16, 20, "Son las cuatro y veinte");
        assert_time(17, 25, "Son las cinco y veinticinco");
        assert_time(18, 30, "Son las seis y media");
        assert_time(19, 35, "Son las ocho menos veinticinco");
        assert_time(20, 40, "Son las nueve menos veinte");
        assert_time(21, 45, "Son las diez menos cuarto");
        assert_time(22, 50, "Son las once menos diez");
        assert_time(23, 55, "Son las doce menos cinco");

        assert_time(24, 00, "Son las doce");
    }

    #[test]
    fn expect_dots_to_match_minutes() {
        let zero = get_led_pattern(0, 0);
        let one = get_led_pattern(1, 1);
        let two = get_led_pattern(2, 2);
        let three = get_led_pattern(3, 3);
        let four = get_led_pattern(4, 4);
        let five = get_led_pattern(5, 5);

        assert!(zero & M1 == 0);
        assert!(zero & M2 == 0);
        assert!(zero & M3 == 0);
        assert!(zero & M4 == 0);

        assert!(one & M1 != 0);
        assert!(one & M2 == 0);
        assert!(one & M3 == 0);
        assert!(one & M4 == 0);

        assert!(two & M1 != 0);
        assert!(two & M2 != 0);
        assert!(two & M3 == 0);
        assert!(two & M4 == 0);

        assert!(three & M1 != 0);
        assert!(three & M2 != 0);
        assert!(three & M3 != 0);
        assert!(three & M4 == 0);

        assert!(four & M1 != 0);
        assert!(four & M2 != 0);
        assert!(four & M3 != 0);
        assert!(four & M4 != 0);

        assert!(five & M1 == 0);
        assert!(five & M2 == 0);
        assert!(five & M3 == 0);
        assert!(five & M4 == 0);
    }

    #[test]
    fn expect_same_results_for_am_and_pm() {
        assert_eq!(get_led_pattern(0, 0), get_led_pattern(12, 0));
        assert_eq!(get_led_pattern(1, 1), get_led_pattern(13, 1));
        assert_eq!(get_led_pattern(2, 2), get_led_pattern(14, 2));
        assert_eq!(get_led_pattern(3, 3), get_led_pattern(15, 3));
        assert_eq!(get_led_pattern(4, 4), get_led_pattern(16, 4));
        assert_eq!(get_led_pattern(5, 5), get_led_pattern(17, 5));
        assert_eq!(get_led_pattern(6, 6), get_led_pattern(18, 6));
        assert_eq!(get_led_pattern(7, 7), get_led_pattern(19, 7));
        assert_eq!(get_led_pattern(8, 8), get_led_pattern(20, 8));
        assert_eq!(get_led_pattern(9, 9), get_led_pattern(21, 9));
        assert_eq!(get_led_pattern(10, 10), get_led_pattern(22, 10));
        assert_eq!(get_led_pattern(11, 11), get_led_pattern(23, 11));
        assert_eq!(get_led_pattern(0, 24), get_led_pattern(12, 24));
        assert_eq!(get_led_pattern(1, 25), get_led_pattern(13, 25));
        assert_eq!(get_led_pattern(2, 26), get_led_pattern(14, 26));
        assert_eq!(get_led_pattern(3, 27), get_led_pattern(15, 27));
        assert_eq!(get_led_pattern(4, 28), get_led_pattern(16, 28));
        assert_eq!(get_led_pattern(5, 29), get_led_pattern(17, 29));
        assert_eq!(get_led_pattern(6, 30), get_led_pattern(18, 30));
        assert_eq!(get_led_pattern(7, 31), get_led_pattern(19, 31));
        assert_eq!(get_led_pattern(8, 32), get_led_pattern(20, 32));
        assert_eq!(get_led_pattern(9, 33), get_led_pattern(21, 33));
        assert_eq!(get_led_pattern(10, 34), get_led_pattern(22, 34));
        assert_eq!(get_led_pattern(11, 35), get_led_pattern(23, 35));
        assert_eq!(get_led_pattern(11, 36), get_led_pattern(23, 36));
    }

    fn assert_time(hours_24h: u8, minutes: u8, time_string: &str) {
        assert_eq!(remove_whitespace(time_string).to_uppercase(), evaluate_to_string(get_led_pattern(hours_24h, minutes)));
    }

    fn remove_whitespace(s: &str) -> String {
        s.chars()
            .filter(|c| !c.is_whitespace())
            .collect()
    }

    fn evaluate_to_string(pattern: u128) -> String {
        return CLOCKFACE.iter()
            .filter(|c| (1 << (*c).1) & pattern != 0)
            .map(|i| i.0)
            .collect();
    }
}