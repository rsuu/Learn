fn main() {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_function_macro() {
        extern crate function_macro;
        use function_macro::make_answer;

        make_answer!();

        assert_eq!(42, answer());
    }

    #[test]
    fn test_derive_macro() {
        extern crate derive_macro;
        use derive_macro::{AnswerFn, HelperAttr};

        #[derive(AnswerFn)]
        struct A;

        #[derive(HelperAttr)]
        struct B {
            #[helper]
            field: (),
        }

        assert_eq!(42, answer());
    }

    #[test]
    fn test_attribute_macro() {
        extern crate attribute_macro;

        use attribute_macro::show_streams;

        // Example: Basic function
        #[show_streams]
        fn invoke1() {}
        // out: attr: ""
        // out: item: "fn invoke1() { }"

        // Example: Attribute with input
        #[show_streams(bar)]
        fn invoke2() {}
        // out: attr: "bar"
        // out: item: "fn invoke2() {}"

        // Example: Multiple tokens in the input
        #[show_streams(multiple => tokens)]
        fn invoke3() {}
        // out: attr: "multiple => tokens"
        // out: item: "fn invoke3() {}"

        // Example:
        #[show_streams { delimiters }]
        fn invoke4() {}
        // out: attr: "delimiters"
        // out: item: "fn invoke4() {}"

        /* output:
        attr: "delimiters"
        item: "fn invoke4() { }"
        attr: "multiple => tokens"
        item: "fn invoke3() { }"
        attr: "bar"
        item: "fn invoke2() { }"
        attr: ""
        item: "fn invoke1() { }"
        */
    }
}
