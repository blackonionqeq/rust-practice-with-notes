trait Parser {
  type Output;
  // 注意这个Self::Output
  fn parse(&self, input: &str) ->Self::Output;
}

struct NumberParse;

struct WordCountParse;

impl Parser for NumberParse {
    type Output = Option<i32>;
    fn parse(&self, input: &str) -> Self::Output {
      // 根据parse的声明，返回的是Output也就是Option<i32>所以还需Ok
        let res = input.parse::<i32>();
        res.ok()
    }
}

impl Parser for WordCountParse {
  type Output = usize;
    fn parse(&self, input: &str) -> Self::Output {
        input.split_whitespace().count()
    }
}

fn main() {
  // 注意空结构体的新建不用加{}
    let number_parser = NumberParse;
    if let Some(num) = number_parser.parse("42") {
        println!("{}", num);
    }
    if let Some(num) = number_parser.parse("not a number") {
        println!("{}", num);
    } else {
        println!("not a number");
    }
    
    let word_count_parser = WordCountParse;
    println!("{}", word_count_parser.parse("rust is fun"));
}
