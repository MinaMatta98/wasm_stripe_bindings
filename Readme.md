# Stripe Wasm Bindings
This repository serves as an example of converting several stripe functions into wasm bindgen functions that can be directly used within a RUST wasm32_unknown_unknown environment.

The required target must be installed:

```bash
rustup target add wasm32_unknown_unknown
```

Example functions that have been overwritten:

|Wasm Function  | JS Function  | Address |
|---|---|---|
|`new_stripe()`|`Stripe()`|https://docs.stripe.com/js/initializing|
|`create_payment_method()`|`CreatePaymentMethod()`| https://docs.stripe.com/js/payment_methods/create_payment_method|
|`create()`|`elements.create()`|https://docs.stripe.com/js/elements_object/create_payment_element|
|`submit()`|`elements.submit()`|https://docs.stripe.com/js/elements/submit|
|`mount()`|`elements.submit()`|https://docs.stripe.com/js/element/mount|

```rs
// Note that all options are passed as JsValue, as this is resolved via
// JsValue::from_serde(T).unwrap() where T is the option param
#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Stripe;

    #[derive(Debug, Clone)]
    pub type Elements;

    #[derive(Debug)]
    pub type PaymentElement;

    #[derive(Debug, Clone)]
    pub type PaymentMethod;

    // constructor attribute decorator => Stripe()
    #[wasm_bindgen(constructor, catch)]
    fn new_stripe(value: JsValue) -> Result<Stripe, JsValue>;

    // method attribute decorator: elements => stripe.elements()
    #[wasm_bindgen(method, catch)]
    fn elements(this: &Stripe, options: JsValue) -> Result<Elements, JsValue>;

    // Snake case to camelCase: create_payment_method => CreatePaymentMethod via js_name attribute
    #[wasm_bindgen(method, catch, js_name = createPaymentMethod)]
    fn create_payment_method(this: &Stripe, elements: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    fn create(this: &Elements, payment: JsValue) -> Result<PaymentElement, JsValue>;

    #[wasm_bindgen(method, catch)]
    fn submit(this: &Elements) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    fn mount(this: &PaymentElement, dom_element: JsValue) -> Result<(), JsValue>;
}
```
