use rustemo::Parser;
use crate::tml::TmlParser;

mod tml;
mod tml_actions;

fn main() {
    let snippet = r#"
    fn test():
        if any(true):
            return
        end

        real a = 5
        real b = 6.2
        bool x = false
    end
    "#;

    let snippet2 = r#"
        real a = 5
        real b = 6.2
        int d = 3
        g = -2
        mam = "balsa"
        ss = 0b111001
        hex_try = 0x99AAFA
        bool b = false
    "#;

    let snippet3 = r#"
    # Global variables: tensor<real, 10> buffer, int ptr

    fn init_fnc():
        ptr = 0
        buffer = 0.0  # Broadcast 0.0 to all elements
    end

    fn output_fnc():
        buffer[ptr] = t.u  # Store new input sample

        real sum_val = 0.0
        for i = 0:10:
            sum_val = sum_val + buffer[i]
        end

        t.y = sum_val / 10.0  # Output the average

        # Update circular pointer
        ptr = ptr + 1
        if ptr >= 10:
            ptr = 0
        end
    end
    "#;

    let snippet4 = r#"
        fn test():
            for i = 0:10:
                sum_val = sum_val + buffer[i]
            end
        end
    "#;

    let snippet_t = r#"
    fn output_fnc():
        # Lokalna varijabla 'calc' ne koristi t.
        calc = t.u * t.gain

        # Pristup izlazu i parametrima preko t.
        if calc > t.max_val:
            t.y = t.max_val
        else:
            t.y = calc
        end
    end
    "#;

    let parser = TmlParser::new();
    println!("{:#?}", parser.parse(snippet_t));
}