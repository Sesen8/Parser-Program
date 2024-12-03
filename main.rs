use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::option;
use std::fmt::Display;

//use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PROGRAM, DATA, DATADEFS, DATADEF, INPUT, INPUTOPS, INPUTOP, 
    PROCESS, PROCESSOPS, PROCESSOP, OUTPUT, OUTPUTOPS, OUTPUTOP,
    END, ID, NUM, TRUE, FALSE, READ, COLON, COMMA, PERIOD,
    LPAREN, RPAREN, ASSIGN, VECTOR, NUMBER, REGRESSIONA,
    REGRESSIONB, MEAN, STDDEV, CORRELATION, STRING, TYPE, NEWLINE,
}


//dont need for now
#[derive(Debug, Clone)]
pub struct Token {
    pub typeoftoken: TokenType,
    pub word: String,
}





//matches the words and terminals found to their types
fn assign(word: &str) -> Option<TokenType>{    
    let lowercase_word = word.to_ascii_lowercase();

    if lowercase_word != word {
        return None;
        //panic!("Lexical Error Found")

    }

    match word {
        "program" => Some(TokenType::PROGRAM),

        "data" => Some(TokenType::DATA),
        "datadefs" => Some(TokenType::DATADEFS),
        "datadef" => Some(TokenType::DATADEF),

        "inputops" => Some(TokenType::INPUTOPS),
        "inputop" => Some(TokenType::INPUTOP),
        "input" => Some(TokenType::INPUT),

        "processops" => Some(TokenType::PROCESSOPS),
        "processop" => Some(TokenType::PROCESSOP),
        "process" => Some(TokenType::PROCESS),
        
        "outputops" => Some(TokenType::OUTPUTOPS),
        "outputop" => Some(TokenType::OUTPUTOP),
        "output" => Some(TokenType::OUTPUT),

        "true" => Some(TokenType::TRUE),
        "false" => Some(TokenType::FALSE),
        "read" => Some(TokenType::READ),

        ":" => Some(TokenType::COLON),
        "," => Some(TokenType::COMMA),
        "." => Some(TokenType::PERIOD),
        "(" => Some(TokenType::LPAREN),
        ")" => Some(TokenType::RPAREN),
        "=" => Some(TokenType::ASSIGN),

        "vector" => Some(TokenType::VECTOR),
        "number" => Some(TokenType::NUMBER),

        "regressiona" => Some(TokenType::REGRESSIONA),
        "regressionb" => Some(TokenType::REGRESSIONB),
        "mean" => Some(TokenType::MEAN),
        "stddev" => Some(TokenType::STDDEV),
        "correlation" => Some(TokenType::CORRELATION),
        "type" => Some(TokenType::TYPE),
        "end" => Some(TokenType::END),

        _ if word.starts_with('"') && word.ends_with('"') => Some(TokenType::STRING),
        _ if word.parse::<i64>().is_ok() => Some(TokenType::NUM),
        _ if word.chars().all(|c| c.is_alphanumeric() || c == '_') => Some(TokenType::ID),

        
        "\n" => Some(TokenType::NEWLINE),
        
        _ => {
           
            None

        }
        
    }
}




fn tokenize(word: &str, typelist: &mut Vec<TokenType>, wordlist: &mut Vec<String>){
    
    wordlist.push(word.to_string());
    

    if let Some(token_type) = assign(&word) {
        typelist.push(token_type.clone());
        let token_type_clone = token_type.clone();

       println!("{:?} for '{}'", token_type_clone, word);

       //makes a token instance for later use
       let token = Token {
            typeoftoken: token_type,
            word: word.to_string(),
        };


    } else {
        println!("Unknown token type for word: {}", word);
        //eprintln!("Lexical Error");
    }
    
  


}



fn main() {

    // reads cargo command // andre helped with the vector and args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the DA program file path as an argument.");
        return;
    }
    let file_path = &args[1];  
    let flag = &args[2];

    if flag != "-s" && flag != "-p" {
        eprintln!("Invalid flag. Please use -s or -p.");
        return;
    }

    println!("Processing Input File: {}", file_path); 

    
    //opens the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening the file: {}", err);
            return;
        }
    };

    //file buffer, tmp current word, word vector, token type vetor
    let buf_reader = io::BufReader::new(file);
    let mut currentword = String::new();
    let mut typelist: Vec<TokenType> = Vec::new();
    let mut wordlist: Vec<String> = Vec::new();
    let mut passed = false;
    

    
    //andre said read by character not just lines
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            
            let mut string_check = false; 

            for ch in line.chars() {    
                if ch == '\"'{
                    string_check = !string_check; 
                    
                    }  
                if string_check{
                    currentword.push(ch);
            
                    }
                else if ch == ':' || ch == '=' || ch == '-' ||  
                    ch == '(' || ch == ')' || ch == '.'  || 
                    ch == ' ' || ch == ',' || ch == '\r' || ch == '\n' {
                        if !currentword.is_empty(){
                            tokenize(&currentword.clone(), &mut typelist, &mut wordlist);
                        }
                        currentword.clear();

                        //andre said to include this
                        if ch != ' ' {
                            tokenize(&String::from(ch), &mut typelist, &mut wordlist);
                        }        
                    }
                
                else  {
                    currentword.push(ch);                    
                    }

                }

            } 

            //grabs last words, fixed last problem
        if !currentword.is_empty() {
                tokenize(&currentword.clone(), &mut typelist, &mut wordlist);
                currentword.clear();
        }    
    }


    Parse(typelist, &mut passed);

    if passed{
        println!("Lexical and Syntax analysis passed");

    }
    

}



fn Parse(typelist: Vec<TokenType>, passed: &mut bool ){
    
    let mut index = 0;
    let mut iter = typelist.iter().peekable();

    while let Some(token) = iter.next() {
        
        if token == &TokenType::from_str("DATA").unwrap() {
            index+=1;
            if let Some(&next_token) = iter.peek() {
                //println!("Processing token: {:?}", token); 
                if next_token == &TokenType::from_str("COLON").unwrap() {
                    iter.next();
                    index+=1;
                    datasection(typelist.clone(), &mut index);
                }
            }
        } 
        
        else if token == &TokenType::from_str("INPUT").unwrap() {
            index+=1;
            if let Some(&next_token) = iter.peek(){
                //println!("Processing token: {:?}", token); 
                if next_token == &TokenType::from_str("COLON").unwrap() {
                    iter.next();
                    index+=1;
                    inputsection(typelist.clone(), &mut index);
                }

            }
            
        } 

        else if token == &TokenType::from_str("PROCESS").unwrap() {
            index+=1;
            if let Some(&next_token) = iter.peek(){
                //println!("Processing token: {:?}", token); 
                if next_token == &TokenType::from_str("COLON").unwrap() {
                    iter.next();
                    index+=1;
                    processsection(typelist.clone(), &mut index);
                }
                
            }
            
        }
        

        else if token == &TokenType::from_str("OUTPUT").unwrap() {
            index+=1;
            if let Some(&next_token) = iter.peek(){
                //println!("Processing token: {:?}", token); 
                if next_token == &TokenType::from_str("COLON").unwrap() {
                    iter.next();
                    index+=1;
                    outputsection(typelist.clone(), &mut index);
                }
                
            }
            
        } 
        
        
    }

    *passed = true;



    
}
    

impl TokenType {
    fn from_str(input: &str) -> Option<Self> {
        match input.to_ascii_uppercase().as_str() {
            "PROGRAM" => Some(TokenType::PROGRAM),
            "DATA" => Some(TokenType::DATA),
            "DATADEFS" => Some(TokenType::DATADEFS),
            "DATADEF" => Some(TokenType::DATADEF),
            "INPUT" => Some(TokenType::INPUT),
            "INPUTOPS" => Some(TokenType::INPUTOPS),
            "INPUTOP" => Some(TokenType::INPUTOP),
            "PROCESS" => Some(TokenType::PROCESS),
            "PROCESSOPS" => Some(TokenType::PROCESSOPS),
            "PROCESSOP" => Some(TokenType::PROCESSOP),
            "OUTPUT" => Some(TokenType::OUTPUT),
            "OUTPUTOPS" => Some(TokenType::OUTPUTOPS),
            "OUTPUTOP" => Some(TokenType::OUTPUTOP),
            "END" => Some(TokenType::END),
            "ID" => Some(TokenType::ID),
            "NUM" => Some(TokenType::NUM),
            "TRUE" => Some(TokenType::TRUE),
            "FALSE" => Some(TokenType::FALSE),
            "READ" => Some(TokenType::READ),
            "COLON" => Some(TokenType::COLON),
            "COMMA" => Some(TokenType::COMMA),
            "PERIOD" => Some(TokenType::PERIOD),
            "LPAREN" => Some(TokenType::LPAREN),
            "RPAREN" => Some(TokenType::RPAREN),
            "ASSIGN" => Some(TokenType::ASSIGN),
            "VECTOR" => Some(TokenType::VECTOR),
            "NUMBER" => Some(TokenType::NUMBER),
            "REGRESSIONA" => Some(TokenType::REGRESSIONA),
            "REGRESSIONB" => Some(TokenType::REGRESSIONB),
            "MEAN" => Some(TokenType::MEAN),
            "STDDEV" => Some(TokenType::STDDEV),
            "CORRELATION" => Some(TokenType::CORRELATION),
            "STRING" => Some(TokenType::STRING),
            "TYPE" => Some(TokenType::TYPE),
            "NEWLINE" => Some(TokenType::NEWLINE),
            _ => None,
        }
    }
}



fn datasection(typelist: Vec<TokenType> , index: &mut usize){
    
    
    dataparsing(typelist.clone(), index); 
    let mut iter = typelist.iter().skip(*index).peekable();
    
    
    while let Some(token) = iter.next() {
       
        if token == &TokenType::COMMA {
            *index+=1;
            datasection(typelist.clone(), index);
            
        }
        if token == &TokenType::INPUT {
            break;
        }
        else{   
            break;
        }
    }

    
    

}



fn dataparsing(typelist: Vec<TokenType>,  index: &mut usize){
    let mut iter = typelist.iter().skip(*index).peekable();
    while let Some(token) = iter.next() {
        match token {
            TokenType::ID => {
                
                *index += 1;
            }
            TokenType::COLON => {
                
                *index += 1;
                if let Some(&type_token) = iter.peek() {
                    //println!("Processing token: {:?}", type_token);
                    if *type_token == TokenType::VECTOR || *type_token == TokenType::NUMBER {
                        //iter.next();
                        *index += 1;
                    } else {
                        println!("Syntax Error: Expected VECTOR or NUMBER");
                        
                        break;
                    }
                } else {
                    println!("Syntax Error: Expected VECTOR or NUMBER");
                    break;
                }
            }
            TokenType::INPUT => {
                
                break;  
            }

            _ => {
                break;
            }
        }
    }
}






fn inputsection(typelist: Vec<TokenType>,  index: &mut usize){
    
    inputparsing(typelist.clone(), index);
    let mut iter = typelist.iter().skip(*index).peekable();
    
    while let Some(token) = iter.next() {
        
        if token == &TokenType::COMMA {
            
            *index+=1;
            inputsection(typelist.clone(), index);
            
        }

        if token == &TokenType::PROCESS {
            break;
        }

        else{
            
            break;
        }
    }
    
}




fn inputparsing(typelist: Vec<TokenType>, index: &mut usize){

    let mut iter = typelist.iter().skip(*index).peekable();
    while let Some(token) = iter.next() {
        match token {
            TokenType::ID => {
                
                *index += 1;
            }
            TokenType::ASSIGN => {
                
                *index += 1;
            }
            TokenType::READ => {
                
                *index += 1;
            }
            TokenType::LPAREN => {
               
                *index += 1;
            }
            TokenType::STRING => {
                
                *index += 1;
            }
            TokenType::COMMA => {
                
                *index += 1;
            }
            TokenType::FALSE | TokenType::TRUE => {
                
                *index += 1;
            }
            TokenType::COMMA => {
                
                *index += 1;
            }
            TokenType::NUM => {
                
                *index += 1;
            }
            TokenType::RPAREN => {
                
                *index += 1;
                break;
            }
            _ => {
                println!("Syntax Error: Unexpected token: {:?}", token);
                break;
            }
        }
    }

}




fn processsection(typelist: Vec<TokenType>,  index: &mut usize){
    
    
    processparsing(typelist.clone(), index);
    let mut iter = typelist.iter().skip(*index).peekable();
    
    
    while let Some(token) = iter.next() {
       
        if token == &TokenType::COMMA {
            *index+=1;
            processsection(typelist.clone(), index);    
        }

        if token == &TokenType::OUTPUT {
            break;
        }

        else{
            
            break;
        }
    }
    
}





fn processparsing(typelist: Vec<TokenType>, index: &mut usize) {
    let mut iter = typelist.iter().skip(*index).peekable();

    while let Some(token) = iter.next() {
        match token {
            TokenType::ID => {
                
                *index += 1;
            }
            TokenType::ASSIGN => {
                
                *index += 1;

                if let Some(next_token) = iter.next() {
                    match next_token {
                        TokenType::REGRESSIONA | TokenType::REGRESSIONB | TokenType::CORRELATION
                        | TokenType::MEAN | TokenType::STDDEV => {
                            
                            *index += 1;

                            // LPAREN
                            if let Some(op_token) = iter.next() {
                                match op_token {
                                    TokenType::LPAREN => {
                                        
                                        *index += 1;
                                    }
                                    _ => {
                                        println!("Syntax Error: Expected LPAREN, found: {:?}", op_token);
                                        break;
                                    }
                                }
                            } 
                            else {
                                println!("Syntax Error: Unexpected end of input after {:?}", next_token);
                                break;
                            }



                            // ID
                            if let Some(op_token) = iter.next() {
                                match op_token {
                                    TokenType::ID => {
                                        
                                        *index += 1;
                                    }
                                    _ => {
                                        println!("Syntax Error: Expected ID, found: {:?}", op_token);
                                        break;
                                    }
                                }
                            } 
                            
                            else {
                                println!("Syntax Error: Unexpected end of input after {:?}", next_token);
                                break;
                            }

                            // possible COMMA and 2nd ID
                            if let Some(&next_token) = iter.peek() {
                                match next_token {
                                    TokenType::COMMA => {
                                        if let Some(this_token) = iter.next() {
                                            
                                            *index += 1;
                                        } else {
                                            println!("Syntax Error: Unexpected end of input after {:?}", next_token);
                                            break;
                                        }

                                        if let Some(this_token) = iter.next() {
                                            match this_token {
                                                TokenType::ID => {
                                                    
                                                    *index += 1;
                                                }
                                                _ => {
                                                    println!("Syntax Error: Expected ID, found: {:?}", this_token);
                                                    break;
                                                }
                                            }
                                        } else {
                                            println!("Syntax Error: Unexpected end of input after COMMA");
                                            break;
                                        }
                                    }
                                    _ => {} 
                                }
                            }

                            //RPAREN
                            if let Some(this_token) = iter.next() {
                                match this_token {
                                    TokenType::RPAREN => {
                                        
                                        *index += 1;

                                    }
                                    _ => {
                                        println!("Syntax Error: Expected RPAREN, found: {:?}", this_token);
                                        break;
                                        
                                    }
                                }
                            } 
                            else {
                                println!("Syntax Error: Unexpected end of input after {:?}", next_token);
                                break;
                                
                            }

                            // ending COMMA (was giving me a bug so i added it)
                            if let Some(&TokenType::COMMA) = iter.peek() {
                                *index += 1;
                                
                                processsection(typelist.clone(), index);
                            }




                        }
                        _ => {
                            println!("Syntax Error: Unexpected process operation after ASSIGN: {:?}", next_token);
                            break;
                        }
                    }
                } else {
                    println!("Syntax Error: Expected process operation after ASSIGN");
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
}




fn outputsection(typelist: Vec<TokenType>,  index: &mut usize){
    
    outputparsing(typelist.clone(), index);    
    let mut iter = typelist.iter().skip(*index).peekable();
    
    while let Some(token) = iter.next() {
        
        if token == &TokenType::COMMA {
            
            *index+=1;
            outputsection(typelist.clone(), index);   
        }

        if token == &TokenType::END {
           
            break;
        }

        else{
            
            break;
        }
    }
    
    
}







fn outputparsing(typelist: Vec<TokenType>, index: &mut usize){

    let mut iter = typelist.iter().skip(*index).peekable();
    while let Some(token) = iter.next() {
        match token {
            TokenType::STRING => {
                //println!("Processing token: {:?}", token);
                *index += 1;
            }

            TokenType::ID =>{
                //println!("Processing token: {:?}", token);
                *index += 1;
                
            }

            TokenType::NEWLINE =>{
                *index += 1;
            }

            TokenType::END | TokenType::PERIOD => {
                //println!("Processing token: {:?}", token);
                *index += 1;
            }
            _ => {
                break;
            }
        }
    }
    

}














