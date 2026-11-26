

#[derive(Debug, Clone, PartialEq)]
enum TOKENS{
    TOKENFLOAT(f64),
    TOKENADD,
    TOKENSUB,
    TOKENMUL,
    TOKENDIV
}

fn Convert(Array: Vec<String>) -> Vec<TOKENS>{
    let mut ResultArray : Vec<TOKENS> = Vec::new();
    for i in Array{
        println!("{}", i);
        match i.parse::<f64>(){
            Err(e)  => {match i{
                val if val == '+'.to_string() => ResultArray.push(TOKENS::TOKENADD),
                val if val == '-'.to_string() => ResultArray.push(TOKENS::TOKENSUB),
                val if val == '*'.to_string() => ResultArray.push(TOKENS::TOKENMUL),
                val if val == '/'.to_string() => ResultArray.push(TOKENS::TOKENDIV),
                _ => {},
            }}, //match statement
            Ok(num) => {ResultArray.push(TOKENS::TOKENFLOAT(num))},//Add enum
        }
    }
    return ResultArray;
}

fn Tokenize(StringInput: String)-> Vec<TOKENS>{
    let Characters: Vec<char> = StringInput.chars().collect(); //array of every element of the string
    let mut Buffer : String = "".to_string(); 
    let mut ElonMusk :Vec<String> = Vec::new();
    for i in Characters{  //iterator
            match i{
                '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => Buffer.push(i),
                '.' => {
                    let mut Bool: i8 = 0;
                    let B : Vec<char> = Buffer.chars().collect();
                    for i in B{
                        if i == '.' {Bool = 1; println!("Buffer alr has a '.' baka!!!");}
                    }
                    if Bool == 0 {Buffer.push(i);}
                }
                '+'|'-'|'/'|'*' => {ElonMusk.push(Buffer); ElonMusk.push(i.to_string()); Buffer = "".to_string();},
                _   => {},
        }
    }
    if Buffer != "".to_string(){ElonMusk.push(Buffer); Buffer = "".to_string();}
    return Convert(ElonMusk); //Elon Mosque
}

fn Parse(RawArray : Vec<TOKENS>)-> Vec<TOKENS>{
    let V: Vec<TOKENS> = Vec::new();
    let ArrSize : usize = if RawArray.is_empty(){0} else {RawArray.len()}; //basically trinary
    let mut OpLoc :Vec<usize> = Vec::new();
    return V;
}


pub fn Calculate() -> f64{ //impliment laterr
    return 0.0; //basically pass
}
