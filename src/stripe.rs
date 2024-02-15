use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Promise, Reflect};

#[wasm_bindgen]
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StripeOptions {
    mode: String,
    amount: u32,
    currency: String,
    payment_method_creation: String,
    layout: Layout,
    appearance: Appearance,
}

impl StripeOptions {
    fn new(
        mode: String,
        amount: u32,
        currency: String,
        payment_method_creation: String,
        layout: Layout,
        appearance: Appearance,
    ) -> Self {
        Self {
            mode,
            amount,
            currency,
            payment_method_creation,
            layout,
            appearance,
        }
    }

    fn to_jsvalue(&self) -> JsValue {
        JsValue::from_serde(self).unwrap()
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Layout {
    #[serde(rename = "type")]
    typ: String,
    default_collapsed: bool,
    radios: bool,
    spaced_accordion_items: bool,
}

impl Layout {
    fn new(
        typ: String,
        default_collapsed: bool,
        radios: bool,
        spaced_accordion_items: bool,
    ) -> Self {
        Self {
            typ,
            default_collapsed,
            radios,
            spaced_accordion_items,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Appearance {
    theme: String,
}

impl Appearance {
    fn new(theme: String) -> Self {
        Self { theme }
    }
}

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct StripeError {
    message: String,
}

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

    #[wasm_bindgen(constructor, catch)]
    fn new_stripe(value: JsValue) -> Result<Stripe, JsValue>;

    #[wasm_bindgen(method, catch)]
    fn elements(this: &Stripe, options: JsValue) -> Result<Elements, JsValue>;

    #[wasm_bindgen(method, catch, js_name = createPaymentMethod)]
    fn create_payment_method(this: &Stripe, elements: JsValue) -> Result<Promise, JsValue>;

    #[wasm_bindgen(method, catch)]
    fn create(this: &Elements, payment: JsValue) -> Result<PaymentElement, JsValue>;

    #[wasm_bindgen(method, catch)]
    fn submit(this: &Elements) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    fn mount(this: &PaymentElement, dom_element: JsValue) -> Result<(), JsValue>;

}

pub fn mount_payment_element(price: u32) -> (Elements, Stripe) {
    dotenv::dotenv().ok();
    let stripe_public_key = std::env::var("STRIPE_PUBLIC_KEY").unwrap();
    tracing::debug!("loading: mount_payment_element");
    let stripe = Stripe::new_stripe(JsValue::from_str(&stripe_public_key)).unwrap();

    let layout = Layout::new("accordion".into(), false, false, true);

    let appearance = Appearance::new("flat".into());

    let options = StripeOptions::new(
        "payment".to_string(),
        price,
        "aud".into(),
        "manual".into(),
        layout,
        appearance,
    );

    let elements = stripe.elements(options.to_jsvalue()).unwrap();

    let payment_elements = elements.create(JsValue::from_str("payment")).unwrap();

    payment_elements
        .mount(JsValue::from_str("#payment-element"))
        .unwrap();

    (elements, stripe)
}

pub async fn element_submission(
    elements: Elements,
    error_div: web_sys::HtmlElement,
    stripe: Stripe,
) -> Result<PaymentMethod, JsValue> {
    if let Err(e) = elements.submit() {
        error_div.set_text_content(Some(&e.into_serde::<StripeError>().unwrap().message));
    }

    let payment_obj = web_sys::js_sys::Object::new();
    Reflect::set(&payment_obj, &"elements".into(), &elements.obj).unwrap();

    let payment = stripe.create_payment_method(payment_obj.into());

    if let Err(e) = payment {
        error_div.set_text_content(Some(&e.into_serde::<StripeError>().unwrap().message));
        Err(e)
    } else {
        Ok(JsFuture::from(payment.unwrap())
            .await
            .unwrap()
            .unchecked_ref::<PaymentMethod>()
            .clone())
    }
}

pub fn payment_method_to_string(
    payment_method: PaymentMethod,
) -> Result<String, serde_json::Error> {

    tracing::info!("method {:?}", payment_method.obj);

    web_sys::js_sys::JSON::stringify(&payment_method.obj)
        .unwrap()
        .into_serde::<String>()
}
