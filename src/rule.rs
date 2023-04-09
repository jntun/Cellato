use crate::cell::State;

pub type Config<'a> = (&'a State, &'a State, &'a State);
#[derive(Debug)]
pub struct WolframRule(bool, bool, bool, bool, bool, bool, bool, bool);

impl From<WolframRule> for u8 {
    fn from(rule: WolframRule) -> Self {
        let mut binary_rule: u8 = 0b00000000;
        if rule.0 { binary_rule = binary_rule >> 7 & 1 }
        if rule.1 { binary_rule = binary_rule >> 6 & 1 }
        if rule.2 { binary_rule = binary_rule >> 5 & 1 }
        if rule.3 { binary_rule = binary_rule >> 4 & 1 }
        if rule.4 { binary_rule = binary_rule >> 3 & 1 }
        if rule.5 { binary_rule = binary_rule >> 2 & 1 }
        if rule.6 { binary_rule = binary_rule >> 1 & 1 }
        if rule.7 { binary_rule = binary_rule >> 0 & 1 }
        binary_rule
    }
}

pub fn slow_wolfram(rule: WolframRule, input_cfg: Config) -> State {
    return match input_cfg {
        (State::ON, State::ON, State::ON) => { if rule.0 { State::ON} else { State::OFF }},     // 1
        (State::ON, State::ON, State::OFF) => { if rule.1 { State::ON} else { State::OFF }},    // 2
        (State::ON, State::OFF, State::ON) => { if rule.2 { State::ON} else { State::OFF }},    // 3
        (State::ON, State::OFF, State::OFF) => { if rule.3 { State::ON} else { State::OFF }},   // 4
        (State::OFF, State::ON, State::ON) => { if rule.4 { State::ON} else { State::OFF }},   // 5
        (State::OFF, State::ON, State::OFF) => { if rule.5 { State::ON} else { State::OFF }},   // 6
        (State::OFF, State::OFF, State::ON) => { if rule.6 { State::ON} else { State::OFF }},   // 7
        (State::OFF, State::OFF, State::OFF) => { if rule.7 { State::ON} else { State::OFF }},   // 8
    }
}

pub fn wolfram(rule: u8, input_cfg: Config) -> State {
    return match input_cfg {
        (State::ON, State::ON, State::ON) => { if (rule >> 7 & 1) == 1 { State::ON} else { State::OFF }},     // 1
        (State::ON, State::ON, State::OFF) => { if (rule >> 6 & 1) == 1 { State::ON} else { State::OFF }},    // 2
        (State::ON, State::OFF, State::ON) => { if (rule >> 5 & 1) == 1 { State::ON} else { State::OFF }},    // 3
        (State::ON, State::OFF, State::OFF) => { if (rule >> 4 & 1) == 1{ State::ON} else { State::OFF }},   // 4
        (State::OFF, State::ON, State::ON) => { if (rule >> 3 & 1) == 1 { State::ON} else { State::OFF }},   // 5
        (State::OFF, State::ON, State::OFF) => { if (rule >> 2 & 1) == 1 { State::ON} else { State::OFF }},   // 6
        (State::OFF, State::OFF, State::ON) => { if (rule >> 1 & 1) == 1 { State::ON} else { State::OFF }},   // 7
        (State::OFF, State::OFF, State::OFF) => { if (rule >> 0 & 1) == 1 { State::ON} else { State::OFF }},   // 8
    }
}