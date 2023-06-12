# emoscript
The emoscript is a simple stack-based programming language designed for fun and education. It aims to provide an engaging and visually appealing way to learn the basics of programming, especially for those who might be intimidated by traditional programming languages. The project uses emojis to represent various programming constructs and operations, making it more accessible and enjoyable.

## Reason
The emoscript was created to explore the potential of using emojis in programming languages and to make learning to code more engaging and fun. The idea was to experiment with a different approach to writing and reading code, making it more visually appealing and interactive.

## Future of emoscript
While it's unlikely that the emoscript will replace traditional programming languages, it serves as a unique and enjoyable way to learn programming concepts. It can be used as a stepping stone for beginners or as a way to experiment with language design. The project can be extended with new emojis and additional features, allowing for continued growth and improvement.

## Technical Overview
The emoscript is implemented in Rust and uses a stack-based execution model. It consists of the following components:

* Interpreter: The main structure containing the execution environment, including variables and the evaluation stack.

* Token: An enum representing the different types of tokens, such as numbers, operations, and variables.

* tokenize: A function to convert the input code into a list of tokens.

* perform_operation: A function to perform arithmetic operations based on the provided operator and operands.

* run: A function to execute the provided code.

## How to use
1. Clone the repository.

2. Install Rust, if not already installed.

3. Modify the input variable in the main function to represent the desired expression using emojis. 
For example, "2️⃣🗨.🗨0️⃣🗨🤔2️⃣"; represents "2.0 / 2".

4. Run the code with cargo run.

## Emoji Mapping
| **Emoji** | **Operation**          | **Example**                                         |
|-----------|------------------------|-----------------------------------------------------|
| 🙂        |     + (Addition)       |     12🙂34 (12 + 34)                               |
| 😢        |     - (Subtraction)    |     12😢34 (12 - 34)                               |
| 😂        |     * (Multiplication) |     12😂34 (12 * 34)                               |
| 🤔        |     / (Division)       |     12🤔34 (12 / 34)                               |
| 😕😄😞   | if-then-else           | 😕😄🙂1️⃣😞🙂2️⃣ (if <expression> then +1 else +2)  |
| 🔁🔚     | loop                   | 9️⃣🔁🔚 (🔁 - begin loop, 🔚 - end loop)            |
| [num1🔁num2]🔚     | range-loop   | [0🔁8]🔚                                             |
| 🏁func_name expr🚩| function     | 🏁f2️⃣😂2️⃣🚩                                         |
| 📞func_name|     call function     |     📞f                                             |

## Example
```rust
fn main() {
    let input = "2️⃣🗨.🗨0️⃣🗨🤔2️⃣";
    let mut peekable_chars = PeekableChars::new(input);
    let tokens = peekable_chars.tokenize();

    let mut interpreter = EmojiInterpreter::new(tokens);
    let result = interpreter.evaluate();
    println!("Result: {}", result);
    // if then else
    let input = "2️⃣🗨.🗨5️⃣🗨🤔😄🙂1️⃣😞🙂2️⃣"
}
```

## How to run

Playground is here -> 

## Designing an Emoji-based Programming Language: A Step-by-Step Guide with Rust
In this tutorial, we'll walk you through the process of designing and implementing an emoji-based programming language using Rust. By the end, you'll have a better understanding of language design principles and will be equipped with the knowledge needed to create your own unique programming language.

### Step 1: Define the language structure and syntax
Choose the basic structure and syntax of your language. For this example, we'll use emojis to represent different language constructs like numbers, operations, and control flow. Here's an overview of the syntax:

* Numbers: Use emojis like '2️⃣' and '🗨' to represent digits and decimal points.

* Operations: Use emojis like '🙂', '😢', '😂', and '🤔' to represent basic arithmetic operations (addition, subtraction, multiplication, and division).

* Control Flow: Use emojis like '😕', '😄', '😞', and '😢' to represent control flow constructs like if, then, else, and endif.

* Variables: Use the '🔤' emoji followed by a number to represent a variable assignment.

### Step 2: Create the tokenizer
Write a tokenizer (lexer) to transform the input code into a stream of tokens. The tokenizer should recognize the various emojis representing numbers, operations, and control flow constructs and emit corresponding tokens for each.

Create a PeekableChars struct that will be used to iterate through the input string and peek at the next character. Implement the Tokenize trait for PeekableChars with a tokenize() method that processes the input string and generates a vector of EmojiToken enums.

### Step 3: Implement the interpreter
Create an EmojiInterpreter struct to process the stream of tokens generated by the tokenizer. The struct should have a new() method to initialize the interpreter with the tokens, as well as an evaluate() method to process the tokens and perform the specified operations.

Inside the evaluate() method, use a loop to iterate through the tokens and process them according to their type. The processing should include arithmetic operations, control flow constructs, and variable assignment. Handle any errors during execution gracefully.

### Step 4: Test your language
Test your language using various code snippets to ensure it works correctly. Include edge cases and error conditions to verify the robustness of your implementation. For example:

```rust
let input = "2️⃣🗨.🗨0️⃣🗨🤔2️⃣";
let mut peekable_chars = PeekableChars::new(input);
let tokens = peekable_chars.tokenize();

let mut interpreter = EmojiInterpreter::new(tokens);
let result = interpreter.evaluate();
println!("Result: {}", result);

let input2 = "2️⃣🗨.🗨5️⃣🗨😕😄🙂1️⃣😞🙂2️⃣";
let mut peekable_chars2 = PeekableChars::new(input2);
let tokens2 = peekable_chars2.tokenize();

let mut interpreter2 = EmojiInterpreter::new(tokens2);
let result2 = interpreter2.evaluate();
println!("Result: {}", result2);

let input3 = "🔤2️⃣🙂2️⃣";
let mut peekable_chars3 = PeekableChars::new(input3);
let tokens3 = peekable_chars3.tokenize();

let mut interpreter3 = EmojiInterpreter::new(tokens3);
let result3 = interpreter3.evaluate();
println!("Result: {}", result3);
```

### Step 5: Optimize and expand your language
Once you have verified that your language implementation works correctly, consider ways to optimize the performance, improve error handling, and expand the language with additional features. Some suggestions include:

* Implement more advanced error handling and reporting to provide helpful feedback when users encounter issues with their code.

* Add support for more advanced data structures, such as lists, maps, and objects.

* Implement functions or procedures to allow users to create reusable pieces of code.

* Add support for loops and other advanced control flow constructs.

* Integrate external libraries to provide additional functionality.

### Step 6: Document your language
Write clear and comprehensive documentation for your language, detailing its syntax, features, and usage. Include examples and tutorials to help users get started and to demonstrate the various features of your language. Make sure to cover any error messages or warnings that users might encounter, as well as any best practices for writing efficient and maintainable code in your language.

### Step 7: Share your language with the community
Once you're satisfied with your language implementation and documentation, share your work with the community. This could include publishing the source code on a platform like GitHub, creating a website with documentation and examples, and promoting your language on social media, forums, or blog posts. Engage with users, gather feedback, and continually refine and expand your language based on their suggestions and needs.

### Conclusion
Designing and implementing your own programming language can be a rewarding and educational experience. By following this step-by-step guide, you'll gain valuable insight into language design principles and learn how to create a unique and functional programming language using Rust. As you continue to refine and expand your language, you'll have the opportunity to contribute to the broader programming community and create a useful tool for others to use and enjoy.

## Architecture perspective (designing)
soon

## Technical solution
soon