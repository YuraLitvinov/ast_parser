use openai_rust::{chat::{ChatArguments, ChatCompletion}}; 
use gemini_rust::{Gemini, Message, Role, Content};
use tokio;
use dotenv::dotenv;
use std::fs;
use syn::{File, Item};
use std::error::Error as Err;


struct OpenAI;
impl OpenAI {
    async fn connect(file_content: String) -> Result<(), Box<dyn Err>>{
        let client = openai_rust::Client::new(&std::env::var("API_KEY_OPENAI").unwrap());
        let args = openai_rust::chat::ChatArguments::new(&std::env::var("OPENAI_MODEL").expect("Unsupported OpenAI model"), vec![
            openai_rust::chat::Message {
                role: "user".to_owned(),
                content: file_content.to_owned(),

            }
        ]);
        let res = client.create_chat(args).await.unwrap();
        println!("{}", res);
        Ok(())
    }
    
}

struct GoogleGemini;
impl GoogleGemini {
    async fn connect(file_content: String) -> Result<(), Box<dyn std::error::Error>>{
    let api_key = std::env::var("API_KEY_GEMINI")?;
    let client = Gemini::new(&api_key);
    
    let response = client.generate_content()
        .with_system_prompt("You are a good assistant")
        .with_user_message(file_content)
        .execute()
        .await?;
    
    println!("Response: {}", response.text());
    
    Ok(())

    }

    
}


fn file_get() -> () {
    let cmd_args: Vec<String> = env::args().collect();
    let file_path= &cmd_args[1].clone();
    println!("{file_path}")
}
fn parse(){
    let source = fs::read_to_string("main.rs").expect("Unable to read file");
    let ast: File = syn::parse_file(&source).expect("Unable to parse file");

    for item in ast.items {
        match item {
            Item::Fn(func) => println!("Function: {}", func.sig.ident),
            Item::Struct(s) => println!("Struct: {}", s.ident),
            _ => (),
        }
    }
}
#[tokio::main]
async fn main(){ 
    dotenv().ok();
    let test: String = "How do you do?".to_string();
    let test1 = GoogleGemini::connect(test).await;
    let test2 = match test1 {
        Ok(t) => println!("{t:?}"),
        Err(error) => panic!("{error:?}"),
        
    };
    println!("{test2:?}");
 
}



