// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

struct ProductName(String);

impl ProductName {
    fn new(name: String) -> Self {
        if name.is_empty() || name.len() > 300 {
            panic!("Invalid product name");
        }
        ProductName(name)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

struct Quantity(u32);

impl Quantity {
    fn new(quantity: u32) -> Self {
        if quantity == 0 {
            panic!("Quantity must be greater than zero");
        }
        Quantity(quantity)
    }

    fn value(&self) -> &u32 {
        &self.0
    }
}

struct UnitPrice(u32);

impl UnitPrice {
    fn new(price: u32) -> Self {
        if price == 0 {
            panic!("Unit price must be greater than zero");
        }
        UnitPrice(price)
    }

    fn value(&self) -> &u32 {
        &self.0
    }
}

pub struct Order {
    product_name: ProductName,
    quantity: Quantity,
    unit_price: UnitPrice,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Order {
            product_name: ProductName::new(product_name),
            quantity: Quantity::new(quantity),
            unit_price: UnitPrice::new(unit_price),
        }
    }

    pub fn product_name(&self) -> &String {
        self.product_name.value()
    }

    pub fn set_product_name(&mut self, product_name: String) {
        self.product_name = ProductName::new(product_name);
    }

    pub fn quantity(&self) -> &u32 {
        self.quantity.value()
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = Quantity::new(quantity);
    }

    pub fn unit_price(&self) -> &u32 {
        self.unit_price.value()
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        self.unit_price = UnitPrice::new(unit_price);
    }

    pub fn total(&self) -> u32 {
        self.quantity.value() * self.unit_price.value()
    }
}
