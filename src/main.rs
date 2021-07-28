// The Translator for "Apu-bot". 
// Writen by n1ubi.
//
// Example:
//  Miten printtaan muuttujan C?
//             vvvv
//  How do I print a variable in C
//
// To dos marked with block comment, notes/explanations marked with //
//

use std::env; // For "command-line" arguments.


pub fn parse(input: String) {
    let mut output = String::from("");

    // Hopefully you can read the code.
    //let con = "input.contains"; // I tried this, but didn't work.
    if input != "" {
        // The actual translating part.
        if input.contains("miten") || input.contains("kuinka")              {output.push_str("how do I ");}
        if input.contains("print")                                          {output.push_str("print ");}
        if input.contains("muuttuja") || input.contains("var")              // Too long. 
        || input.contains("variable")                                       {output.push_str("variable");}
        if input.contains("string") || input.contains("teksti")             {output.push_str("string");}
        //if input.contains("") || input.contains("")              {output.push_str("");} 
        if input.contains("jos") || input.contains("if")                    {output.push_str("if");}
        if input.contains("tai") || input.contains("else")                  {output.push_str("else");}
        /* TO DO: if input contains unknown word, output.push_str(unknownword) */
        // If contains language name, add it to the output. (To make the search result more accurate.)
        if input.contains("js") || input.contains("javascript")             {output.push_str(" JavaScript");}
        if input.contains("py") || input.contains("python")                 {output.push_str(" Python");}
        if input.contains("c")                                              {output.push_str(" C");}
        if input.contains("cpp") || input.contains("C++")                   {output.push_str(" C++");}
        if input.contains("c-sharp") || input.contains("c#")                {output.push_str(" C#");}
        if input.contains("java") || input.contains("jar")                  {output.push_str(" Java");}
        if input.contains("rs") || input.contains("rust")                   {output.push_str(" Rust");}
        if input.contains("ps") || input.contains("powershell")             {output.push_str(" Powershell");}
        if input.contains("sh") || input.contains("bash")                   {output.push_str(" Bash");}
        if input.contains("bat") || input.contains("batch")                 {output.push_str(" Batch");}
        if input.contains("kotlin")                                         {output.push_str(" Kotlin");}
        if input.contains("swift")                                          {output.push_str(" Swift");}
        if input.contains("html")                                           {output.push_str(" HTML");}
        if input.contains("css")                                            {output.push_str(" CSS");}
        if input.contains("perl")                                           {output.push_str(" Perl");}
        if input.contains("haskell")                                        {output.push_str(" Haskell");}
        if input.contains("sql")                                            {output.push_str(" SQL");}
        if input.contains("ts") || input.contains("typescript")             {output.push_str(" TypeScript");}
        if input.contains("lua") || input.contains("roblox script")         {output.push_str(" Lua");}
        if input.contains("php")                                            {output.push_str(" PHP");}
        if input.contains("objectivec") || input.contains("objc")           {output.push_str(" Obj-C");}
        if input.contains("assembly") || input.contains("asm")              {output.push_str(" Assembly");}
        if input.contains("autoit")                                         {output.push_str(" Autoit");}
        if input.contains("coffeescript")                                   {output.push_str(" CoffeeScript");}
        if input.contains("vbs") || input.contains("visual basic")          {output.push_str(" VBS");}
        if input.contains("dart")                                           {output.push_str(" Dart");}
        if input.contains("xml")                                            {output.push_str(" XML");}
        if input.contains("pascal")                                         {output.push_str(" Pascal");}
        if input.contains("ruby")                                           {output.push_str(" Ruby");}

        // Output => link   I can ignore all this vvvv       output vvvvvvv
        // Replace " " with "+". ( https://www.google.com/search?q=google+search )
        // + = %2B, / = %2F
        output = output.replace("+", "%2B").replace("/", "%2F").replace(" ", "+");


        println!("{}",output); // Replace the println with return.
    }
    else {
        /* return an error or something. */
        print!("An error occured.");
    }    
}

pub fn main()
{
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    // Program assumes it's going to get only one argument.

    let input = arg.to_string().replace("-", " ").to_lowercase();

    parse(input);
}
