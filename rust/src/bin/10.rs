
use std::io;
use std::collections::HashMap;

#[derive(Clone,Copy,Debug)]
enum OpType {
    Bot,
    Output,
}

#[derive(Clone,Copy,Debug)]
struct Operation {
    typ: OpType,
    recipient: i32,
    value: Option<i32>,
}

#[derive(Debug)]
struct Bot {
    id: i32,
    low: Option<i32>,
    high: Option<i32>,
    low_op: Option<Operation>,
    high_op: Option<Operation>,
}

impl Bot {
    fn new(v: i32) -> Bot {
        Bot {
            id: v,
            low: None,
            high: None,
            low_op: None,
            high_op: None,
        }
    }

    fn ready(&self) -> bool {
        match (self.low, self.low_op, self.high, self.high_op) {
            (None, _, _, _) => false,
            (_, None, _, _) => false,
            (_, _, None, _) => false,
            (_, _, _, None) => false,
            (_, _, _, _) => true,
        }
    }

    fn set_low(&mut self, op: Operation) -> bool {
        self.low_op = Some(op);
        self.ready()
    }

    fn set_high(&mut self, op: Operation) -> bool {
        self.high_op = Some(op);
        self.ready()
    }

    fn give_value(&mut self, v: i32) -> bool {
        match (self.low, self.high) {
            (Some(_), Some(_)) => {
                panic!("Bot has two values already");
            }
            (Some(i), None) if v >= i => {
                self.high = Some(v);
            }
            (Some(i), None) if v < i => {
                self.high = self.low;
                self.low = Some(v);
            }
            (None, Some(i)) if v > i => {
                self.low = self.high;
                self.high = Some(v);
            }
            (None, Some(i)) if v <= i => {
                self.low = Some(v);
            }
            (None, None) => {
                self.high = Some(v);
            }
            (_, _) => panic!(""),
        }

        println!("[{}] low: {:?}, high: {:?}", self.id, self.low, self.high);

        self.ready()
    }

    fn take_low(&mut self) -> Operation {
        if self.low == None {
            panic!("Bot isn't prepared for take_low");
        }
        let mut op = self.low_op.unwrap();
        op.value = self.low;
        self.low_op = None;
        self.low = None;
        op
    }

    fn take_high(&mut self) -> Operation {
        if self.high == None {
            panic!("Bot isn't prepared for take_high");
        }
        let mut op = self.high_op.unwrap();
        op.value = self.high;
        self.high_op = None;
        self.high = None;
        op
    }
}

fn main() {
    let mut bots = HashMap::new();
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.len() == 0 {
                    break;
                }
                let input = {
                    let s = input.to_string();
                    input.clear();
                    s
                };

                let mut ops = Vec::new();

                let mut row = input.trim().split_whitespace();
                let cmd = row.next();
                let value: i32 = row.next().unwrap().parse().unwrap();
                match cmd {
                    Some("bot") => {
                        let bot = bots.entry(value).or_insert(Bot::new(value));
                        let first = row.nth(3).unwrap();
                        let first_value: i32 = row.next().unwrap().parse().unwrap();
                        match first {
                            "bot" => {
                                if bot.set_low(Operation {
                                    typ: OpType::Bot,
                                    recipient: first_value,
                                    value: None,
                                }) {
                                    ops.push(bot.take_low());
                                    ops.push(bot.take_high());
                                }
                            }
                            "output" => {
                                if bot.set_low(Operation {
                                    typ: OpType::Output,
                                    recipient: first_value,
                                    value: None,
                                }) {
                                    ops.push(bot.take_low());
                                    ops.push(bot.take_high());
                                }
                            }
                            &_ => panic!(""),
                        }

                        let second = row.nth(3).unwrap();
                        let second_value: i32 = row.next().unwrap().parse().unwrap();
                        match second {
                            "bot" => {
                                if bot.set_high(Operation {
                                    typ: OpType::Bot,
                                    recipient: second_value,
                                    value: None,
                                }) {
                                    ops.push(bot.take_low());
                                    ops.push(bot.take_high());
                                }
                            }
                            "output" => {
                                if bot.set_high(Operation {
                                    typ: OpType::Output,
                                    recipient: second_value,
                                    value: None,
                                }) {
                                    ops.push(bot.take_low());
                                    ops.push(bot.take_high());
                                }
                            }
                            &_ => panic!(""),
                        }
                    }
                    Some("value") => {
                        let id: i32 = row.nth(3).unwrap().parse().unwrap();


                        {
                            let mut bot = bots.entry(id).or_insert(Bot::new(id));
                            if bot.give_value(value) {
                                ops.push(bot.take_low());
                                ops.push(bot.take_high());
                            }
                        }


                    }
                    _ => panic!(""),
                }

                loop {
                    let mut new_ops = Vec::new();
                    if ops.len() == 0 {
                        break;
                    }

                    for op in ops {
                        match op.typ {
                            OpType::Output => {
                                println!("{:?} -> output {}", op.value, op.recipient);
                                // Do nothing
                            }
                            OpType::Bot => {
                                let bot = bots.entry(op.recipient)
                                              .or_insert(Bot::new(op.recipient));
                                println!("{:?} -> bot {}", op.value, bot.id);
                                if bot.give_value(op.value.unwrap()) {
                                    new_ops.push(bot.take_low());
                                    new_ops.push(bot.take_high());
                                }
                            }
                        }
                    }

                    ops = new_ops;
                }
            }
            _ => panic!(""),
        }
    }
}
